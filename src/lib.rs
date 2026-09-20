#[cfg(feature = "use_ros_shim")]
#[allow(missing_docs)]
mod shim;
#[cfg(feature = "use_ros_shim")]
pub use shim::*;

#[cfg(not(feature = "use_ros_shim"))]
include!(concat!(env!("OUT_DIR"), "/interfaces.rs"));
