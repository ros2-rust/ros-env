// Copied verbatim from the generated Rust code for `rcl_interfaces` 1.2.3,
// built from the `humble` distro. Do not edit. Run `vendor_interfaces.py`
// to refresh it.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[link(name = "rcl_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__FloatingPointRange(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rcl_interfaces__rosidl_generator_c")]
extern "C" {
    fn rcl_interfaces__msg__FloatingPointRange__init(msg: *mut FloatingPointRange) -> bool;
    fn rcl_interfaces__msg__FloatingPointRange__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<FloatingPointRange>,
        size: usize,
    ) -> bool;
    fn rcl_interfaces__msg__FloatingPointRange__Sequence__fini(
        seq: *mut rosidl_runtime_rs::Sequence<FloatingPointRange>,
    );
    fn rcl_interfaces__msg__FloatingPointRange__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<FloatingPointRange>,
        out_seq: *mut rosidl_runtime_rs::Sequence<FloatingPointRange>,
    ) -> bool;
}

// Corresponds to rcl_interfaces__msg__FloatingPointRange
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Represents bounds and a step value for a floating point typed parameter.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FloatingPointRange {
    /// Start value for valid values, inclusive.
    pub from_value: f64,

    /// End value for valid values, inclusive.
    pub to_value: f64,

    /// Size of valid steps between the from and to bound.
    ///
    /// Step is considered to be a magnitude, therefore negative values are treated
    /// the same as positive values, and a step value of zero implies a continuous
    /// range of values.
    ///
    /// Ideally, the step would be less than or equal to the distance between the
    /// bounds, as well as an even multiple of the distance between the bounds, but
    /// neither are required.
    ///
    /// If the absolute value of the step is larger than or equal to the distance
    /// between the two bounds, then the bounds will be the only valid values. e.g. if
    /// the range is defined as {from_value: 1.0, to_value: 2.0, step: 5.0} then the
    /// valid values will be 1.0 and 2.0.
    ///
    /// If the step is less than the distance between the bounds, but the distance is
    /// not a multiple of the step, then the "to" bound will always be a valid value,
    /// e.g. if the range is defined as {from_value: 2.0, to_value: 5.0, step: 2.0}
    /// then the valid values will be 2.0, 4.0, and 5.0.
    pub step: f64,
}

impl Default for FloatingPointRange {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rcl_interfaces__msg__FloatingPointRange__init(&mut msg as *mut _) {
                panic!("Call to rcl_interfaces__msg__FloatingPointRange__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for FloatingPointRange {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__FloatingPointRange__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__FloatingPointRange__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe {
            rcl_interfaces__msg__FloatingPointRange__Sequence__copy(in_seq, out_seq as *mut _)
        }
    }
}

impl rosidl_runtime_rs::Message for FloatingPointRange {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for FloatingPointRange
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rcl_interfaces/msg/FloatingPointRange";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__FloatingPointRange()
        }
    }
}

#[link(name = "rcl_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__IntegerRange(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rcl_interfaces__rosidl_generator_c")]
extern "C" {
    fn rcl_interfaces__msg__IntegerRange__init(msg: *mut IntegerRange) -> bool;
    fn rcl_interfaces__msg__IntegerRange__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<IntegerRange>,
        size: usize,
    ) -> bool;
    fn rcl_interfaces__msg__IntegerRange__Sequence__fini(
        seq: *mut rosidl_runtime_rs::Sequence<IntegerRange>,
    );
    fn rcl_interfaces__msg__IntegerRange__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<IntegerRange>,
        out_seq: *mut rosidl_runtime_rs::Sequence<IntegerRange>,
    ) -> bool;
}

// Corresponds to rcl_interfaces__msg__IntegerRange
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Represents bounds and a step value for an integer typed parameter.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IntegerRange {
    /// Start value for valid values, inclusive.
    pub from_value: i64,

    /// End value for valid values, inclusive.
    pub to_value: i64,

    /// Size of valid steps between the from and to bound.
    ///
    /// A step value of zero implies a continuous range of values. Ideally, the step
    /// would be less than or equal to the distance between the bounds, as well as an
    /// even multiple of the distance between the bounds, but neither are required.
    ///
    /// If the absolute value of the step is larger than or equal to the distance
    /// between the two bounds, then the bounds will be the only valid values. e.g. if
    /// the range is defined as {from_value: 1, to_value: 2, step: 5} then the valid
    /// values will be 1 and 2.
    ///
    /// If the step is less than the distance between the bounds, but the distance is
    /// not a multiple of the step, then the "to" bound will always be a valid value,
    /// e.g. if the range is defined as {from_value: 2, to_value: 5, step: 2} then
    /// the valid values will be 2, 4, and 5.
    pub step: u64,
}

impl Default for IntegerRange {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rcl_interfaces__msg__IntegerRange__init(&mut msg as *mut _) {
                panic!("Call to rcl_interfaces__msg__IntegerRange__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for IntegerRange {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__IntegerRange__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__IntegerRange__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__IntegerRange__Sequence__copy(in_seq, out_seq as *mut _) }
    }
}

impl rosidl_runtime_rs::Message for IntegerRange {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for IntegerRange
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rcl_interfaces/msg/IntegerRange";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__IntegerRange(
            )
        }
    }
}

#[link(name = "rcl_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ListParametersResult(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rcl_interfaces__rosidl_generator_c")]
extern "C" {
    fn rcl_interfaces__msg__ListParametersResult__init(msg: *mut ListParametersResult) -> bool;
    fn rcl_interfaces__msg__ListParametersResult__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<ListParametersResult>,
        size: usize,
    ) -> bool;
    fn rcl_interfaces__msg__ListParametersResult__Sequence__fini(
        seq: *mut rosidl_runtime_rs::Sequence<ListParametersResult>,
    );
    fn rcl_interfaces__msg__ListParametersResult__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<ListParametersResult>,
        out_seq: *mut rosidl_runtime_rs::Sequence<ListParametersResult>,
    ) -> bool;
}

// Corresponds to rcl_interfaces__msg__ListParametersResult
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// The resulting parameters under the given prefixes.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListParametersResult {
    // This member is not documented.
    #[allow(missing_docs)]
    pub names: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

    /// The resulting prefixes under the given prefixes.
    /// TODO(wjwwood): link to prefix definition and rules.
    pub prefixes: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,
}

impl Default for ListParametersResult {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rcl_interfaces__msg__ListParametersResult__init(&mut msg as *mut _) {
                panic!("Call to rcl_interfaces__msg__ListParametersResult__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for ListParametersResult {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__ListParametersResult__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__ListParametersResult__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe {
            rcl_interfaces__msg__ListParametersResult__Sequence__copy(in_seq, out_seq as *mut _)
        }
    }
}

impl rosidl_runtime_rs::Message for ListParametersResult {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for ListParametersResult
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rcl_interfaces/msg/ListParametersResult";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ListParametersResult()
        }
    }
}

#[link(name = "rcl_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__Log(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rcl_interfaces__rosidl_generator_c")]
extern "C" {
    fn rcl_interfaces__msg__Log__init(msg: *mut Log) -> bool;
    fn rcl_interfaces__msg__Log__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<Log>,
        size: usize,
    ) -> bool;
    fn rcl_interfaces__msg__Log__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Log>);
    fn rcl_interfaces__msg__Log__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<Log>,
        out_seq: *mut rosidl_runtime_rs::Sequence<Log>,
    ) -> bool;
}

// Corresponds to rcl_interfaces__msg__Log
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Severity level constants
///
/// These logging levels follow the Python Standard
/// https://docs.python.org/3/library/logging.html#logging-levels
/// And are implemented in rcutils as well
/// https://github.com/ros2/rcutils/blob/35f29850064e0c33a4063cbc947ebbfeada11dba/include/rcutils/logging.h#L164-L172
/// This leaves space for other standard logging levels to be inserted in the middle in the future,
/// as well as custom user defined levels.
/// Since there are several other logging enumeration standard for different implementations,
/// other logging implementations may need to provide level mappings to match their internal implementations.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Log {
    /// Fields
    ///
    /// Timestamp when this message was generated by the node.
    pub stamp: builtin_interfaces::msg::rmw::Time,

    /// Corresponding log level, see above definitions.
    pub level: u8,

    /// The name representing the logger this message came from.
    pub name: rosidl_runtime_rs::String,

    /// The full log message.
    pub msg: rosidl_runtime_rs::String,

    /// The file the message came from.
    pub file: rosidl_runtime_rs::String,

    /// The function the message came from.
    pub function: rosidl_runtime_rs::String,

    /// The line in the file the message came from.
    pub line: u32,
}

impl Log {
    /// Debug is for pedantic information, which is useful when debugging issues.
    pub const DEBUG: u8 = 10;

    /// Info is the standard informational level and is used to report expected
    /// information.
    pub const INFO: u8 = 20;

    /// Warning is for information that may potentially cause issues or possibly unexpected
    /// behavior.
    pub const WARN: u8 = 30;

    /// Error is for information that this node cannot resolve.
    pub const ERROR: u8 = 40;

    /// Information about a impending node shutdown.
    pub const FATAL: u8 = 50;
}

impl Default for Log {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rcl_interfaces__msg__Log__init(&mut msg as *mut _) {
                panic!("Call to rcl_interfaces__msg__Log__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for Log {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__Log__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__Log__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__Log__Sequence__copy(in_seq, out_seq as *mut _) }
    }
}

impl rosidl_runtime_rs::Message for Log {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for Log
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rcl_interfaces/msg/Log";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__Log() }
    }
}

#[link(name = "rcl_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterDescriptor(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rcl_interfaces__rosidl_generator_c")]
extern "C" {
    fn rcl_interfaces__msg__ParameterDescriptor__init(msg: *mut ParameterDescriptor) -> bool;
    fn rcl_interfaces__msg__ParameterDescriptor__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<ParameterDescriptor>,
        size: usize,
    ) -> bool;
    fn rcl_interfaces__msg__ParameterDescriptor__Sequence__fini(
        seq: *mut rosidl_runtime_rs::Sequence<ParameterDescriptor>,
    );
    fn rcl_interfaces__msg__ParameterDescriptor__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<ParameterDescriptor>,
        out_seq: *mut rosidl_runtime_rs::Sequence<ParameterDescriptor>,
    ) -> bool;
}

// Corresponds to rcl_interfaces__msg__ParameterDescriptor
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This is the message to communicate a parameter's descriptor.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParameterDescriptor {
    /// The name of the parameter.
    pub name: rosidl_runtime_rs::String,

    /// Enum values are defined in the `ParameterType.msg` message.
    pub type_: u8,

    /// Description of the parameter, visible from introspection tools.
    pub description: rosidl_runtime_rs::String,

    /// Parameter constraints
    /// Plain English description of additional constraints which cannot be expressed
    /// with the available constraints, e.g. "only prime numbers".
    ///
    /// By convention, this should only be used to clarify constraints which cannot
    /// be completely expressed with the parameter constraints below.
    pub additional_constraints: rosidl_runtime_rs::String,

    /// If 'true' then the value cannot change after it has been initialized.
    pub read_only: bool,

    /// If true, the parameter is allowed to change type.
    pub dynamic_typing: bool,

    /// If any of the following sequences are not empty, then the constraint inside of
    /// them apply to this parameter.
    ///
    /// FloatingPointRange and IntegerRange are mutually exclusive.
    /// FloatingPointRange consists of a from_value, a to_value, and a step.
    pub floating_point_range:
        rosidl_runtime_rs::BoundedSequence<super::super::msg::rmw::FloatingPointRange, 1>,

    /// IntegerRange consists of a from_value, a to_value, and a step.
    pub integer_range: rosidl_runtime_rs::BoundedSequence<super::super::msg::rmw::IntegerRange, 1>,
}

impl Default for ParameterDescriptor {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rcl_interfaces__msg__ParameterDescriptor__init(&mut msg as *mut _) {
                panic!("Call to rcl_interfaces__msg__ParameterDescriptor__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for ParameterDescriptor {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__ParameterDescriptor__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__ParameterDescriptor__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe {
            rcl_interfaces__msg__ParameterDescriptor__Sequence__copy(in_seq, out_seq as *mut _)
        }
    }
}

impl rosidl_runtime_rs::Message for ParameterDescriptor {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for ParameterDescriptor
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rcl_interfaces/msg/ParameterDescriptor";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterDescriptor()
        }
    }
}

#[link(name = "rcl_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterEventDescriptors(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rcl_interfaces__rosidl_generator_c")]
extern "C" {
    fn rcl_interfaces__msg__ParameterEventDescriptors__init(
        msg: *mut ParameterEventDescriptors,
    ) -> bool;
    fn rcl_interfaces__msg__ParameterEventDescriptors__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<ParameterEventDescriptors>,
        size: usize,
    ) -> bool;
    fn rcl_interfaces__msg__ParameterEventDescriptors__Sequence__fini(
        seq: *mut rosidl_runtime_rs::Sequence<ParameterEventDescriptors>,
    );
    fn rcl_interfaces__msg__ParameterEventDescriptors__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<ParameterEventDescriptors>,
        out_seq: *mut rosidl_runtime_rs::Sequence<ParameterEventDescriptors>,
    ) -> bool;
}

// Corresponds to rcl_interfaces__msg__ParameterEventDescriptors
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message contains descriptors of a parameter event.
/// It was an atomic update.
/// A specific parameter name can only be in one of the three sets.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParameterEventDescriptors {
    // This member is not documented.
    #[allow(missing_docs)]
    pub new_parameters: rosidl_runtime_rs::Sequence<super::super::msg::rmw::ParameterDescriptor>,

    // This member is not documented.
    #[allow(missing_docs)]
    pub changed_parameters:
        rosidl_runtime_rs::Sequence<super::super::msg::rmw::ParameterDescriptor>,

    // This member is not documented.
    #[allow(missing_docs)]
    pub deleted_parameters:
        rosidl_runtime_rs::Sequence<super::super::msg::rmw::ParameterDescriptor>,
}

impl Default for ParameterEventDescriptors {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rcl_interfaces__msg__ParameterEventDescriptors__init(&mut msg as *mut _) {
                panic!("Call to rcl_interfaces__msg__ParameterEventDescriptors__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for ParameterEventDescriptors {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe {
            rcl_interfaces__msg__ParameterEventDescriptors__Sequence__init(seq as *mut _, size)
        }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__ParameterEventDescriptors__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe {
            rcl_interfaces__msg__ParameterEventDescriptors__Sequence__copy(
                in_seq,
                out_seq as *mut _,
            )
        }
    }
}

impl rosidl_runtime_rs::Message for ParameterEventDescriptors {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for ParameterEventDescriptors
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rcl_interfaces/msg/ParameterEventDescriptors";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterEventDescriptors()
        }
    }
}

#[link(name = "rcl_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterEvent(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rcl_interfaces__rosidl_generator_c")]
extern "C" {
    fn rcl_interfaces__msg__ParameterEvent__init(msg: *mut ParameterEvent) -> bool;
    fn rcl_interfaces__msg__ParameterEvent__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<ParameterEvent>,
        size: usize,
    ) -> bool;
    fn rcl_interfaces__msg__ParameterEvent__Sequence__fini(
        seq: *mut rosidl_runtime_rs::Sequence<ParameterEvent>,
    );
    fn rcl_interfaces__msg__ParameterEvent__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<ParameterEvent>,
        out_seq: *mut rosidl_runtime_rs::Sequence<ParameterEvent>,
    ) -> bool;
}

// Corresponds to rcl_interfaces__msg__ParameterEvent
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This message contains a parameter event.
/// Because the parameter event was an atomic update, a specific parameter name
/// can only be in one of the three sets.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParameterEvent {
    /// The time stamp when this parameter event occurred.
    pub stamp: builtin_interfaces::msg::rmw::Time,

    /// Fully qualified ROS path to node.
    pub node: rosidl_runtime_rs::String,

    /// New parameters that have been set for this node.
    pub new_parameters: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Parameter>,

    /// Parameters that have been changed during this event.
    pub changed_parameters: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Parameter>,

    /// Parameters that have been deleted during this event.
    pub deleted_parameters: rosidl_runtime_rs::Sequence<super::super::msg::rmw::Parameter>,
}

impl Default for ParameterEvent {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rcl_interfaces__msg__ParameterEvent__init(&mut msg as *mut _) {
                panic!("Call to rcl_interfaces__msg__ParameterEvent__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for ParameterEvent {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__ParameterEvent__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__ParameterEvent__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__ParameterEvent__Sequence__copy(in_seq, out_seq as *mut _) }
    }
}

impl rosidl_runtime_rs::Message for ParameterEvent {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for ParameterEvent
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rcl_interfaces/msg/ParameterEvent";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterEvent()
        }
    }
}

#[link(name = "rcl_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__Parameter(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rcl_interfaces__rosidl_generator_c")]
extern "C" {
    fn rcl_interfaces__msg__Parameter__init(msg: *mut Parameter) -> bool;
    fn rcl_interfaces__msg__Parameter__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<Parameter>,
        size: usize,
    ) -> bool;
    fn rcl_interfaces__msg__Parameter__Sequence__fini(
        seq: *mut rosidl_runtime_rs::Sequence<Parameter>,
    );
    fn rcl_interfaces__msg__Parameter__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<Parameter>,
        out_seq: *mut rosidl_runtime_rs::Sequence<Parameter>,
    ) -> bool;
}

// Corresponds to rcl_interfaces__msg__Parameter
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// This is the message to communicate a parameter. It is an open struct with an enum in
/// the descriptor to select which value is active.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Parameter {
    /// The full name of the parameter.
    pub name: rosidl_runtime_rs::String,

    /// The parameter's value which can be one of several types, see
    /// `ParameterValue.msg` and `ParameterType.msg`.
    pub value: super::super::msg::rmw::ParameterValue,
}

impl Default for Parameter {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rcl_interfaces__msg__Parameter__init(&mut msg as *mut _) {
                panic!("Call to rcl_interfaces__msg__Parameter__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for Parameter {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__Parameter__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__Parameter__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__Parameter__Sequence__copy(in_seq, out_seq as *mut _) }
    }
}

impl rosidl_runtime_rs::Message for Parameter {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for Parameter
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rcl_interfaces/msg/Parameter";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__Parameter()
        }
    }
}

#[link(name = "rcl_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterType(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rcl_interfaces__rosidl_generator_c")]
extern "C" {
    fn rcl_interfaces__msg__ParameterType__init(msg: *mut ParameterType) -> bool;
    fn rcl_interfaces__msg__ParameterType__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<ParameterType>,
        size: usize,
    ) -> bool;
    fn rcl_interfaces__msg__ParameterType__Sequence__fini(
        seq: *mut rosidl_runtime_rs::Sequence<ParameterType>,
    );
    fn rcl_interfaces__msg__ParameterType__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<ParameterType>,
        out_seq: *mut rosidl_runtime_rs::Sequence<ParameterType>,
    ) -> bool;
}

// Corresponds to rcl_interfaces__msg__ParameterType
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// These types correspond to the value that is set in the ParameterValue message.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParameterType {
    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,
}

impl ParameterType {
    /// Default value, which implies this is not a valid parameter.
    pub const PARAMETER_NOT_SET: u8 = 0;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PARAMETER_BOOL: u8 = 1;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PARAMETER_INTEGER: u8 = 2;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PARAMETER_DOUBLE: u8 = 3;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PARAMETER_STRING: u8 = 4;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PARAMETER_BYTE_ARRAY: u8 = 5;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PARAMETER_BOOL_ARRAY: u8 = 6;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PARAMETER_INTEGER_ARRAY: u8 = 7;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PARAMETER_DOUBLE_ARRAY: u8 = 8;

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PARAMETER_STRING_ARRAY: u8 = 9;
}

impl Default for ParameterType {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rcl_interfaces__msg__ParameterType__init(&mut msg as *mut _) {
                panic!("Call to rcl_interfaces__msg__ParameterType__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for ParameterType {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__ParameterType__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__ParameterType__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__ParameterType__Sequence__copy(in_seq, out_seq as *mut _) }
    }
}

impl rosidl_runtime_rs::Message for ParameterType {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for ParameterType
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rcl_interfaces/msg/ParameterType";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterType()
        }
    }
}

#[link(name = "rcl_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterValue(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rcl_interfaces__rosidl_generator_c")]
extern "C" {
    fn rcl_interfaces__msg__ParameterValue__init(msg: *mut ParameterValue) -> bool;
    fn rcl_interfaces__msg__ParameterValue__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<ParameterValue>,
        size: usize,
    ) -> bool;
    fn rcl_interfaces__msg__ParameterValue__Sequence__fini(
        seq: *mut rosidl_runtime_rs::Sequence<ParameterValue>,
    );
    fn rcl_interfaces__msg__ParameterValue__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<ParameterValue>,
        out_seq: *mut rosidl_runtime_rs::Sequence<ParameterValue>,
    ) -> bool;
}

// Corresponds to rcl_interfaces__msg__ParameterValue
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Used to determine which of the next *_value fields are set.
/// ParameterType.PARAMETER_NOT_SET indicates that the parameter was not set
/// (if gotten) or is uninitialized.
/// Values are enumerated in `ParameterType.msg`.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParameterValue {
    /// The type of this parameter, which corresponds to the appropriate field below.
    pub type_: u8,

    /// "Variant" style storage of the parameter value. Only the value corresponding
    /// the type field will have valid information.
    /// Boolean value, can be either true or false.
    pub bool_value: bool,

    /// Integer value ranging from -9,223,372,036,854,775,808 to
    /// 9,223,372,036,854,775,807.
    pub integer_value: i64,

    /// A double precision floating point value following IEEE 754.
    pub double_value: f64,

    /// A textual value with no practical length limit.
    pub string_value: rosidl_runtime_rs::String,

    /// An array of bytes, used for non-textual information.
    pub byte_array_value: rosidl_runtime_rs::Sequence<u8>,

    /// An array of boolean values.
    pub bool_array_value: rosidl_runtime_rs::Sequence<bool>,

    /// An array of 64-bit integer values.
    pub integer_array_value: rosidl_runtime_rs::Sequence<i64>,

    /// An array of 64-bit floating point values.
    pub double_array_value: rosidl_runtime_rs::Sequence<f64>,

    /// An array of string values.
    pub string_array_value: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,
}

impl Default for ParameterValue {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rcl_interfaces__msg__ParameterValue__init(&mut msg as *mut _) {
                panic!("Call to rcl_interfaces__msg__ParameterValue__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for ParameterValue {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__ParameterValue__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__ParameterValue__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__ParameterValue__Sequence__copy(in_seq, out_seq as *mut _) }
    }
}

impl rosidl_runtime_rs::Message for ParameterValue {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for ParameterValue
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rcl_interfaces/msg/ParameterValue";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__ParameterValue()
        }
    }
}

#[link(name = "rcl_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__SetParametersResult(
    ) -> *const std::ffi::c_void;
}

#[link(name = "rcl_interfaces__rosidl_generator_c")]
extern "C" {
    fn rcl_interfaces__msg__SetParametersResult__init(msg: *mut SetParametersResult) -> bool;
    fn rcl_interfaces__msg__SetParametersResult__Sequence__init(
        seq: *mut rosidl_runtime_rs::Sequence<SetParametersResult>,
        size: usize,
    ) -> bool;
    fn rcl_interfaces__msg__SetParametersResult__Sequence__fini(
        seq: *mut rosidl_runtime_rs::Sequence<SetParametersResult>,
    );
    fn rcl_interfaces__msg__SetParametersResult__Sequence__copy(
        in_seq: &rosidl_runtime_rs::Sequence<SetParametersResult>,
        out_seq: *mut rosidl_runtime_rs::Sequence<SetParametersResult>,
    ) -> bool;
}

// Corresponds to rcl_interfaces__msg__SetParametersResult
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// A true value of the same index indicates that the parameter was set
/// successfully. A false value indicates the change was rejected.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetParametersResult {
    // This member is not documented.
    #[allow(missing_docs)]
    pub successful: bool,

    /// Reason why the setting was either successful or a failure. This should only be
    /// used for logging and user interfaces.
    pub reason: rosidl_runtime_rs::String,
}

impl Default for SetParametersResult {
    fn default() -> Self {
        unsafe {
            let mut msg = std::mem::zeroed();
            if !rcl_interfaces__msg__SetParametersResult__init(&mut msg as *mut _) {
                panic!("Call to rcl_interfaces__msg__SetParametersResult__init() failed");
            }
            msg
        }
    }
}

impl rosidl_runtime_rs::SequenceAlloc for SetParametersResult {
    fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__SetParametersResult__Sequence__init(seq as *mut _, size) }
    }
    fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe { rcl_interfaces__msg__SetParametersResult__Sequence__fini(seq as *mut _) }
    }
    fn sequence_copy(
        in_seq: &rosidl_runtime_rs::Sequence<Self>,
        out_seq: &mut rosidl_runtime_rs::Sequence<Self>,
    ) -> bool {
        // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
        unsafe {
            rcl_interfaces__msg__SetParametersResult__Sequence__copy(in_seq, out_seq as *mut _)
        }
    }
}

impl rosidl_runtime_rs::Message for SetParametersResult {
    type RmwMsg = Self;
    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        msg_cow
    }
    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        msg
    }
}

impl rosidl_runtime_rs::RmwMessage for SetParametersResult
where
    Self: Sized,
{
    const TYPE_NAME: &'static str = "rcl_interfaces/msg/SetParametersResult";
    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__rcl_interfaces__msg__SetParametersResult()
        }
    }
}
