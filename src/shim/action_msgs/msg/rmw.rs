// Copied verbatim from the generated Rust code for `action_msgs` 1.2.3,
// built from the `humble` distro. Do not edit. Run `vendor_interfaces.py`
// to refresh it.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[link(name = "action_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__action_msgs__msg__GoalInfo(
    ) -> *const std::ffi::c_void;
}

#[link(name = "action_msgs__rosidl_generator_c")]
extern "C" {
    fn action_msgs__msg__GoalInfo__init(msg: *mut GoalInfo) -> bool;
    fn action_msgs__msg__GoalInfo__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<GoalInfo>,
        size: usize,
    ) -> bool;
    fn action_msgs__msg__GoalInfo__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GoalInfo>);
    fn action_msgs__msg__GoalInfo__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<GoalInfo>,
        out_seq: *mut rosidl_runtime_rs::Sequence<GoalInfo>,
    ) -> bool;
}

// Corresponds to action_msgs__msg__GoalInfo
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Goal ID

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoalInfo {
    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

    /// Time when the goal was accepted
    pub stamp: builtin_interfaces::msg::rmw::Time,
}

impl Default for GoalInfo {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !action_msgs__msg__GoalInfo__init(&mut msg as *mut _) {
                panic!("Call to action_msgs__msg__GoalInfo__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for GoalInfo {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { action_msgs__msg__GoalInfo__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { action_msgs__msg__GoalInfo__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { action_msgs__msg__GoalInfo__Sequence__copy(in_seq, out_seq as *mut _) }
    }
}

impl rosidl_runtime_rs::Message for GoalInfo {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for GoalInfo
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "action_msgs/msg/GoalInfo";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__action_msgs__msg__GoalInfo()
        }
    }
}

#[link(name = "action_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__action_msgs__msg__GoalStatus(
    ) -> *const std::ffi::c_void;
}

#[link(name = "action_msgs__rosidl_generator_c")]
extern "C" {
    fn action_msgs__msg__GoalStatus__init(msg: *mut GoalStatus) -> bool;
    fn action_msgs__msg__GoalStatus__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<GoalStatus>,
        size: usize,
    ) -> bool;
    fn action_msgs__msg__GoalStatus__Sequence__fini(
        seq: *mut rosidl_runtime_rs::Sequence<GoalStatus>,
    );
    fn action_msgs__msg__GoalStatus__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<GoalStatus>,
        out_seq: *mut rosidl_runtime_rs::Sequence<GoalStatus>,
    ) -> bool;
}

// Corresponds to action_msgs__msg__GoalStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// An action goal can be in one of these states after it is accepted by an action
/// server.
///
/// For more information, see http://design.ros2.org/articles/actions.html

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoalStatus {
    /// Goal info (contains ID and timestamp).
    pub goal_info: super::super::msg::rmw::GoalInfo,

    /// Action goal state-machine status.
    pub status: i8,
}

impl GoalStatus {
    /// Indicates status has not been properly set.
    pub const STATUS_UNKNOWN: i8 = 0;

    /// The goal has been accepted and is awaiting execution.
    pub const STATUS_ACCEPTED: i8 = 1;

    /// The goal is currently being executed by the action server.
    pub const STATUS_EXECUTING: i8 = 2;

    /// The client has requested that the goal be canceled and the action server has
    /// accepted the cancel request.
    pub const STATUS_CANCELING: i8 = 3;

    /// The goal was achieved successfully by the action server.
    pub const STATUS_SUCCEEDED: i8 = 4;

    /// The goal was canceled after an external request from an action client.
    pub const STATUS_CANCELED: i8 = 5;

    /// The goal was terminated by the action server without an external request.
    pub const STATUS_ABORTED: i8 = 6;
}

impl Default for GoalStatus {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !action_msgs__msg__GoalStatus__init(&mut msg as *mut _) {
                panic!("Call to action_msgs__msg__GoalStatus__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for GoalStatus {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { action_msgs__msg__GoalStatus__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { action_msgs__msg__GoalStatus__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { action_msgs__msg__GoalStatus__Sequence__copy(in_seq, out_seq as *mut _) }
    }
}

impl rosidl_runtime_rs::Message for GoalStatus {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for GoalStatus
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "action_msgs/msg/GoalStatus";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__action_msgs__msg__GoalStatus()
        }
    }
}

#[link(name = "action_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__action_msgs__msg__GoalStatusArray(
    ) -> *const std::ffi::c_void;
}

#[link(name = "action_msgs__rosidl_generator_c")]
extern "C" {
    fn action_msgs__msg__GoalStatusArray__init(msg: *mut GoalStatusArray) -> bool;
    fn action_msgs__msg__GoalStatusArray__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<GoalStatusArray>,
        size: usize,
    ) -> bool;
    fn action_msgs__msg__GoalStatusArray__Sequence__fini(
        seq: *mut rosidl_runtime_rs::Sequence<GoalStatusArray>,
    );
    fn action_msgs__msg__GoalStatusArray__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<GoalStatusArray>,
        out_seq: *mut rosidl_runtime_rs::Sequence<GoalStatusArray>,
    ) -> bool;
}

// Corresponds to action_msgs__msg__GoalStatusArray
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// An array of goal statuses.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoalStatusArray {
    // This member is not documented.
    #[allow(missing_docs)]
    pub status_list: rosidl_runtime_rs::Sequence<super::super::msg::rmw::GoalStatus>,
}

impl Default for GoalStatusArray {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !action_msgs__msg__GoalStatusArray__init(&mut msg as *mut _) {
                panic!("Call to action_msgs__msg__GoalStatusArray__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for GoalStatusArray {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { action_msgs__msg__GoalStatusArray__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { action_msgs__msg__GoalStatusArray__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { action_msgs__msg__GoalStatusArray__Sequence__copy(in_seq, out_seq as *mut _) }
    }
}

impl rosidl_runtime_rs::Message for GoalStatusArray {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for GoalStatusArray
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "action_msgs/msg/GoalStatusArray";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__action_msgs__msg__GoalStatusArray(
            )
        }
    }
}
