// Copied verbatim from the generated Rust code for `rosgraph_msgs` 1.2.3,
// built from the `humble` distro. Do not edit. Run `vendor_interfaces.py`
// to refresh it.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[link(name = "rosgraph_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rosgraph_msgs__msg__Action(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rosgraph_msgs__rosidl_generator_c")]
extern "C" {
    fn rosgraph_msgs__msg__Action__init(msg: *mut Action) -> bool;
    fn rosgraph_msgs__msg__Action__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<Action>,
        size: usize,
    ) -> bool;
    fn rosgraph_msgs__msg__Action__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Action>);
    fn rosgraph_msgs__msg__Action__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<Action>,
        out_seq: *mut rosidl_runtime_rs::Sequence<Action>,
    ) -> bool;
}

// Corresponds to rosgraph_msgs__msg__Action
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Describes a single Action endpoint, which may be a Server or Client

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Action {
    /// Fully qualified name of the Action
    pub name: rosidl_runtime_rs::String,

    /// An action is actually a composition of the following fundamental ROS entities
    pub send_goal: super::super::msg::rmw::Service,

    // This member is not documented.
    #[allow(missing_docs)]
    pub get_result: super::super::msg::rmw::Service,

    // This member is not documented.
    #[allow(missing_docs)]
    pub cancel_goal: super::super::msg::rmw::Service,

    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::msg::rmw::Topic,

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::super::msg::rmw::Topic,
}

impl Default for Action {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rosgraph_msgs__msg__Action__init(&mut msg as *mut _) {
                panic!("Call to rosgraph_msgs__msg__Action__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for Action {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__Action__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__Action__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__Action__Sequence__copy(in_seq, out_seq as *mut _) }
    }
}

impl rosidl_runtime_rs::Message for Action {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for Action
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rosgraph_msgs/msg/Action";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rosgraph_msgs__msg__Action()
        }
    }
}

#[link(name = "rosgraph_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rosgraph_msgs__msg__Clock(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rosgraph_msgs__rosidl_generator_c")]
extern "C" {
    fn rosgraph_msgs__msg__Clock__init(msg: *mut Clock) -> bool;
    fn rosgraph_msgs__msg__Clock__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<Clock>,
        size: usize,
    ) -> bool;
    fn rosgraph_msgs__msg__Clock__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Clock>);
    fn rosgraph_msgs__msg__Clock__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<Clock>,
        out_seq: *mut rosidl_runtime_rs::Sequence<Clock>,
    ) -> bool;
}

// Corresponds to rosgraph_msgs__msg__Clock
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message communicates the current time.
///
/// For more information, see https://design.ros2.org/articles/clock_and_time.html.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Clock {
    // This member is not documented.
    #[allow(missing_docs)]
    pub clock: builtin_interfaces::msg::rmw::Time,
}

impl Default for Clock {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rosgraph_msgs__msg__Clock__init(&mut msg as *mut _) {
                panic!("Call to rosgraph_msgs__msg__Clock__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for Clock {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__Clock__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__Clock__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__Clock__Sequence__copy(in_seq, out_seq as *mut _) }
    }
}

impl rosidl_runtime_rs::Message for Clock {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for Clock
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rosgraph_msgs/msg/Clock";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rosgraph_msgs__msg__Clock()
        }
    }
}

#[link(name = "rosgraph_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rosgraph_msgs__msg__Graph(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rosgraph_msgs__rosidl_generator_c")]
extern "C" {
    fn rosgraph_msgs__msg__Graph__init(msg: *mut Graph) -> bool;
    fn rosgraph_msgs__msg__Graph__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<Graph>,
        size: usize,
    ) -> bool;
    fn rosgraph_msgs__msg__Graph__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Graph>);
    fn rosgraph_msgs__msg__Graph__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<Graph>,
        out_seq: *mut rosidl_runtime_rs::Sequence<Graph>,
    ) -> bool;
}

// Corresponds to rosgraph_msgs__msg__Graph
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Represents a ROS node graph, which is only a collection of nodes

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Graph {
    // This member is not documented.
    #[allow(missing_docs)]
    pub nodes: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Node>,
}

impl Default for Graph {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rosgraph_msgs__msg__Graph__init(&mut msg as *mut _) {
                panic!("Call to rosgraph_msgs__msg__Graph__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for Graph {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__Graph__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__Graph__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__Graph__Sequence__copy(in_seq, out_seq as *mut _) }
    }
}

impl rosidl_runtime_rs::Message for Graph {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for Graph
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rosgraph_msgs/msg/Graph";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rosgraph_msgs__msg__Graph()
        }
    }
}

#[link(name = "rosgraph_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rosgraph_msgs__msg__InterfaceType(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rosgraph_msgs__rosidl_generator_c")]
extern "C" {
    fn rosgraph_msgs__msg__InterfaceType__init(msg: *mut InterfaceType) -> bool;
    fn rosgraph_msgs__msg__InterfaceType__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<InterfaceType>,
        size: usize,
    ) -> bool;
    fn rosgraph_msgs__msg__InterfaceType__Sequence__fini(
        seq: *mut rosidl_runtime_rs::Sequence<InterfaceType>,
    );
    fn rosgraph_msgs__msg__InterfaceType__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<InterfaceType>,
        out_seq: *mut rosidl_runtime_rs::Sequence<InterfaceType>,
    ) -> bool;
}

// Corresponds to rosgraph_msgs__msg__InterfaceType
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Represent a type of a ROS Graph Interface

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InterfaceType {
    /// The plaintext namespaced name of the type - e.g. sensor_msgs/Image
    pub name: rosidl_runtime_rs::String,

    /// The hash uniquely identifies the exact structure of the type,
    /// the definition of which may change between package version
    pub hash: super::super::msg::rmw::TypeHash,
}

impl Default for InterfaceType {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rosgraph_msgs__msg__InterfaceType__init(&mut msg as *mut _) {
                panic!("Call to rosgraph_msgs__msg__InterfaceType__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for InterfaceType {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__InterfaceType__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__InterfaceType__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__InterfaceType__Sequence__copy(in_seq, out_seq as *mut _) }
    }
}

impl rosidl_runtime_rs::Message for InterfaceType {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for InterfaceType
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rosgraph_msgs/msg/InterfaceType";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rosgraph_msgs__msg__InterfaceType(
            )
        }
    }
}

#[link(name = "rosgraph_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rosgraph_msgs__msg__Node(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rosgraph_msgs__rosidl_generator_c")]
extern "C" {
    fn rosgraph_msgs__msg__Node__init(msg: *mut Node) -> bool;
    fn rosgraph_msgs__msg__Node__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<Node>,
        size: usize,
    ) -> bool;
    fn rosgraph_msgs__msg__Node__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Node>);
    fn rosgraph_msgs__msg__Node__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<Node>,
        out_seq: *mut rosidl_runtime_rs::Sequence<Node>,
    ) -> bool;
}

// Corresponds to rosgraph_msgs__msg__Node
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Represents the observable runtime state of a ROS Node
/// Therefore, does not perfectly align with the abstract specification which created it.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Node {
    /// Fully qualified node name (FQN)
    pub name: rosidl_runtime_rs::String,

    /// Parameter specifications for the node
    pub parameters: rosidl_runtime_rs::Sequence<rcl_interfaces::msg::rmw::ParameterDescriptor>,

    /// Current values of the node's parameters
    /// NOTE:
    ///   parameter_values[] must be empty, or the same size as parameters[]
    ///   When set, parameter_values[] match 1:1 with the same index in parameters[]
    pub parameter_values: rosidl_runtime_rs::Sequence<rcl_interfaces::msg::rmw::ParameterValue>,

    /// Communications endpoints - Topics, Services, and Actions
    pub publishers: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Topic>,

    // This member is not documented.
    #[allow(missing_docs)]
    pub subscriptions: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Topic>,

    // This member is not documented.
    #[allow(missing_docs)]
    pub service_clients: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Service>,

    // This member is not documented.
    #[allow(missing_docs)]
    pub service_servers: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Service>,

    // This member is not documented.
    #[allow(missing_docs)]
    pub action_clients: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Action>,

    // This member is not documented.
    #[allow(missing_docs)]
    pub action_servers: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Action>,
}

impl Default for Node {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rosgraph_msgs__msg__Node__init(&mut msg as *mut _) {
                panic!("Call to rosgraph_msgs__msg__Node__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for Node {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__Node__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__Node__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__Node__Sequence__copy(in_seq, out_seq as *mut _) }
    }
}

impl rosidl_runtime_rs::Message for Node {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for Node
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rosgraph_msgs/msg/Node";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_message_type_support_handle__rosgraph_msgs__msg__Node() }
    }
}

#[link(name = "rosgraph_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rosgraph_msgs__msg__QoSProfile(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rosgraph_msgs__rosidl_generator_c")]
extern "C" {
    fn rosgraph_msgs__msg__QoSProfile__init(msg: *mut QoSProfile) -> bool;
    fn rosgraph_msgs__msg__QoSProfile__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<QoSProfile>,
        size: usize,
    ) -> bool;
    fn rosgraph_msgs__msg__QoSProfile__Sequence__fini(
        seq: *mut rosidl_runtime_rs::Sequence<QoSProfile>,
    );
    fn rosgraph_msgs__msg__QoSProfile__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<QoSProfile>,
        out_seq: *mut rosidl_runtime_rs::Sequence<QoSProfile>,
    ) -> bool;
}

// Corresponds to rosgraph_msgs__msg__QoSProfile
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Message-based representation of ROS 2 Quality of Service settings
/// Default values are kept in sync with RMW by integration test
/// Note that SYSTEM_DEFAULT and BEST_AVAILABLE values cannot be an observed value,
/// because they resolve concretely at runtime.
/// They are included here for completeness to match the data structures in RMW

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct QoSProfile {
    /// Depth of the message queue (only meaningful when history==KEEP_LAST)
    pub depth: u32,

    /// Deadline between messages (0 for no deadline)
    pub deadline: builtin_interfaces::msg::rmw::Duration,

    /// Lifespan of each message (0 for infinite)
    pub lifespan: builtin_interfaces::msg::rmw::Duration,

    // This member is not documented.
    #[allow(missing_docs)]
    pub history: u8,

    // This member is not documented.
    #[allow(missing_docs)]
    pub reliability: u8,

    // This member is not documented.
    #[allow(missing_docs)]
    pub durability: u8,

    // This member is not documented.
    #[allow(missing_docs)]
    pub liveliness: u8,

    /// Lease duration for liveliness (0 for infinite)
    pub liveliness_lease_duration: builtin_interfaces::msg::rmw::Duration,
}

impl QoSProfile {
    /// History policy
    pub const HISTORY_SYSTEM_DEFAULT: u8 = 0;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HISTORY_KEEP_LAST: u8 = 1;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HISTORY_KEEP_ALL: u8 = 2;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const HISTORY_UNKNOWN: u8 = 3;

    /// Reliability policy
    pub const RELIABILITY_SYSTEM_DEFAULT: u8 = 0;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RELIABILITY_RELIABLE: u8 = 1;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RELIABILITY_BEST_EFFORT: u8 = 2;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RELIABILITY_UNKNOWN: u8 = 3;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RELIABILITY_BEST_AVAILABLE: u8 = 4;

    /// Durability policy
    pub const DURABILITY_SYSTEM_DEFAULT: u8 = 0;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DURABILITY_TRANSIENT_LOCAL: u8 = 1;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DURABILITY_VOLATILE: u8 = 2;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DURABILITY_UNKNOWN: u8 = 3;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DURABILITY_BEST_AVAILABLE: u8 = 4;

    /// Liveliness policy
    pub const LIVELINESS_SYSTEM_DEFAULT: u8 = 0;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LIVELINESS_AUTOMATIC: u8 = 1;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LIVELINESS_MANUAL_BY_TOPIC: u8 = 3;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LIVELINESS_UNKNOWN: u8 = 4;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LIVELINESS_BEST_AVAILABLE: u8 = 5;
}

impl Default for QoSProfile {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rosgraph_msgs__msg__QoSProfile__init(&mut msg as *mut _) {
                panic!("Call to rosgraph_msgs__msg__QoSProfile__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for QoSProfile {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__QoSProfile__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__QoSProfile__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__QoSProfile__Sequence__copy(in_seq, out_seq as *mut _) }
    }
}

impl rosidl_runtime_rs::Message for QoSProfile {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for QoSProfile
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rosgraph_msgs/msg/QoSProfile";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rosgraph_msgs__msg__QoSProfile()
        }
    }
}

#[link(name = "rosgraph_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rosgraph_msgs__msg__Service(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rosgraph_msgs__rosidl_generator_c")]
extern "C" {
    fn rosgraph_msgs__msg__Service__init(msg: *mut Service) -> bool;
    fn rosgraph_msgs__msg__Service__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<Service>,
        size: usize,
    ) -> bool;
    fn rosgraph_msgs__msg__Service__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Service>);
    fn rosgraph_msgs__msg__Service__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<Service>,
        out_seq: *mut rosidl_runtime_rs::Sequence<Service>,
    ) -> bool;
}

// Corresponds to rosgraph_msgs__msg__Service
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Describes a single Service endpoint, which may be a Server or Client

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Service {
    /// Fully qualified name of the Service
    pub name: rosidl_runtime_rs::String,

    /// Type and actual QoS of the request publisher (Client) or subscription (Server)
    pub request_type: super::super::msg::rmw::InterfaceType,

    // This member is not documented.
    #[allow(missing_docs)]
    pub request_qos: super::super::msg::rmw::QoSProfile,

    /// Type and actual QoS of the request subscription (Client) or publisher (Server)
    pub response_type: super::super::msg::rmw::InterfaceType,

    // This member is not documented.
    #[allow(missing_docs)]
    pub response_qos: super::super::msg::rmw::QoSProfile,
}

impl Default for Service {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rosgraph_msgs__msg__Service__init(&mut msg as *mut _) {
                panic!("Call to rosgraph_msgs__msg__Service__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for Service {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__Service__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__Service__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__Service__Sequence__copy(in_seq, out_seq as *mut _) }
    }
}

impl rosidl_runtime_rs::Message for Service {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for Service
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rosgraph_msgs/msg/Service";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rosgraph_msgs__msg__Service()
        }
    }
}

#[link(name = "rosgraph_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rosgraph_msgs__msg__Topic(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rosgraph_msgs__rosidl_generator_c")]
extern "C" {
    fn rosgraph_msgs__msg__Topic__init(msg: *mut Topic) -> bool;
    fn rosgraph_msgs__msg__Topic__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<Topic>,
        size: usize,
    ) -> bool;
    fn rosgraph_msgs__msg__Topic__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Topic>);
    fn rosgraph_msgs__msg__Topic__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<Topic>,
        out_seq: *mut rosidl_runtime_rs::Sequence<Topic>,
    ) -> bool;
}

// Corresponds to rosgraph_msgs__msg__Topic
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Describes a single topic endpoint, which may be a Publisher or Subscription

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Topic {
    /// Fully qualified name of the topic
    pub name: rosidl_runtime_rs::String,

    /// Type of the topic
    pub type_: super::super::msg::rmw::InterfaceType,

    /// Observed QoS of the endpoint
    pub qos: super::super::msg::rmw::QoSProfile,
}

impl Default for Topic {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rosgraph_msgs__msg__Topic__init(&mut msg as *mut _) {
                panic!("Call to rosgraph_msgs__msg__Topic__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for Topic {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__Topic__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__Topic__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__Topic__Sequence__copy(in_seq, out_seq as *mut _) }
    }
}

impl rosidl_runtime_rs::Message for Topic {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for Topic
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rosgraph_msgs/msg/Topic";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rosgraph_msgs__msg__Topic()
        }
    }
}

#[link(name = "rosgraph_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rosgraph_msgs__msg__TypeHash(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rosgraph_msgs__rosidl_generator_c")]
extern "C" {
    fn rosgraph_msgs__msg__TypeHash__init(msg: *mut TypeHash) -> bool;
    fn rosgraph_msgs__msg__TypeHash__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<TypeHash>,
        size: usize,
    ) -> bool;
    fn rosgraph_msgs__msg__TypeHash__Sequence__fini(
        seq: *mut rosidl_runtime_rs::Sequence<TypeHash>,
    );
    fn rosgraph_msgs__msg__TypeHash__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<TypeHash>,
        out_seq: *mut rosidl_runtime_rs::Sequence<TypeHash>,
    ) -> bool;
}

// Corresponds to rosgraph_msgs__msg__TypeHash
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// RIHS spec version

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TypeHash {
    // This member is not documented.
    #[allow(missing_docs)]
    pub version: u8,

    /// ROSIDL_TYPE_HASH_SIZE == 32
    pub value: [u8; 32],
}

impl Default for TypeHash {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rosgraph_msgs__msg__TypeHash__init(&mut msg as *mut _) {
                panic!("Call to rosgraph_msgs__msg__TypeHash__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for TypeHash {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__TypeHash__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__TypeHash__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rosgraph_msgs__msg__TypeHash__Sequence__copy(in_seq, out_seq as *mut _) }
    }
}

impl rosidl_runtime_rs::Message for TypeHash {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for TypeHash
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rosgraph_msgs/msg/TypeHash";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rosgraph_msgs__msg__TypeHash()
        }
    }
}
