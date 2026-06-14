# ros-env
Top-level Rust crate to include generated Rust code found in a sourced ROS 2 workspace.

## Usage
The [shape/msg/Plane](https://github.com/ros2/common_interfaces/blob/rolling/shape_msgs/msg/Plane.msg) message can be included with:
```rust
// Assuming the rust crate for `shape_msgs` is in the `AMENT_PREFIX_PATH`
use ros_env::shape_msgs::msg::Plane;
```

## Details
Any Rust crate found in the `AMENT_PREFIX_PATH` environment variable, that has opted in, will be `include!()`d.

By default, `ros-env` keeps compatibility with existing users by including every discovered opt-in generated interface package:

```toml
ros-env = "0.2"
```

Cargo feature selection is additive across the workspace: if any dependency enables a feature, it is unified for the final build.

For selective inclusion, disable default features and opt into the package features you need. `rclrs_core` is a feature alias for the common core interface set used by `rclrs` (`action_msgs`, `builtin_interfaces`, `rcl_interfaces`, `rosgraph_msgs`, and `unique_identifier_msgs`):

```toml
ros-env = { version = "0.2", default-features = false, features = ["rclrs_core"] }
```

If your crate or tests need extra generated interfaces, add them explicitly:

```toml
ros-env = { version = "0.2", default-features = false, features = ["rclrs_core", "example_interfaces", "test_msgs"] }
```

To opt in, the crate must have the following metadata present in the Cargo.toml
```toml
[package.metadata.ros-env]
include = true
```

The selectable package feature list is fixed to: `action_msgs`, `builtin_interfaces`, `rcl_interfaces`, `rosgraph_msgs`, `unique_identifier_msgs`, `example_interfaces`, and `test_msgs`.

Packages discovered in AMENT that are re-exported may depend on other generated packages via `*` Cargo dependencies. Those generated dependencies are included automatically when present, but non-generated dependencies remain normal Cargo dependencies and are not re-exported here.

`use_ros_shim` forwards to `rosidl_runtime_rs/use_ros_shim` and lets selective builds skip selected packages that are missing from `AMENT_PREFIX_PATH`. It does not synthesize ROS interface modules. Crates that need no-ROS docs/builds should provide their own stubs (for example `rclrs/src/vendor.rs`). Without the shim, selective mode still requires the selected packages to exist and be opt-in.

By default, crates generated from `rosidl_generator_rs` opt in.

## Limitations
- The [include!()](https://doc.rust-lang.org/std/macro.include.html) macro is literal text inclusion. As such, depending 
  on the number of generated crates found in `AMENT_PREFIX_PATH`, the build times for this crate can be long.
- Non-generated Cargo dependencies of included crates are not added dynamically. Cargo dependencies can only be changed
  through explicitly declared features, so this crate must declare expected non-generated dependencies itself (hence why
  this crate has a `serde` dependency for example).
