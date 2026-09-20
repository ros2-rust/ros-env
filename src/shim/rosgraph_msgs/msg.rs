// Copied verbatim from the generated Rust code for `rosgraph_msgs` 1.2.3,
// built from the `humble` distro. Do not edit. Run `vendor_interfaces.py`
// to refresh it.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Corresponds to rosgraph_msgs__msg__Action
/// Describes a single Action endpoint, which may be a Server or Client

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Action {
    /// Fully qualified name of the Action
    pub name: std::string::String,

    /// An action is actually a composition of the following fundamental ROS entities
    pub send_goal: super::msg::Service,

    // This member is not documented.
    #[allow(missing_docs)]
    pub get_result: super::msg::Service,

    // This member is not documented.
    #[allow(missing_docs)]
    pub cancel_goal: super::msg::Service,

    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::msg::Topic,

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: super::msg::Topic,
}

impl Default for Action {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Action::default())
    }
}

impl rosidl_runtime_rs::Message for Action {
    type RmwMsg = super::msg::rmw::Action;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                name: msg.name.as_str().into(),
                send_goal: super::msg::Service::into_rmw_message(std::borrow::Cow::Owned(
                    msg.send_goal,
                ))
                .into_owned(),
                get_result: super::msg::Service::into_rmw_message(std::borrow::Cow::Owned(
                    msg.get_result,
                ))
                .into_owned(),
                cancel_goal: super::msg::Service::into_rmw_message(std::borrow::Cow::Owned(
                    msg.cancel_goal,
                ))
                .into_owned(),
                feedback: super::msg::Topic::into_rmw_message(std::borrow::Cow::Owned(
                    msg.feedback,
                ))
                .into_owned(),
                status: super::msg::Topic::into_rmw_message(std::borrow::Cow::Owned(msg.status))
                    .into_owned(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                name: msg.name.as_str().into(),
                send_goal: super::msg::Service::into_rmw_message(std::borrow::Cow::Borrowed(
                    &msg.send_goal,
                ))
                .into_owned(),
                get_result: super::msg::Service::into_rmw_message(std::borrow::Cow::Borrowed(
                    &msg.get_result,
                ))
                .into_owned(),
                cancel_goal: super::msg::Service::into_rmw_message(std::borrow::Cow::Borrowed(
                    &msg.cancel_goal,
                ))
                .into_owned(),
                feedback: super::msg::Topic::into_rmw_message(std::borrow::Cow::Borrowed(
                    &msg.feedback,
                ))
                .into_owned(),
                status: super::msg::Topic::into_rmw_message(std::borrow::Cow::Borrowed(
                    &msg.status,
                ))
                .into_owned(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            name: msg.name.to_string(),
            send_goal: super::msg::Service::from_rmw_message(msg.send_goal),
            get_result: super::msg::Service::from_rmw_message(msg.get_result),
            cancel_goal: super::msg::Service::from_rmw_message(msg.cancel_goal),
            feedback: super::msg::Topic::from_rmw_message(msg.feedback),
            status: super::msg::Topic::from_rmw_message(msg.status),
        }
    }
}

// Corresponds to rosgraph_msgs__msg__Clock
/// This message communicates the current time.
///
/// For more information, see https://design.ros2.org/articles/clock_and_time.html.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Clock {
    // This member is not documented.
    #[allow(missing_docs)]
    pub clock: builtin_interfaces::msg::Time,
}

impl Default for Clock {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Clock::default())
    }
}

impl rosidl_runtime_rs::Message for Clock {
    type RmwMsg = super::msg::rmw::Clock;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                clock: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(
                    msg.clock,
                ))
                .into_owned(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                clock: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(
                    &msg.clock,
                ))
                .into_owned(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            clock: builtin_interfaces::msg::Time::from_rmw_message(msg.clock),
        }
    }
}

// Corresponds to rosgraph_msgs__msg__Graph
/// Represents a ROS node graph, which is only a collection of nodes

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Graph {
    // This member is not documented.
    #[allow(missing_docs)]
    pub nodes: Vec<super::msg::Node>,
}

impl Default for Graph {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Graph::default())
    }
}

impl rosidl_runtime_rs::Message for Graph {
    type RmwMsg = super::msg::rmw::Graph;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                nodes: msg
                    .nodes
                    .into_iter()
                    .map(|elem| {
                        super::msg::Node::into_rmw_message(std::borrow::Cow::Owned(elem))
                            .into_owned()
                    })
                    .collect(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                nodes: msg
                    .nodes
                    .iter()
                    .map(|elem| {
                        super::msg::Node::into_rmw_message(std::borrow::Cow::Borrowed(elem))
                            .into_owned()
                    })
                    .collect(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            nodes: msg
                .nodes
                .into_iter()
                .map(super::msg::Node::from_rmw_message)
                .collect(),
        }
    }
}

// Corresponds to rosgraph_msgs__msg__InterfaceType
/// Represent a type of a ROS Graph Interface

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct InterfaceType {
    /// The plaintext namespaced name of the type - e.g. sensor_msgs/Image
    pub name: std::string::String,

    /// The hash uniquely identifies the exact structure of the type,
    /// the definition of which may change between package version
    pub hash: super::msg::TypeHash,
}

impl Default for InterfaceType {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::msg::rmw::InterfaceType::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for InterfaceType {
    type RmwMsg = super::msg::rmw::InterfaceType;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                name: msg.name.as_str().into(),
                hash: super::msg::TypeHash::into_rmw_message(std::borrow::Cow::Owned(msg.hash))
                    .into_owned(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                name: msg.name.as_str().into(),
                hash: super::msg::TypeHash::into_rmw_message(std::borrow::Cow::Borrowed(&msg.hash))
                    .into_owned(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            name: msg.name.to_string(),
            hash: super::msg::TypeHash::from_rmw_message(msg.hash),
        }
    }
}

// Corresponds to rosgraph_msgs__msg__Node
/// Represents the observable runtime state of a ROS Node
/// Therefore, does not perfectly align with the abstract specification which created it.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Node {
    /// Fully qualified node name (FQN)
    pub name: std::string::String,

    /// Parameter specifications for the node
    pub parameters: Vec<rcl_interfaces::msg::ParameterDescriptor>,

    /// Current values of the node's parameters
    /// NOTE:
    ///   parameter_values[] must be empty, or the same size as parameters[]
    ///   When set, parameter_values[] match 1:1 with the same index in parameters[]
    pub parameter_values: Vec<rcl_interfaces::msg::ParameterValue>,

    /// Communications endpoints - Topics, Services, and Actions
    pub publishers: Vec<super::msg::Topic>,

    // This member is not documented.
    #[allow(missing_docs)]
    pub subscriptions: Vec<super::msg::Topic>,

    // This member is not documented.
    #[allow(missing_docs)]
    pub service_clients: Vec<super::msg::Service>,

    // This member is not documented.
    #[allow(missing_docs)]
    pub service_servers: Vec<super::msg::Service>,

    // This member is not documented.
    #[allow(missing_docs)]
    pub action_clients: Vec<super::msg::Action>,

    // This member is not documented.
    #[allow(missing_docs)]
    pub action_servers: Vec<super::msg::Action>,
}

impl Default for Node {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Node::default())
    }
}

impl rosidl_runtime_rs::Message for Node {
    type RmwMsg = super::msg::rmw::Node;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                name: msg.name.as_str().into(),
                parameters: msg
                    .parameters
                    .into_iter()
                    .map(|elem| {
                        rcl_interfaces::msg::ParameterDescriptor::into_rmw_message(
                            std::borrow::Cow::Owned(elem),
                        )
                        .into_owned()
                    })
                    .collect(),
                parameter_values: msg
                    .parameter_values
                    .into_iter()
                    .map(|elem| {
                        rcl_interfaces::msg::ParameterValue::into_rmw_message(
                            std::borrow::Cow::Owned(elem),
                        )
                        .into_owned()
                    })
                    .collect(),
                publishers: msg
                    .publishers
                    .into_iter()
                    .map(|elem| {
                        super::msg::Topic::into_rmw_message(std::borrow::Cow::Owned(elem))
                            .into_owned()
                    })
                    .collect(),
                subscriptions: msg
                    .subscriptions
                    .into_iter()
                    .map(|elem| {
                        super::msg::Topic::into_rmw_message(std::borrow::Cow::Owned(elem))
                            .into_owned()
                    })
                    .collect(),
                service_clients: msg
                    .service_clients
                    .into_iter()
                    .map(|elem| {
                        super::msg::Service::into_rmw_message(std::borrow::Cow::Owned(elem))
                            .into_owned()
                    })
                    .collect(),
                service_servers: msg
                    .service_servers
                    .into_iter()
                    .map(|elem| {
                        super::msg::Service::into_rmw_message(std::borrow::Cow::Owned(elem))
                            .into_owned()
                    })
                    .collect(),
                action_clients: msg
                    .action_clients
                    .into_iter()
                    .map(|elem| {
                        super::msg::Action::into_rmw_message(std::borrow::Cow::Owned(elem))
                            .into_owned()
                    })
                    .collect(),
                action_servers: msg
                    .action_servers
                    .into_iter()
                    .map(|elem| {
                        super::msg::Action::into_rmw_message(std::borrow::Cow::Owned(elem))
                            .into_owned()
                    })
                    .collect(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                name: msg.name.as_str().into(),
                parameters: msg
                    .parameters
                    .iter()
                    .map(|elem| {
                        rcl_interfaces::msg::ParameterDescriptor::into_rmw_message(
                            std::borrow::Cow::Borrowed(elem),
                        )
                        .into_owned()
                    })
                    .collect(),
                parameter_values: msg
                    .parameter_values
                    .iter()
                    .map(|elem| {
                        rcl_interfaces::msg::ParameterValue::into_rmw_message(
                            std::borrow::Cow::Borrowed(elem),
                        )
                        .into_owned()
                    })
                    .collect(),
                publishers: msg
                    .publishers
                    .iter()
                    .map(|elem| {
                        super::msg::Topic::into_rmw_message(std::borrow::Cow::Borrowed(elem))
                            .into_owned()
                    })
                    .collect(),
                subscriptions: msg
                    .subscriptions
                    .iter()
                    .map(|elem| {
                        super::msg::Topic::into_rmw_message(std::borrow::Cow::Borrowed(elem))
                            .into_owned()
                    })
                    .collect(),
                service_clients: msg
                    .service_clients
                    .iter()
                    .map(|elem| {
                        super::msg::Service::into_rmw_message(std::borrow::Cow::Borrowed(elem))
                            .into_owned()
                    })
                    .collect(),
                service_servers: msg
                    .service_servers
                    .iter()
                    .map(|elem| {
                        super::msg::Service::into_rmw_message(std::borrow::Cow::Borrowed(elem))
                            .into_owned()
                    })
                    .collect(),
                action_clients: msg
                    .action_clients
                    .iter()
                    .map(|elem| {
                        super::msg::Action::into_rmw_message(std::borrow::Cow::Borrowed(elem))
                            .into_owned()
                    })
                    .collect(),
                action_servers: msg
                    .action_servers
                    .iter()
                    .map(|elem| {
                        super::msg::Action::into_rmw_message(std::borrow::Cow::Borrowed(elem))
                            .into_owned()
                    })
                    .collect(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            name: msg.name.to_string(),
            parameters: msg
                .parameters
                .into_iter()
                .map(rcl_interfaces::msg::ParameterDescriptor::from_rmw_message)
                .collect(),
            parameter_values: msg
                .parameter_values
                .into_iter()
                .map(rcl_interfaces::msg::ParameterValue::from_rmw_message)
                .collect(),
            publishers: msg
                .publishers
                .into_iter()
                .map(super::msg::Topic::from_rmw_message)
                .collect(),
            subscriptions: msg
                .subscriptions
                .into_iter()
                .map(super::msg::Topic::from_rmw_message)
                .collect(),
            service_clients: msg
                .service_clients
                .into_iter()
                .map(super::msg::Service::from_rmw_message)
                .collect(),
            service_servers: msg
                .service_servers
                .into_iter()
                .map(super::msg::Service::from_rmw_message)
                .collect(),
            action_clients: msg
                .action_clients
                .into_iter()
                .map(super::msg::Action::from_rmw_message)
                .collect(),
            action_servers: msg
                .action_servers
                .into_iter()
                .map(super::msg::Action::from_rmw_message)
                .collect(),
        }
    }
}

// Corresponds to rosgraph_msgs__msg__QoSProfile
/// Message-based representation of ROS 2 Quality of Service settings
/// Default values are kept in sync with RMW by integration test
/// Note that SYSTEM_DEFAULT and BEST_AVAILABLE values cannot be an observed value,
/// because they resolve concretely at runtime.
/// They are included here for completeness to match the data structures in RMW

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct QoSProfile {
    /// Depth of the message queue (only meaningful when history==KEEP_LAST)
    pub depth: u32,

    /// Deadline between messages (0 for no deadline)
    pub deadline: builtin_interfaces::msg::Duration,

    /// Lifespan of each message (0 for infinite)
    pub lifespan: builtin_interfaces::msg::Duration,

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
    pub liveliness_lease_duration: builtin_interfaces::msg::Duration,
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
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::msg::rmw::QoSProfile::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for QoSProfile {
    type RmwMsg = super::msg::rmw::QoSProfile;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                depth: msg.depth,
                deadline: builtin_interfaces::msg::Duration::into_rmw_message(
                    std::borrow::Cow::Owned(msg.deadline),
                )
                .into_owned(),
                lifespan: builtin_interfaces::msg::Duration::into_rmw_message(
                    std::borrow::Cow::Owned(msg.lifespan),
                )
                .into_owned(),
                history: msg.history,
                reliability: msg.reliability,
                durability: msg.durability,
                liveliness: msg.liveliness,
                liveliness_lease_duration: builtin_interfaces::msg::Duration::into_rmw_message(
                    std::borrow::Cow::Owned(msg.liveliness_lease_duration),
                )
                .into_owned(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                depth: msg.depth,
                deadline: builtin_interfaces::msg::Duration::into_rmw_message(
                    std::borrow::Cow::Borrowed(&msg.deadline),
                )
                .into_owned(),
                lifespan: builtin_interfaces::msg::Duration::into_rmw_message(
                    std::borrow::Cow::Borrowed(&msg.lifespan),
                )
                .into_owned(),
                history: msg.history,
                reliability: msg.reliability,
                durability: msg.durability,
                liveliness: msg.liveliness,
                liveliness_lease_duration: builtin_interfaces::msg::Duration::into_rmw_message(
                    std::borrow::Cow::Borrowed(&msg.liveliness_lease_duration),
                )
                .into_owned(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            depth: msg.depth,
            deadline: builtin_interfaces::msg::Duration::from_rmw_message(msg.deadline),
            lifespan: builtin_interfaces::msg::Duration::from_rmw_message(msg.lifespan),
            history: msg.history,
            reliability: msg.reliability,
            durability: msg.durability,
            liveliness: msg.liveliness,
            liveliness_lease_duration: builtin_interfaces::msg::Duration::from_rmw_message(
                msg.liveliness_lease_duration,
            ),
        }
    }
}

// Corresponds to rosgraph_msgs__msg__Service
/// Describes a single Service endpoint, which may be a Server or Client

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Service {
    /// Fully qualified name of the Service
    pub name: std::string::String,

    /// Type and actual QoS of the request publisher (Client) or subscription (Server)
    pub request_type: super::msg::InterfaceType,

    // This member is not documented.
    #[allow(missing_docs)]
    pub request_qos: super::msg::QoSProfile,

    /// Type and actual QoS of the request subscription (Client) or publisher (Server)
    pub response_type: super::msg::InterfaceType,

    // This member is not documented.
    #[allow(missing_docs)]
    pub response_qos: super::msg::QoSProfile,
}

impl Default for Service {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Service::default())
    }
}

impl rosidl_runtime_rs::Message for Service {
    type RmwMsg = super::msg::rmw::Service;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                name: msg.name.as_str().into(),
                request_type: super::msg::InterfaceType::into_rmw_message(std::borrow::Cow::Owned(
                    msg.request_type,
                ))
                .into_owned(),
                request_qos: super::msg::QoSProfile::into_rmw_message(std::borrow::Cow::Owned(
                    msg.request_qos,
                ))
                .into_owned(),
                response_type: super::msg::InterfaceType::into_rmw_message(
                    std::borrow::Cow::Owned(msg.response_type),
                )
                .into_owned(),
                response_qos: super::msg::QoSProfile::into_rmw_message(std::borrow::Cow::Owned(
                    msg.response_qos,
                ))
                .into_owned(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                name: msg.name.as_str().into(),
                request_type: super::msg::InterfaceType::into_rmw_message(
                    std::borrow::Cow::Borrowed(&msg.request_type),
                )
                .into_owned(),
                request_qos: super::msg::QoSProfile::into_rmw_message(std::borrow::Cow::Borrowed(
                    &msg.request_qos,
                ))
                .into_owned(),
                response_type: super::msg::InterfaceType::into_rmw_message(
                    std::borrow::Cow::Borrowed(&msg.response_type),
                )
                .into_owned(),
                response_qos: super::msg::QoSProfile::into_rmw_message(std::borrow::Cow::Borrowed(
                    &msg.response_qos,
                ))
                .into_owned(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            name: msg.name.to_string(),
            request_type: super::msg::InterfaceType::from_rmw_message(msg.request_type),
            request_qos: super::msg::QoSProfile::from_rmw_message(msg.request_qos),
            response_type: super::msg::InterfaceType::from_rmw_message(msg.response_type),
            response_qos: super::msg::QoSProfile::from_rmw_message(msg.response_qos),
        }
    }
}

// Corresponds to rosgraph_msgs__msg__Topic
/// Describes a single topic endpoint, which may be a Publisher or Subscription

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Topic {
    /// Fully qualified name of the topic
    pub name: std::string::String,

    /// Type of the topic
    pub type_: super::msg::InterfaceType,

    /// Observed QoS of the endpoint
    pub qos: super::msg::QoSProfile,
}

impl Default for Topic {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Topic::default())
    }
}

impl rosidl_runtime_rs::Message for Topic {
    type RmwMsg = super::msg::rmw::Topic;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                name: msg.name.as_str().into(),
                type_: super::msg::InterfaceType::into_rmw_message(std::borrow::Cow::Owned(
                    msg.type_,
                ))
                .into_owned(),
                qos: super::msg::QoSProfile::into_rmw_message(std::borrow::Cow::Owned(msg.qos))
                    .into_owned(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                name: msg.name.as_str().into(),
                type_: super::msg::InterfaceType::into_rmw_message(std::borrow::Cow::Borrowed(
                    &msg.type_,
                ))
                .into_owned(),
                qos: super::msg::QoSProfile::into_rmw_message(std::borrow::Cow::Borrowed(&msg.qos))
                    .into_owned(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            name: msg.name.to_string(),
            type_: super::msg::InterfaceType::from_rmw_message(msg.type_),
            qos: super::msg::QoSProfile::from_rmw_message(msg.qos),
        }
    }
}

// Corresponds to rosgraph_msgs__msg__TypeHash
/// RIHS spec version

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TypeHash::default())
    }
}

impl rosidl_runtime_rs::Message for TypeHash {
    type RmwMsg = super::msg::rmw::TypeHash;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                version: msg.version,
                value: msg.value,
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                version: msg.version,
                value: msg.value,
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            version: msg.version,
            value: msg.value,
        }
    }
}
