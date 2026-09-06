use super::cargo::CargoWorkspace;
use ament_rs::{AMENT_PREFIX_PATH_ENV_VAR, search_paths::get_search_paths};
use cargo_toml::Manifest;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

pub const METADATA_TABLE: &str = "ros-env";
pub const METADATA_KEY: &str = "interfaces";

/// Generated interface package name -> path to its installed Cargo manifest.
type InterfaceManifests = BTreeMap<String, PathBuf>;

/// Generated interface package name -> other interface packages it imports.
type InterfaceDependencies = BTreeMap<String, Vec<String>>;

/// Interface requests collected from the consuming Cargo dependency graph.
pub(crate) struct InterfaceRequests {
    /// Requested interface package -> names of the crates that asked for it.
    by_interface: BTreeMap<String, BTreeSet<String>>,
    /// Mutable manifests that Cargo must watch for request changes.
    local_manifests: Vec<PathBuf>,
}

impl InterfaceRequests {
    /// Return whether the dependency graph requested no ROS interfaces.
    pub(crate) fn is_empty(&self) -> bool {
        self.by_interface.is_empty()
    }

    /// Return local manifests that can change without changing the root manifest.
    pub(crate) fn local_manifests(&self) -> &[PathBuf] {
        &self.local_manifests
    }
}

impl CargoWorkspace {
    /// Collect the union of interface requests from the consuming Cargo graph.
    ///
    /// Every crate may declare `[package.metadata.ros-env] interfaces = [...]`. Those
    /// lists are merged so a library only names what it uses, and an application that
    /// depends on several such libraries still gets the full set.
    ///
    /// A virtual workspace has no root package, so it can declare the same key on
    /// `[workspace.metadata.ros-env]` instead.
    ///
    /// Packages with no `source` live on disk (workspace members and path deps). Their
    /// manifests are recorded so Cargo rebuilds this script when a request changes.
    /// Registry crates are immutable and do not need that tracking.
    pub(crate) fn interface_requests(&self) -> Result<InterfaceRequests, String> {
        let metadata = self.metadata()?;

        let mut requests = InterfaceRequests {
            by_interface: BTreeMap::new(),
            local_manifests: Vec::new(),
        };

        // A missing table is fine: it just means the workspace itself did not
        // request any interfaces.
        let workspace_metadata = metadata.get("metadata");

        let declaration = format!("`[workspace.metadata.{METADATA_TABLE}] {METADATA_KEY}`");
        requests.add(workspace_metadata, &declaration, "the workspace".to_owned())?;

        let packages = metadata
            .get("packages")
            .and_then(Value::as_array)
            .ok_or("`cargo metadata` output has no `packages` array")?;

        for package in packages {
            let name = package
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or("<unnamed>");

            // `source` is set for crates.io (and other registries). Path and workspace
            // members leave it empty. Watch their Cargo.toml so a change to
            // `[package.metadata.ros-env] interfaces` reruns this script. Their other
            // sources do not affect the generated module tree.
            if package.get("source").is_none_or(Value::is_null) {
                if let Some(manifest) = package.get("manifest_path").and_then(Value::as_str) {
                    requests.local_manifests.push(PathBuf::from(manifest));
                }
            }

            let declaration =
                format!("`[package.metadata.{METADATA_TABLE}] {METADATA_KEY}` in `{name}`");
            requests.add(package.get("metadata"), &declaration, name.to_owned())?;
        }

        Ok(requests)
    }
}

impl InterfaceRequests {
    /// Merge one crate or workspace's `interfaces` list into the union.
    ///
    /// `metadata` is the JSON object for `[package.metadata]` or
    /// `[workspace.metadata]`. A missing `ros-env.interfaces` key is ignored
    /// (that crate requested nothing). A present key that is not an array of
    /// strings fails, and `declaration` is used in the error so the bad
    /// Cargo.toml can be named.
    ///
    /// Each requested package is mapped to the crates that asked for it. Later,
    /// if a package is missing from `AMENT_PREFIX_PATH`, those names appear in
    /// the diagnostic.
    fn add(
        &mut self,
        metadata: Option<&Value>,
        declaration: &str,
        requester: String,
    ) -> Result<(), String> {
        for interface in interfaces_from_metadata(metadata, declaration)? {
            self.by_interface
                .entry(interface)
                .or_default()
                .insert(requester.clone());
        }
        Ok(())
    }
}

/// Installed generated interfaces after resolving overlay precedence.
pub(crate) struct InterfaceRegistry {
    manifests: InterfaceManifests,
}

impl InterfaceRegistry {
    /// Discover generated interface crates on `AMENT_PREFIX_PATH`.
    ///
    /// The search path lists overlays before underlays, so the first prefix that
    /// provides a package wins and later prefixes cannot shadow it.
    pub(crate) fn discover() -> Self {
        let mut manifests = InterfaceManifests::new();

        for prefix in get_search_paths().unwrap_or_default() {
            for cargo_toml in generated_manifests_in_prefix(&PathBuf::from(prefix)) {
                let Some(rust_dir) = cargo_toml.parent() else {
                    continue;
                };

                manifests
                    .entry(package_name_from_rust_dir(rust_dir).to_owned())
                    .or_insert(cargo_toml);
            }
        }

        Self { manifests }
    }

    /// Return whether no generated interface packages were discovered.
    pub(crate) fn is_empty(&self) -> bool {
        self.manifests.is_empty()
    }

    /// Resolve requested interfaces and all generated dependencies they require.
    pub(crate) fn resolve(
        &self,
        requests: &InterfaceRequests,
    ) -> Result<InterfaceSelection, String> {
        let mut missing = missing_direct_requests(requests, &self.manifests);
        let included = self.expand_generated_dependencies(requests, &mut missing);

        fail_if_missing(missing)?;

        let manifests = included
            .iter()
            .filter_map(|package| self.manifests.get(package).cloned())
            .collect();

        Ok(InterfaceSelection { manifests })
    }

    /// Walk `*` dependencies of requested packages and add every reachable generated crate.
    ///
    /// Generated sources reference those packages by bare path (`builtin_interfaces::msg::Time`).
    /// We resolve that with `use crate::<dep>` inside each included module, so every `*`
    /// dependency must also be a sibling module. Missing ones are recorded rather than
    /// skipped: a partial graph would compile until the first unresolved `use`.
    fn expand_generated_dependencies(
        &self,
        requests: &InterfaceRequests,
        missing: &mut Vec<String>,
    ) -> BTreeSet<String> {
        let dependencies = self.load_dependencies();
        let mut included: BTreeSet<String> = requests.by_interface.keys().cloned().collect();
        let mut pending: VecDeque<String> = included.iter().cloned().collect();

        while let Some(package) = pending.pop_front() {
            let Some(package_dependencies) = dependencies.get(&package) else {
                continue;
            };

            for dependency in package_dependencies {
                if !self.manifests.contains_key(dependency) {
                    missing.push(format!(
                        "  - `{dependency}`, required by interface package `{package}`"
                    ));
                    continue;
                }

                if included.insert(dependency.clone()) {
                    pending.push_back(dependency.clone());
                }
            }
        }

        included
    }

    /// Load each generated package's generated interface dependencies.
    fn load_dependencies(&self) -> InterfaceDependencies {
        self.manifests
            .iter()
            .filter_map(|(package, cargo_toml)| {
                Manifest::from_path(cargo_toml)
                    .ok()
                    .map(|manifest| (package.to_owned(), interface_dependency_names(&manifest)))
            })
            .collect()
    }
}

/// Name requested packages that are not installed on `AMENT_PREFIX_PATH`.
fn missing_direct_requests(
    requests: &InterfaceRequests,
    available: &InterfaceManifests,
) -> Vec<String> {
    requests
        .by_interface
        .iter()
        .filter(|(interface, _)| !available.contains_key(*interface))
        .map(|(interface, requesters)| {
            let requesters: Vec<&str> = requesters.iter().map(String::as_str).collect();
            format!("  - `{interface}`, requested by {}", requesters.join(", "))
        })
        .collect()
}

/// Fail the build if any requested or transitively required package is missing.
fn fail_if_missing(mut missing: Vec<String>) -> Result<(), String> {
    if missing.is_empty() {
        return Ok(());
    }

    missing.sort();
    missing.dedup();
    Err(format!(
        "the following interface packages were not found on {AMENT_PREFIX_PATH_ENV_VAR}:\n{}\n\
         Make sure the packages are built and their workspace is sourced.",
        missing.join("\n")
    ))
}

/// A validated, transitively complete set of generated interface packages.
pub(crate) struct InterfaceSelection {
    pub(super) manifests: Vec<PathBuf>,
}

/// List the generated interface manifests installed under one ament prefix.
///
/// `rosidl_generator_rs` installs each package's crate at
/// `<prefix>/share/<package>/rust/Cargo.toml`. A prefix without a `share` directory, or
/// a package that ships no generated Rust, contributes nothing.
fn generated_manifests_in_prefix(prefix: &Path) -> Vec<PathBuf> {
    let Ok(packages) = fs::read_dir(prefix.join("share")) else {
        return Vec::new();
    };

    packages
        .filter_map(Result::ok)
        .map(|package| package.path().join("rust").join("Cargo.toml"))
        .filter(|cargo_toml| cargo_toml.is_file())
        .collect()
}

/// Read the ROS package name from an installed `<prefix>/share/<package>/rust` path.
pub(super) fn package_name_from_rust_dir(rust_dir: &Path) -> &str {
    rust_dir
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|name| name.to_str())
        .expect("AMENT package directory should be <prefix>/share/<package>/rust")
}

/// Return packages named as `*` dependencies in a generated crate's manifest.
pub(super) fn interface_dependency_names(manifest: &Manifest) -> Vec<String> {
    manifest
        .dependencies
        .iter()
        .filter(|(_, version)| version.req() == "*")
        .map(|(name, _)| name.to_owned())
        .collect()
}

/// Validate and return an interface list from one Cargo metadata declaration.
fn interfaces_from_metadata(
    metadata: Option<&Value>,
    declaration: &str,
) -> Result<Vec<String>, String> {
    let requested_interfaces = metadata
        .and_then(|metadata| metadata.get(METADATA_TABLE))
        .and_then(|table| table.get(METADATA_KEY));

    let Some(interfaces) = requested_interfaces else {
        return Ok(Vec::new());
    };

    interfaces
        .as_array()
        .ok_or_else(|| format!("{declaration} must be an array of interface package names"))?
        .iter()
        .map(|interface| {
            interface
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| format!("{declaration} must only contain package names"))
        })
        .collect()
}
