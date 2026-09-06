# ros-env
Top-level Rust crate to include generated Rust code found in a sourced ROS 2 workspace.

## Usage
Declare the interface packages your crate needs, then use them through `ros_env`:
```toml
[dependencies]
ros-env = "0.3"

[package.metadata.ros-env]
interfaces = ["shape_msgs"]
```
```rust
use ros_env::shape_msgs::msg::Plane;
```

Only the packages you and your dependencies ask for are included, together with the interface 
packages they depend on. In a virtual workspace, where there is no root package to declare on, 
use `[workspace.metadata.ros-env]` instead.

## Details
Every crate in the dependency graph may declare interfaces, and `ros-env` includes the
union of all of those requests. A library therefore only has to declare what it uses
itself, and an application that pulls in several libraries that depend on message packages
automatically gets everything they need without having to list the packages from their dependencies.

Within one resolved version of `ros-env`, Cargo builds one crate holding the union, so
two crates that requested different interfaces still share the same types. A
`ros_env::std_msgs::msg::Header` handed over by a dependency is the same type its
dependents name.

The requested packages are looked up under `share/<package>/rust` in the prefixes on
`AMENT_PREFIX_PATH`. Overlays win over underlays, following the search path order. A
package that is requested but cannot be found fails the build, naming the crate that
asked for it.

## Locating the workspace
Cargo has no supported way to tell a dependency's build script which workspace is being
built, so `ros-env` derives it from `OUT_DIR`. That guess holds for the usual layout but not, 
for example, when the target directory lives outside the cargo workspace (e.g. for colcon build). 
Rather than guess wrong, the build fails and asks you to declare the root. The tidiest way 
is a one-off entry in the workspace's `.cargo/config.toml`:

```toml
[env]
CARGO_WORKSPACE_DIR = { value = "", relative = true }
```

This is inspired from [embuild](https://docs.rs/embuild/latest/embuild/cargo/fn.workspace_dir.html)
with some tweaks for our use case.

## Limitations
- The [include!()](https://doc.rust-lang.org/std/macro.include.html) macro is literal text
  inclusion, so build times scale with the size of the requested closure rather than with
  the number of crates Cargo has to compile.
- Cargo can resolve semver-incompatible versions of `ros-env` into the same graph. Types
  from those crate instances remain incompatible even when generated from identical ROS
  definitions. All participating crates must therefore use compatible `ros-env` version
  requirements.
- The dependencies of the included crates are not included. You cannot dynamically alter
  cargo dependencies through anything other than features, and features need to be
  explicitly declared and enabled. As such, this crate must have all expected dependencies
  itself (hence why this crate has a `serde` dependency for example).

## AI Policy
Generative tools are allowed in producing contributions to its projects, with some qualifications:

- Any contribution may consist, in whole or in part, of the output of one or more generative tools.
- Any use of generative tools in a contribution must be disclosed at the time of making the contribution.
- The disclosure must be recorded in a way that ensures it has the same or greater lifetime as the contribution itself.

For source code contributions, you should add a disclosure statement in the commit message for all commits where some portion of the source code was generated.

`Assisted-by: AGENT_NAME:MODEL_VERSION [TOOL1] [TOOL2]`

Provide a similar disclosure statement in the PR description.

See the projects [AI Policy](https://github.com/ros2-rust/ros2_rust/blob/main/docs/AI_POLICY.md) for more details.
