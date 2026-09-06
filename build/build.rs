mod cargo;
mod codegen;
mod interfaces;

use ament_rs::AMENT_PREFIX_PATH_ENV_VAR;
use std::fmt::Display;

use crate::{
    cargo::{CARGO_WORKSPACE_DIR_ENV_VAR, CargoBuildScript, CargoWorkspace},
    codegen::write_empty_interfaces,
    interfaces::{InterfaceRegistry, METADATA_KEY, METADATA_TABLE},
};

/// Stop the build with a diagnostic owned by `ros-env`.
fn fail(message: impl Display) -> ! {
    eprintln!("ros-env: {message}");
    std::process::exit(1);
}

fn main() {
    CargoBuildScript::rerun_if_environment_changed(AMENT_PREFIX_PATH_ENV_VAR);
    CargoBuildScript::rerun_if_environment_changed(CARGO_WORKSPACE_DIR_ENV_VAR);

    let output_path = CargoBuildScript::output_path("interfaces.rs");
    let registry = InterfaceRegistry::discover();

    // Documentation builds and other ROS-less environments have nothing to include.
    if registry.is_empty() {
        CargoBuildScript::warning(format_args!(
            "no generated interface packages found on {AMENT_PREFIX_PATH_ENV_VAR}, \
             ros_env will be empty"
        ));
        write_empty_interfaces(&output_path);
        return;
    }

    let workspace = CargoWorkspace::locate().unwrap_or_else(|err| fail(err));
    let requests = workspace
        .interface_requests()
        .unwrap_or_else(|err| fail(err));
    workspace.rerun_if_manifests_change(requests.local_manifests());

    if requests.is_empty() {
        CargoBuildScript::warning(format_args!(
            "no crate in {} declares `[package.metadata.{METADATA_TABLE}] {METADATA_KEY}`, \
             ros_env will be empty",
            workspace.root().display()
        ));
        write_empty_interfaces(&output_path);
        return;
    }

    let selection = registry.resolve(&requests).unwrap_or_else(|err| fail(err));
    selection.rerun_if_sources_change();
    selection.write_to(&output_path);
}
