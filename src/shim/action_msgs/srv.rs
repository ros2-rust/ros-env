// Copied verbatim from the generated Rust code for `action_msgs` 1.2.3,
// built from the `humble` distro. Do not edit. Run `vendor_interfaces.py`
// to refresh it.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Corresponds to action_msgs__srv__CancelGoal_Request

// This struct is not documented.
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CancelGoal_Request {
    /// Goal info describing the goals to cancel, see above.
    pub goal_info: super::msg::GoalInfo,
}

impl Default for CancelGoal_Request {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::srv::rmw::CancelGoal_Request::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for CancelGoal_Request {
    type RmwMsg = super::srv::rmw::CancelGoal_Request;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                goal_info: super::msg::GoalInfo::into_rmw_message(std::borrow::Cow::Owned(
                    msg.goal_info,
                ))
                .into_owned(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                goal_info: super::msg::GoalInfo::into_rmw_message(std::borrow::Cow::Borrowed(
                    &msg.goal_info,
                ))
                .into_owned(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            goal_info: super::msg::GoalInfo::from_rmw_message(msg.goal_info),
        }
    }
}

// Corresponds to action_msgs__srv__CancelGoal_Response

// This struct is not documented.
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CancelGoal_Response {
    /// Return code, see above definitions.
    pub return_code: i8,

    /// Goals that accepted the cancel request.
    pub goals_canceling: Vec<super::msg::GoalInfo>,
}

impl CancelGoal_Response {
    /// Indicates the request was accepted without any errors.
    ///
    /// One or more goals have transitioned to the CANCELING state. The
    /// goals_canceling list is not empty.
    pub const ERROR_NONE: i8 = 0;

    /// Indicates the request was rejected.
    ///
    /// No goals have transitioned to the CANCELING state. The goals_canceling list is
    /// empty.
    pub const ERROR_REJECTED: i8 = 1;

    /// Indicates the requested goal ID does not exist.
    ///
    /// No goals have transitioned to the CANCELING state. The goals_canceling list is
    /// empty.
    pub const ERROR_UNKNOWN_GOAL_ID: i8 = 2;

    /// Indicates the goal is not cancelable because it is already in a terminal state.
    ///
    /// No goals have transitioned to the CANCELING state. The goals_canceling list is
    /// empty.
    pub const ERROR_GOAL_TERMINATED: i8 = 3;
}

impl Default for CancelGoal_Response {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::srv::rmw::CancelGoal_Response::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for CancelGoal_Response {
    type RmwMsg = super::srv::rmw::CancelGoal_Response;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                return_code: msg.return_code,
                goals_canceling: msg
                    .goals_canceling
                    .into_iter()
                    .map(|elem| {
                        super::msg::GoalInfo::into_rmw_message(std::borrow::Cow::Owned(elem))
                            .into_owned()
                    })
                    .collect(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                return_code: msg.return_code,
                goals_canceling: msg
                    .goals_canceling
                    .iter()
                    .map(|elem| {
                        super::msg::GoalInfo::into_rmw_message(std::borrow::Cow::Borrowed(elem))
                            .into_owned()
                    })
                    .collect(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            return_code: msg.return_code,
            goals_canceling: msg
                .goals_canceling
                .into_iter()
                .map(super::msg::GoalInfo::from_rmw_message)
                .collect(),
        }
    }
}

#[link(name = "action_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__action_msgs__srv__CancelGoal(
    ) -> *const std::ffi::c_void;
}

// Corresponds to action_msgs__srv__CancelGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct CancelGoal;

impl rosidl_runtime_rs::Service for CancelGoal {
    type Request = CancelGoal_Request;
    type Response = CancelGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__action_msgs__srv__CancelGoal()
        }
    }
}
