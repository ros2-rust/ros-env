use ament_rs::{search_paths::get_search_paths, AMENT_PREFIX_PATH_ENV_VAR};
use cargo_toml::Manifest;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

fn is_marked_for_inclusion(path: &PathBuf) -> bool {
    Manifest::from_path(path)
        .map(|manifest| {
            manifest
                .package
                .as_ref()
                .and_then(|pkg| pkg.metadata.as_ref())
                .and_then(|metadata| metadata.get("ros-env"))
                .and_then(|ros_env| ros_env.get("include"))
                .and_then(|include| include.as_bool())
                .unwrap_or(false)
        })
        .unwrap_or(false)
}

fn star_deps_to_use(manifest: &Manifest) -> String {
    // Find all dependencies for this crate that have a `*` version requirement.
    // We will assume that these are other exported dependencies that need symbols
    // exposed in their module.
    manifest
        .dependencies
        .iter()
        .filter(|(_, version)| version.req() == "*")
        .map(|(name, _)| format!("use crate::{name};\n"))
        .collect::<String>()
}

fn crate_name_from_ament_package_dir(package_dir: &PathBuf) -> &str {
    package_dir
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|name| name.to_str())
        .expect("AMENT package directory should be <prefix>/share/<package>/rust")
}

fn try_rustfmt(path: &Path) {
    match Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .arg(path)
        .status()
    {
        Ok(status) if status.success() => {}
        Ok(status) => {
            println!("cargo:warning=rustfmt exited with status: {status}");
        }
        Err(err) => {
            println!(
                "cargo:warning=failed to run rustfmt for {}: {err}",
                path.display()
            );
        }
    }
}

fn main() {
    println!("cargo:rerun-if-env-changed={AMENT_PREFIX_PATH_ENV_VAR}");

    let ament_prefix_paths = get_search_paths().unwrap_or_default();

    // Re-export any generated interface crates that we find
    let export_crate_tomls: Vec<PathBuf> = ament_prefix_paths
        .iter()
        .map(PathBuf::from)
        .flat_map(|base_path| {
            // 1. Try to read share/ directory
            fs::read_dir(base_path.join("share")).into_iter().flatten()
        })
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .flat_map(|package_dir| {
            // 2. Try to read <package>/rust/ directory
            fs::read_dir(package_dir.path().join("rust"))
                .into_iter()
                .flatten()
        })
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.file_name() == Some(std::ffi::OsStr::new("Cargo.toml")))
        .filter(is_marked_for_inclusion)
        .collect();

    // Make sure the script re-runs if any of the sources we want to include change.
    for cargo_toml in &export_crate_tomls {
        println!("cargo:rerun-if-changed={}", cargo_toml.display());

        if let Some(package_dir) = cargo_toml.parent() {
            let src_dir = package_dir.join("src");
            println!("cargo:rerun-if-changed={}", src_dir.display());
        }
    }

    let content: String = export_crate_tomls
        .iter()
        .filter_map(|path| path.parent().map(|p| p.to_path_buf()))
        .map(|package_dir| {
            let package = crate_name_from_ament_package_dir(&package_dir);

            // Find all dependencies for this crate that have a `*` version requirement.
            // We will assume that these are other exported dependencies that need symbols
            // exposed in their module.
            let dependencies: String = Manifest::from_path(package_dir.clone().join("Cargo.toml"))
                .iter()
                .map(star_deps_to_use)
                .collect();

            let internal_mods: String = fs::read_dir(package_dir.join("src"))
                .into_iter()
                .flatten()
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().is_file())
                // Ignore lib.rs and any rmw.rs. lib.rs is only used if the crate is consumed
                // independently, and rmw.rs files need their top-level module
                // (i.e., msg, srv, action) to exist to be re-exported.
                .filter(|entry| {
                    let name = entry.file_name();
                    name != "lib.rs" && name != "rmw.rs"
                })
                // Wrap the inclusion of each file in a module matching the file stem
                // so that the generated code can be imported like `ros_env::std_msgs::msgs::Bool`
                .filter_map(|e| {
                    let path = std::path::absolute(e.path()).expect("Failed to get absolute path for idiomatic module");
                    path.file_stem()
                        .and_then(|stem| stem.to_str())
                        .map(|stem| {
                            let idiomatic_path = path.to_string_lossy().replace('\\', "/");

                            let parent = path
                                .parent()
                                .expect("Failed to create rmw path");

                            let rmw_path = parent
                                .join(stem)
                                .join("rmw.rs")
                                .to_string_lossy()
                                .replace('\\', "/");

                            format!("pub mod {stem} {{ {dependencies} include!(\"{idiomatic_path}\"); pub mod rmw {{ {dependencies} include!(\"{rmw_path}\"); }} }}")
                        })
                })
                .collect();

            format!("#[allow(unused_imports, missing_docs)]\npub mod {package} {{ {internal_mods} }}")
        })
        .collect();

    let out_path =
        PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR not set")).join("interfaces.rs");
    fs::write(&out_path, content).expect("Failed to write interfaces.rs");

    try_rustfmt(&out_path);
}
