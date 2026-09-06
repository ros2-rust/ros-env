use super::cargo::CargoBuildScript;
use super::interfaces::{
    InterfaceSelection, interface_dependency_names, package_name_from_rust_dir,
};
use cargo_toml::Manifest;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

impl InterfaceSelection {
    /// Ask Cargo to rerun when a selected generated manifest or source changes.
    pub(crate) fn rerun_if_sources_change(&self) {
        for cargo_toml in &self.manifests {
            CargoBuildScript::rerun_if_path_changed(cargo_toml);

            if let Some(rust_dir) = cargo_toml.parent() {
                CargoBuildScript::rerun_if_path_changed(rust_dir.join("src"));
            }
        }
    }

    /// Render and write the complete source file included by `src/lib.rs`.
    pub(crate) fn write_to(&self, output_path: &Path) {
        let source: String = self
            .manifests
            .iter()
            .filter_map(|cargo_toml| generate_package_module(cargo_toml))
            .collect();

        write_interfaces(output_path, &source);
    }
}

/// Write an empty generated source file when no interfaces can or need to be selected.
pub(crate) fn write_empty_interfaces(output_path: &Path) {
    write_interfaces(output_path, "");
}

/// Render one ROS package as a module of `ros_env`.
///
/// `rosidl_generator_rs` installs a package's crate like this:
///
/// ```text
/// <prefix>/share/std_msgs/rust/
///     Cargo.toml       ROS deps as `*` requirements, e.g. builtin_interfaces
///     src/lib.rs       crate root, only used when compiled as its own crate
///     src/msg.rs       one file per interface kind (msg, srv, action)
///     src/msg/rmw.rs   the kind's RMW counterpart
/// ```
///
/// which this turns into:
///
/// ```text
/// pub mod std_msgs {
///     pub mod msg {
///         use crate::builtin_interfaces;
///         include!("<prefix>/share/std_msgs/rust/src/msg.rs");
///         pub mod rmw { /* include!(".../src/msg/rmw.rs") */ }
///     }
/// }
/// ```
fn generate_package_module(cargo_toml: &Path) -> Option<String> {
    let rust_dir = cargo_toml.parent()?;
    let package_name = package_name_from_rust_dir(rust_dir);

    let dependency_imports = Manifest::from_path(cargo_toml)
        .map(|manifest| generate_dependency_imports(&manifest))
        .unwrap_or_default();

    let kind_modules: String = generated_kind_sources(&rust_dir.join("src"))
        .into_iter()
        .filter_map(|path| generate_kind_module(path, &dependency_imports))
        .collect();

    Some(format!(
        "#[allow(unused_imports, missing_docs)]\
         pub mod {package_name} {{\
             {kind_modules}\
         }}"
    ))
}

/// List the kind sources in a generated crate's `src/`, one per `msg`, `srv`, `action`.
///
/// An unreadable `src/` yields nothing rather than failing: the package simply
/// contributes no modules.
fn generated_kind_sources(src_dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(src_dir) else {
        return Vec::new();
    };

    entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| is_generated_kind_source(path))
        .collect()
}

/// Accept `src/<kind>.rs`, rejecting the two files that must not become modules.
///
/// `lib.rs` is the standalone crate root and would redeclare the whole tree. A
/// top-level `rmw.rs` is only emitted by some generators and has to stay nested inside
/// its kind module, which `generate_kind_module` handles.
fn is_generated_kind_source(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    match path.file_name().and_then(|name| name.to_str()) {
        Some("lib.rs" | "rmw.rs") | None => false,
        Some(_) => true,
    }
}

/// Render one kind source (`msg.rs`, `srv.rs`, …) and its matching RMW implementation.
///
/// The module name is the file stem, so `src/msg.rs` becomes `pub mod msg` and its
/// `src/msg/rmw.rs` counterpart becomes a nested `pub mod rmw`. Both need the imports:
/// each file names its ROS dependencies on its own.
///
/// Paths are made absolute because `include!` resolves relative paths against the file
/// doing the including, which here is `$OUT_DIR/interfaces.rs`. Backslashes become
/// forward slashes so Windows paths survive being written into a Rust string literal.
fn generate_kind_module(source_path: PathBuf, dependency_imports: &str) -> Option<String> {
    let source_path =
        std::path::absolute(source_path).expect("Failed to get absolute path for idiomatic module");
    let module_name = source_path.file_stem()?.to_str()?;
    let idiomatic_path = source_path.to_string_lossy().replace('\\', "/");
    let rmw_path = source_path
        .parent()
        .expect("Failed to create rmw path")
        .join(module_name)
        .join("rmw.rs")
        .to_string_lossy()
        .replace('\\', "/");

    Some(format!(
        "pub mod {module_name} {{\
             {dependency_imports}\
             include!(\"{idiomatic_path}\");\
             pub mod rmw {{\
                 {dependency_imports}\
                 include!(\"{rmw_path}\");\
             }}\
         }}"
    ))
}

/// Render imports that expose generated dependencies inside a package module.
fn generate_dependency_imports(manifest: &Manifest) -> String {
    interface_dependency_names(manifest)
        .into_iter()
        .map(|name| format!("use crate::{name};\n"))
        .collect()
}

/// Write and best-effort format the generated source.
fn write_interfaces(output_path: &Path, source: &str) {
    fs::write(output_path, source).expect("Failed to write interfaces.rs");
    try_rustfmt(output_path);
}

/// Format generated Rust when `rustfmt` is available, warning rather than failing if not.
fn try_rustfmt(path: &Path) {
    match Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
        .arg(path)
        .status()
    {
        Ok(status) if status.success() => {}
        Ok(status) => {
            CargoBuildScript::warning(format_args!("rustfmt exited with status: {status}"))
        }
        Err(err) => CargoBuildScript::warning(format_args!(
            "failed to run rustfmt for {}: {err}",
            path.display()
        )),
    }
}
