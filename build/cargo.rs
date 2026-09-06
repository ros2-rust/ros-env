use serde_json::Value;
use std::env;
use std::ffi::OsStr;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::process::Command;

pub(crate) const CARGO_WORKSPACE_DIR_ENV_VAR: &str = "CARGO_WORKSPACE_DIR";

/// Cargo build-script instructions used by interface discovery and code generation.
pub(crate) struct CargoBuildScript;

impl CargoBuildScript {
    /// Emit a warning through Cargo. Pass `format_args!(...)` for interpolation.
    pub(crate) fn warning(message: impl Display) {
        println!("cargo:warning={message}");
    }

    /// Ask Cargo to rerun the build script when `path` changes.
    pub(crate) fn rerun_if_path_changed(path: impl AsRef<Path>) {
        println!("cargo:rerun-if-changed={}", path.as_ref().display());
    }

    /// Ask Cargo to rerun the build script when an environment variable changes.
    pub(crate) fn rerun_if_environment_changed(variable: impl Display) {
        println!("cargo:rerun-if-env-changed={variable}");
    }

    /// Return a file path inside this build-script invocation's output directory.
    pub(crate) fn output_path(file_name: impl AsRef<Path>) -> PathBuf {
        PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR not set")).join(file_name)
    }
}

/// A consuming Cargo workspace with a validated root manifest.
pub(crate) struct CargoWorkspace {
    root: PathBuf,
}

impl CargoWorkspace {
    /// Locate the consuming workspace from an explicit override or Cargo's output path.
    ///
    /// Cargo does not expose the workspace root to dependency build scripts. The
    /// `CARGO_WORKSPACE_DIR` convention is preferred; otherwise this recognizes Cargo's
    /// old and new `OUT_DIR` layouts.
    pub(crate) fn locate() -> Result<Self, String> {
        let cargo_workspace_dir =
            env::var_os(CARGO_WORKSPACE_DIR_ENV_VAR).filter(|value| !value.is_empty());

        if let Some(value) = cargo_workspace_dir {
            let root = PathBuf::from(value);

            if !root.join("Cargo.toml").is_file() {
                return Err(format!(
                    "`{CARGO_WORKSPACE_DIR_ENV_VAR}` is set to `{}`, which does not contain a Cargo.toml",
                    root.display()
                ));
            }

            return Ok(Self { root });
        }

        workspace_root_from_out_dir()
            .map(|root| Self { root })
            .ok_or_else(|| {
                format!(
                    "could not determine the Cargo workspace root from OUT_DIR.\n\
                     Declare it once in the workspace's .cargo/config.toml so that a plain \
                     `cargo build` keeps working:\n\
                     \n    \
                     [env]\n    \
                     {CARGO_WORKSPACE_DIR_ENV_VAR} = {{ value = \"\", relative = true }}\n"
                )
            })
    }

    /// Return the validated workspace root.
    pub(crate) fn root(&self) -> &Path {
        &self.root
    }

    /// Query Cargo for the consuming workspace's target-specific dependency graph.
    pub(super) fn metadata(&self) -> Result<Value, String> {
        let manifest_path = self.root.join("Cargo.toml");

        let cargo_metadata = run_cargo_metadata(&manifest_path)?;
        serde_json::from_str(&cargo_metadata)
            .map_err(|err| format!("failed to parse `cargo metadata` output: {err}"))
    }

    /// Ask Cargo to rerun when the workspace graph or a local manifest changes.
    pub(crate) fn rerun_if_manifests_change(&self, local_manifests: &[PathBuf]) {
        for file_name in ["Cargo.toml", "Cargo.lock"] {
            CargoBuildScript::rerun_if_path_changed(self.root.join(file_name));
        }

        // Registry manifests are immutable, but path dependencies can change their
        // requests without changing the workspace's root manifest.
        for manifest in local_manifests {
            CargoBuildScript::rerun_if_path_changed(manifest);
        }
    }
}

/// Derive the workspace root from Cargo's build-script output directory.
///
/// Returns `None` when the target directory is outside the workspace or the layout is
/// unrecognized, allowing the caller to request an explicit override instead of silently
/// selecting the wrong manifest.
fn workspace_root_from_out_dir() -> Option<PathBuf> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR")?);

    // `OUT_DIR` always ends in `.../build/<unit>/out` or, in newer Cargo, one extra
    // directory between those two. The four in-tree layouts we recognise are:
    //
    // Older Cargo (`<pkg>-<hash>` holds `out`):
    //   <ws>/target/<profile>/build/<pkg>-<hash>/out
    //   <ws>/target/<triple>/<profile>/build/<pkg>-<hash>/out
    //
    // Newer Cargo (`<hash>` holds a nested unit dir, which holds `out`):
    //   <ws>/target/<profile>/build/<hash>/<unit>/out
    //   <ws>/target/<triple>/<profile>/build/<hash>/<unit>/out
    //
    // Walking up until a directory named `build` is unsafe: a crate named `build`
    // produces a directory of the same name at the same depth in both layouts. The
    // parent of `out` distinguishes them: `<pkg>-<hash>` is the older unit directory.
    let package_name = env::var("CARGO_PKG_NAME").ok()?;
    let unit_name = out_dir.parent()?.file_name()?.to_string_lossy();
    let nested = !unit_name.starts_with(&format!("{package_name}-"));

    let levels = if nested { 3 } else { 2 };
    let build_dir = ancestor(&out_dir, levels);

    // If the layout is unrecognized, don't guess and let the user provide it explicitly
    if build_dir.file_name() != Some(OsStr::new("build")) {
        return None;
    }

    // Above `build` sit `<profile>` and `target`. Cargo also inserts `<triple>` when
    // `--target` or `[build] target` is set, including when that triple equals HOST.
    // `HOST == TARGET` therefore cannot choose the depth. Try both:
    //
    //   <ws>/target/<profile>/build                  → 3 ancestors
    //   <ws>/target/<triple>/<profile>/build         → 4 ancestors
    //
    // and keep the first ancestor that actually contains a Cargo.toml. Custom
    // `--target-dir` layouts (for example colcon's `build/<pkg>`) miss both and
    // return `None`.
    [3, 4]
        .into_iter()
        .map(|depth| ancestor(&build_dir, depth))
        .find(|root| root.join("Cargo.toml").is_file())
}

/// Return the ancestor `levels` above `path`.
fn ancestor(path: &Path, levels: usize) -> PathBuf {
    let mut path = path.to_path_buf();

    for _ in 0..levels {
        path.pop();
    }

    path
}

/// Run `cargo metadata`, retrying from strict offline mode to normal online resolution.
///
/// Metadata loads dev dependencies that an outer `cargo build` may not have fetched.
/// Filtering to `TARGET` avoids fetching dependencies for inactive platforms.
fn run_cargo_metadata(manifest_path: &Path) -> Result<String, String> {
    if !manifest_path.is_file() {
        return Err(format!("`{}` does not exist", manifest_path.display()));
    }

    let cargo = env::var_os("CARGO").unwrap_or_else(|| OsStr::new("cargo").to_owned());
    let target = env::var_os("TARGET").ok_or("Cargo did not provide the build script's TARGET")?;

    let workspace_root = manifest_path
        .parent()
        .expect("Cargo.toml should have a parent directory");

    let mut last_error = String::new();

    // Prefer a read-only query, then relax one constraint at a time:
    //
    // 1. `--offline --locked` — cache is complete and the lockfile is current
    // 2. `--offline`          — lockfile is stale but every crate is already cached
    // 3. `--locked`           — lockfile is current but a crate still needs fetching
    //                           (`cargo metadata` always loads dev-dependencies, even
    //                           when the outer `cargo build` did not fetch them)
    // 4. neither              — last resort: allow both a fetch and a lockfile update
    for (offline, locked) in [(true, true), (true, false), (false, true), (false, false)] {
        let mut command = Command::new(&cargo);

        command
            .args(["metadata", "--format-version", "1"])
            .arg("--filter-platform")
            .arg(&target)
            .arg("--manifest-path")
            .arg(manifest_path)
            // Cargo discovers `.cargo/config.toml` from cwd, not from --manifest-path.
            .current_dir(workspace_root);

        if offline {
            command.arg("--offline");
        }

        if locked {
            command.arg("--locked");
        }

        match command.output() {
            Ok(output) if output.status.success() => {
                return String::from_utf8(output.stdout)
                    .map_err(|err| format!("`cargo metadata` produced invalid UTF-8: {err}"));
            }
            Ok(output) => last_error = String::from_utf8_lossy(&output.stderr).trim().to_owned(),
            Err(err) => last_error = err.to_string(),
        }
    }

    Err(format!(
        "failed to run `cargo metadata` for {}:\n{last_error}",
        manifest_path.display()
    ))
}
