// Copied verbatim from the generated Rust code for `rcl_interfaces` 1.2.3,
// built from the `humble` distro. Do not edit. Run `vendor_interfaces.py`
// to refresh it.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Corresponds to rcl_interfaces__msg__FloatingPointRange
/// Represents bounds and a step value for a floating point typed parameter.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::msg::rmw::FloatingPointRange::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for FloatingPointRange {
    type RmwMsg = super::msg::rmw::FloatingPointRange;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                from_value: msg.from_value,
                to_value: msg.to_value,
                step: msg.step,
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                from_value: msg.from_value,
                to_value: msg.to_value,
                step: msg.step,
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            from_value: msg.from_value,
            to_value: msg.to_value,
            step: msg.step,
        }
    }
}

// Corresponds to rcl_interfaces__msg__IntegerRange
/// Represents bounds and a step value for an integer typed parameter.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::msg::rmw::IntegerRange::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for IntegerRange {
    type RmwMsg = super::msg::rmw::IntegerRange;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                from_value: msg.from_value,
                to_value: msg.to_value,
                step: msg.step,
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                from_value: msg.from_value,
                to_value: msg.to_value,
                step: msg.step,
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            from_value: msg.from_value,
            to_value: msg.to_value,
            step: msg.step,
        }
    }
}

// Corresponds to rcl_interfaces__msg__ListParametersResult
/// The resulting parameters under the given prefixes.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListParametersResult {
    // This member is not documented.
    #[allow(missing_docs)]
    pub names: Vec<std::string::String>,

    /// The resulting prefixes under the given prefixes.
    /// TODO(wjwwood): link to prefix definition and rules.
    pub prefixes: Vec<std::string::String>,
}

impl Default for ListParametersResult {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::msg::rmw::ListParametersResult::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for ListParametersResult {
    type RmwMsg = super::msg::rmw::ListParametersResult;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                names: msg
                    .names
                    .into_iter()
                    .map(|elem| elem.as_str().into())
                    .collect(),
                prefixes: msg
                    .prefixes
                    .into_iter()
                    .map(|elem| elem.as_str().into())
                    .collect(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                names: msg.names.iter().map(|elem| elem.as_str().into()).collect(),
                prefixes: msg
                    .prefixes
                    .iter()
                    .map(|elem| elem.as_str().into())
                    .collect(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            names: msg.names.into_iter().map(|elem| elem.to_string()).collect(),
            prefixes: msg
                .prefixes
                .into_iter()
                .map(|elem| elem.to_string())
                .collect(),
        }
    }
}

// Corresponds to rcl_interfaces__msg__Log
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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Log {
    /// Fields
    ///
    /// Timestamp when this message was generated by the node.
    pub stamp: builtin_interfaces::msg::Time,

    /// Corresponding log level, see above definitions.
    pub level: u8,

    /// The name representing the logger this message came from.
    pub name: std::string::String,

    /// The full log message.
    pub msg: std::string::String,

    /// The file the message came from.
    pub file: std::string::String,

    /// The function the message came from.
    pub function: std::string::String,

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
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Log::default())
    }
}

impl rosidl_runtime_rs::Message for Log {
    type RmwMsg = super::msg::rmw::Log;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(
                    msg.stamp,
                ))
                .into_owned(),
                level: msg.level,
                name: msg.name.as_str().into(),
                msg: msg.msg.as_str().into(),
                file: msg.file.as_str().into(),
                function: msg.function.as_str().into(),
                line: msg.line,
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(
                    &msg.stamp,
                ))
                .into_owned(),
                level: msg.level,
                name: msg.name.as_str().into(),
                msg: msg.msg.as_str().into(),
                file: msg.file.as_str().into(),
                function: msg.function.as_str().into(),
                line: msg.line,
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
            level: msg.level,
            name: msg.name.to_string(),
            msg: msg.msg.to_string(),
            file: msg.file.to_string(),
            function: msg.function.to_string(),
            line: msg.line,
        }
    }
}

// Corresponds to rcl_interfaces__msg__ParameterDescriptor
/// This is the message to communicate a parameter's descriptor.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParameterDescriptor {
    /// The name of the parameter.
    pub name: std::string::String,

    /// Enum values are defined in the `ParameterType.msg` message.
    pub type_: u8,

    /// Description of the parameter, visible from introspection tools.
    pub description: std::string::String,

    /// Parameter constraints
    /// Plain English description of additional constraints which cannot be expressed
    /// with the available constraints, e.g. "only prime numbers".
    ///
    /// By convention, this should only be used to clarify constraints which cannot
    /// be completely expressed with the parameter constraints below.
    pub additional_constraints: std::string::String,

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
        rosidl_runtime_rs::BoundedSequence<super::msg::rmw::FloatingPointRange, 1>,

    /// IntegerRange consists of a from_value, a to_value, and a step.
    pub integer_range: rosidl_runtime_rs::BoundedSequence<super::msg::rmw::IntegerRange, 1>,
}

impl Default for ParameterDescriptor {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::msg::rmw::ParameterDescriptor::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for ParameterDescriptor {
    type RmwMsg = super::msg::rmw::ParameterDescriptor;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                name: msg.name.as_str().into(),
                type_: msg.type_,
                description: msg.description.as_str().into(),
                additional_constraints: msg.additional_constraints.as_str().into(),
                read_only: msg.read_only,
                dynamic_typing: msg.dynamic_typing,
                floating_point_range: msg.floating_point_range,
                integer_range: msg.integer_range,
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                name: msg.name.as_str().into(),
                type_: msg.type_,
                description: msg.description.as_str().into(),
                additional_constraints: msg.additional_constraints.as_str().into(),
                read_only: msg.read_only,
                dynamic_typing: msg.dynamic_typing,
                floating_point_range: msg.floating_point_range.clone(),
                integer_range: msg.integer_range.clone(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            name: msg.name.to_string(),
            type_: msg.type_,
            description: msg.description.to_string(),
            additional_constraints: msg.additional_constraints.to_string(),
            read_only: msg.read_only,
            dynamic_typing: msg.dynamic_typing,
            floating_point_range: msg.floating_point_range,
            integer_range: msg.integer_range,
        }
    }
}

// Corresponds to rcl_interfaces__msg__ParameterEventDescriptors
/// This message contains descriptors of a parameter event.
/// It was an atomic update.
/// A specific parameter name can only be in one of the three sets.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParameterEventDescriptors {
    // This member is not documented.
    #[allow(missing_docs)]
    pub new_parameters: Vec<super::msg::ParameterDescriptor>,

    // This member is not documented.
    #[allow(missing_docs)]
    pub changed_parameters: Vec<super::msg::ParameterDescriptor>,

    // This member is not documented.
    #[allow(missing_docs)]
    pub deleted_parameters: Vec<super::msg::ParameterDescriptor>,
}

impl Default for ParameterEventDescriptors {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::msg::rmw::ParameterEventDescriptors::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for ParameterEventDescriptors {
    type RmwMsg = super::msg::rmw::ParameterEventDescriptors;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                new_parameters: msg
                    .new_parameters
                    .into_iter()
                    .map(|elem| {
                        super::msg::ParameterDescriptor::into_rmw_message(std::borrow::Cow::Owned(
                            elem,
                        ))
                        .into_owned()
                    })
                    .collect(),
                changed_parameters: msg
                    .changed_parameters
                    .into_iter()
                    .map(|elem| {
                        super::msg::ParameterDescriptor::into_rmw_message(std::borrow::Cow::Owned(
                            elem,
                        ))
                        .into_owned()
                    })
                    .collect(),
                deleted_parameters: msg
                    .deleted_parameters
                    .into_iter()
                    .map(|elem| {
                        super::msg::ParameterDescriptor::into_rmw_message(std::borrow::Cow::Owned(
                            elem,
                        ))
                        .into_owned()
                    })
                    .collect(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                new_parameters: msg
                    .new_parameters
                    .iter()
                    .map(|elem| {
                        super::msg::ParameterDescriptor::into_rmw_message(
                            std::borrow::Cow::Borrowed(elem),
                        )
                        .into_owned()
                    })
                    .collect(),
                changed_parameters: msg
                    .changed_parameters
                    .iter()
                    .map(|elem| {
                        super::msg::ParameterDescriptor::into_rmw_message(
                            std::borrow::Cow::Borrowed(elem),
                        )
                        .into_owned()
                    })
                    .collect(),
                deleted_parameters: msg
                    .deleted_parameters
                    .iter()
                    .map(|elem| {
                        super::msg::ParameterDescriptor::into_rmw_message(
                            std::borrow::Cow::Borrowed(elem),
                        )
                        .into_owned()
                    })
                    .collect(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            new_parameters: msg
                .new_parameters
                .into_iter()
                .map(super::msg::ParameterDescriptor::from_rmw_message)
                .collect(),
            changed_parameters: msg
                .changed_parameters
                .into_iter()
                .map(super::msg::ParameterDescriptor::from_rmw_message)
                .collect(),
            deleted_parameters: msg
                .deleted_parameters
                .into_iter()
                .map(super::msg::ParameterDescriptor::from_rmw_message)
                .collect(),
        }
    }
}

// Corresponds to rcl_interfaces__msg__ParameterEvent
/// This message contains a parameter event.
/// Because the parameter event was an atomic update, a specific parameter name
/// can only be in one of the three sets.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ParameterEvent {
    /// The time stamp when this parameter event occurred.
    pub stamp: builtin_interfaces::msg::Time,

    /// Fully qualified ROS path to node.
    pub node: std::string::String,

    /// New parameters that have been set for this node.
    pub new_parameters: Vec<super::msg::Parameter>,

    /// Parameters that have been changed during this event.
    pub changed_parameters: Vec<super::msg::Parameter>,

    /// Parameters that have been deleted during this event.
    pub deleted_parameters: Vec<super::msg::Parameter>,
}

impl Default for ParameterEvent {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::msg::rmw::ParameterEvent::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for ParameterEvent {
    type RmwMsg = super::msg::rmw::ParameterEvent;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(
                    msg.stamp,
                ))
                .into_owned(),
                node: msg.node.as_str().into(),
                new_parameters: msg
                    .new_parameters
                    .into_iter()
                    .map(|elem| {
                        super::msg::Parameter::into_rmw_message(std::borrow::Cow::Owned(elem))
                            .into_owned()
                    })
                    .collect(),
                changed_parameters: msg
                    .changed_parameters
                    .into_iter()
                    .map(|elem| {
                        super::msg::Parameter::into_rmw_message(std::borrow::Cow::Owned(elem))
                            .into_owned()
                    })
                    .collect(),
                deleted_parameters: msg
                    .deleted_parameters
                    .into_iter()
                    .map(|elem| {
                        super::msg::Parameter::into_rmw_message(std::borrow::Cow::Owned(elem))
                            .into_owned()
                    })
                    .collect(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(
                    &msg.stamp,
                ))
                .into_owned(),
                node: msg.node.as_str().into(),
                new_parameters: msg
                    .new_parameters
                    .iter()
                    .map(|elem| {
                        super::msg::Parameter::into_rmw_message(std::borrow::Cow::Borrowed(elem))
                            .into_owned()
                    })
                    .collect(),
                changed_parameters: msg
                    .changed_parameters
                    .iter()
                    .map(|elem| {
                        super::msg::Parameter::into_rmw_message(std::borrow::Cow::Borrowed(elem))
                            .into_owned()
                    })
                    .collect(),
                deleted_parameters: msg
                    .deleted_parameters
                    .iter()
                    .map(|elem| {
                        super::msg::Parameter::into_rmw_message(std::borrow::Cow::Borrowed(elem))
                            .into_owned()
                    })
                    .collect(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
            node: msg.node.to_string(),
            new_parameters: msg
                .new_parameters
                .into_iter()
                .map(super::msg::Parameter::from_rmw_message)
                .collect(),
            changed_parameters: msg
                .changed_parameters
                .into_iter()
                .map(super::msg::Parameter::from_rmw_message)
                .collect(),
            deleted_parameters: msg
                .deleted_parameters
                .into_iter()
                .map(super::msg::Parameter::from_rmw_message)
                .collect(),
        }
    }
}

// Corresponds to rcl_interfaces__msg__Parameter
/// This is the message to communicate a parameter. It is an open struct with an enum in
/// the descriptor to select which value is active.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Parameter {
    /// The full name of the parameter.
    pub name: std::string::String,

    /// The parameter's value which can be one of several types, see
    /// `ParameterValue.msg` and `ParameterType.msg`.
    pub value: super::msg::ParameterValue,
}

impl Default for Parameter {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Parameter::default())
    }
}

impl rosidl_runtime_rs::Message for Parameter {
    type RmwMsg = super::msg::rmw::Parameter;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                name: msg.name.as_str().into(),
                value: super::msg::ParameterValue::into_rmw_message(std::borrow::Cow::Owned(
                    msg.value,
                ))
                .into_owned(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                name: msg.name.as_str().into(),
                value: super::msg::ParameterValue::into_rmw_message(std::borrow::Cow::Borrowed(
                    &msg.value,
                ))
                .into_owned(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            name: msg.name.to_string(),
            value: super::msg::ParameterValue::from_rmw_message(msg.value),
        }
    }
}

// Corresponds to rcl_interfaces__msg__ParameterType
/// These types correspond to the value that is set in the ParameterValue message.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::msg::rmw::ParameterType::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for ParameterType {
    type RmwMsg = super::msg::rmw::ParameterType;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
        }
    }
}

// Corresponds to rcl_interfaces__msg__ParameterValue
/// Used to determine which of the next *_value fields are set.
/// ParameterType.PARAMETER_NOT_SET indicates that the parameter was not set
/// (if gotten) or is uninitialized.
/// Values are enumerated in `ParameterType.msg`.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub string_value: std::string::String,

    /// An array of bytes, used for non-textual information.
    pub byte_array_value: Vec<u8>,

    /// An array of boolean values.
    pub bool_array_value: Vec<bool>,

    /// An array of 64-bit integer values.
    pub integer_array_value: Vec<i64>,

    /// An array of 64-bit floating point values.
    pub double_array_value: Vec<f64>,

    /// An array of string values.
    pub string_array_value: Vec<std::string::String>,
}

impl Default for ParameterValue {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::msg::rmw::ParameterValue::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for ParameterValue {
    type RmwMsg = super::msg::rmw::ParameterValue;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                type_: msg.type_,
                bool_value: msg.bool_value,
                integer_value: msg.integer_value,
                double_value: msg.double_value,
                string_value: msg.string_value.as_str().into(),
                byte_array_value: msg.byte_array_value.as_slice().into(),
                bool_array_value: msg.bool_array_value.as_slice().into(),
                integer_array_value: msg.integer_array_value.as_slice().into(),
                double_array_value: msg.double_array_value.as_slice().into(),
                string_array_value: msg
                    .string_array_value
                    .into_iter()
                    .map(|elem| elem.as_str().into())
                    .collect(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                type_: msg.type_,
                bool_value: msg.bool_value,
                integer_value: msg.integer_value,
                double_value: msg.double_value,
                string_value: msg.string_value.as_str().into(),
                byte_array_value: msg.byte_array_value.as_slice().into(),
                bool_array_value: msg.bool_array_value.as_slice().into(),
                integer_array_value: msg.integer_array_value.as_slice().into(),
                double_array_value: msg.double_array_value.as_slice().into(),
                string_array_value: msg
                    .string_array_value
                    .iter()
                    .map(|elem| elem.as_str().into())
                    .collect(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            type_: msg.type_,
            bool_value: msg.bool_value,
            integer_value: msg.integer_value,
            double_value: msg.double_value,
            string_value: msg.string_value.to_string(),
            byte_array_value: msg.byte_array_value.into(),
            bool_array_value: msg.bool_array_value.into(),
            integer_array_value: msg.integer_array_value.into(),
            double_array_value: msg.double_array_value.into(),
            string_array_value: msg
                .string_array_value
                .into_iter()
                .map(|elem| elem.to_string())
                .collect(),
        }
    }
}

// Corresponds to rcl_interfaces__msg__SetParametersResult
/// A true value of the same index indicates that the parameter was set
/// successfully. A false value indicates the change was rejected.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetParametersResult {
    // This member is not documented.
    #[allow(missing_docs)]
    pub successful: bool,

    /// Reason why the setting was either successful or a failure. This should only be
    /// used for logging and user interfaces.
    pub reason: std::string::String,
}

impl Default for SetParametersResult {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::msg::rmw::SetParametersResult::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for SetParametersResult {
    type RmwMsg = super::msg::rmw::SetParametersResult;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                successful: msg.successful,
                reason: msg.reason.as_str().into(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                successful: msg.successful,
                reason: msg.reason.as_str().into(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            successful: msg.successful,
            reason: msg.reason.to_string(),
        }
    }
}
