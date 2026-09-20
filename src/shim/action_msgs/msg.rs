// Copied verbatim from the generated Rust code for `action_msgs` 1.2.3,
// built from the `humble` distro. Do not edit. Run `vendor_interfaces.py`
// to refresh it.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Corresponds to action_msgs__msg__GoalInfo
/// Goal ID

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoalInfo {
    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

    /// Time when the goal was accepted
    pub stamp: builtin_interfaces::msg::Time,
}

impl Default for GoalInfo {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::GoalInfo::default())
    }
}

impl rosidl_runtime_rs::Message for GoalInfo {
    type RmwMsg = super::msg::rmw::GoalInfo;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(
                    std::borrow::Cow::Owned(msg.goal_id),
                )
                .into_owned(),
                stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(
                    msg.stamp,
                ))
                .into_owned(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(
                    std::borrow::Cow::Borrowed(&msg.goal_id),
                )
                .into_owned(),
                stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(
                    &msg.stamp,
                ))
                .into_owned(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
            stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
        }
    }
}

// Corresponds to action_msgs__msg__GoalStatus
/// An action goal can be in one of these states after it is accepted by an action
/// server.
///
/// For more information, see http://design.ros2.org/articles/actions.html

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoalStatus {
    /// Goal info (contains ID and timestamp).
    pub goal_info: super::msg::GoalInfo,

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
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::msg::rmw::GoalStatus::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for GoalStatus {
    type RmwMsg = super::msg::rmw::GoalStatus;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                goal_info: super::msg::GoalInfo::into_rmw_message(std::borrow::Cow::Owned(
                    msg.goal_info,
                ))
                .into_owned(),
                status: msg.status,
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                goal_info: super::msg::GoalInfo::into_rmw_message(std::borrow::Cow::Borrowed(
                    &msg.goal_info,
                ))
                .into_owned(),
                status: msg.status,
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            goal_info: super::msg::GoalInfo::from_rmw_message(msg.goal_info),
            status: msg.status,
        }
    }
}

// Corresponds to action_msgs__msg__GoalStatusArray
/// An array of goal statuses.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoalStatusArray {
    // This member is not documented.
    #[allow(missing_docs)]
    pub status_list: Vec<super::msg::GoalStatus>,
}

impl Default for GoalStatusArray {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::msg::rmw::GoalStatusArray::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for GoalStatusArray {
    type RmwMsg = super::msg::rmw::GoalStatusArray;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                status_list: msg
                    .status_list
                    .into_iter()
                    .map(|elem| {
                        super::msg::GoalStatus::into_rmw_message(std::borrow::Cow::Owned(elem))
                            .into_owned()
                    })
                    .collect(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                status_list: msg
                    .status_list
                    .iter()
                    .map(|elem| {
                        super::msg::GoalStatus::into_rmw_message(std::borrow::Cow::Borrowed(elem))
                            .into_owned()
                    })
                    .collect(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            status_list: msg
                .status_list
                .into_iter()
                .map(super::msg::GoalStatus::from_rmw_message)
                .collect(),
        }
    }
}
