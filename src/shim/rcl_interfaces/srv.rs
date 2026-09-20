// Copied verbatim from the generated Rust code for `rcl_interfaces` 1.2.3,
// built from the `humble` distro. Do not edit. Run `vendor_interfaces.py`
// to refresh it.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Corresponds to rcl_interfaces__srv__DescribeParameters_Request

// This struct is not documented.
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DescribeParameters_Request {
    // This member is not documented.
    #[allow(missing_docs)]
    pub names: Vec<std::string::String>,
}

impl Default for DescribeParameters_Request {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::srv::rmw::DescribeParameters_Request::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for DescribeParameters_Request {
    type RmwMsg = super::srv::rmw::DescribeParameters_Request;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                names: msg
                    .names
                    .into_iter()
                    .map(|elem| elem.as_str().into())
                    .collect(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                names: msg.names.iter().map(|elem| elem.as_str().into()).collect(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            names: msg.names.into_iter().map(|elem| elem.to_string()).collect(),
        }
    }
}

// Corresponds to rcl_interfaces__srv__DescribeParameters_Response

// This struct is not documented.
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DescribeParameters_Response {
    // This member is not documented.
    #[allow(missing_docs)]
    pub descriptors: Vec<super::msg::ParameterDescriptor>,
}

impl Default for DescribeParameters_Response {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::srv::rmw::DescribeParameters_Response::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for DescribeParameters_Response {
    type RmwMsg = super::srv::rmw::DescribeParameters_Response;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                descriptors: msg
                    .descriptors
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
                descriptors: msg
                    .descriptors
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
            descriptors: msg
                .descriptors
                .into_iter()
                .map(super::msg::ParameterDescriptor::from_rmw_message)
                .collect(),
        }
    }
}

// Corresponds to rcl_interfaces__srv__GetParameters_Request

// This struct is not documented.
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetParameters_Request {
    /// A list of parameter names to get.
    pub names: Vec<std::string::String>,
}

impl Default for GetParameters_Request {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::srv::rmw::GetParameters_Request::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for GetParameters_Request {
    type RmwMsg = super::srv::rmw::GetParameters_Request;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                names: msg
                    .names
                    .into_iter()
                    .map(|elem| elem.as_str().into())
                    .collect(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                names: msg.names.iter().map(|elem| elem.as_str().into()).collect(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            names: msg.names.into_iter().map(|elem| elem.to_string()).collect(),
        }
    }
}

// Corresponds to rcl_interfaces__srv__GetParameters_Response

// This struct is not documented.
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetParameters_Response {
    // This member is not documented.
    #[allow(missing_docs)]
    pub values: Vec<super::msg::ParameterValue>,
}

impl Default for GetParameters_Response {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::srv::rmw::GetParameters_Response::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for GetParameters_Response {
    type RmwMsg = super::srv::rmw::GetParameters_Response;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                values: msg
                    .values
                    .into_iter()
                    .map(|elem| {
                        super::msg::ParameterValue::into_rmw_message(std::borrow::Cow::Owned(elem))
                            .into_owned()
                    })
                    .collect(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                values: msg
                    .values
                    .iter()
                    .map(|elem| {
                        super::msg::ParameterValue::into_rmw_message(std::borrow::Cow::Borrowed(
                            elem,
                        ))
                        .into_owned()
                    })
                    .collect(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            values: msg
                .values
                .into_iter()
                .map(super::msg::ParameterValue::from_rmw_message)
                .collect(),
        }
    }
}

// Corresponds to rcl_interfaces__srv__GetParameterTypes_Request

// This struct is not documented.
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetParameterTypes_Request {
    // This member is not documented.
    #[allow(missing_docs)]
    pub names: Vec<std::string::String>,
}

impl Default for GetParameterTypes_Request {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::srv::rmw::GetParameterTypes_Request::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for GetParameterTypes_Request {
    type RmwMsg = super::srv::rmw::GetParameterTypes_Request;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                names: msg
                    .names
                    .into_iter()
                    .map(|elem| elem.as_str().into())
                    .collect(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                names: msg.names.iter().map(|elem| elem.as_str().into()).collect(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            names: msg.names.into_iter().map(|elem| elem.to_string()).collect(),
        }
    }
}

// Corresponds to rcl_interfaces__srv__GetParameterTypes_Response

// This struct is not documented.
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetParameterTypes_Response {
    // This member is not documented.
    #[allow(missing_docs)]
    pub types: Vec<u8>,
}

impl Default for GetParameterTypes_Response {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::srv::rmw::GetParameterTypes_Response::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for GetParameterTypes_Response {
    type RmwMsg = super::srv::rmw::GetParameterTypes_Response;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                types: msg.types.as_slice().into(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                types: msg.types.as_slice().into(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            types: msg.types.into(),
        }
    }
}

// Corresponds to rcl_interfaces__srv__ListParameters_Request

// This struct is not documented.
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListParameters_Request {
    /// The list of parameter prefixes to query.
    pub prefixes: Vec<std::string::String>,

    /// Relative depth from given prefixes to return.
    ///
    /// Use DEPTH_RECURSIVE to get the recursive parameters and prefixes for each prefix.
    pub depth: u64,
}

impl ListParameters_Request {
    // This constant is not documented.
    #[allow(missing_docs)]
    pub const DEPTH_RECURSIVE: u64 = 0;
}

impl Default for ListParameters_Request {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::srv::rmw::ListParameters_Request::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for ListParameters_Request {
    type RmwMsg = super::srv::rmw::ListParameters_Request;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                prefixes: msg
                    .prefixes
                    .into_iter()
                    .map(|elem| elem.as_str().into())
                    .collect(),
                depth: msg.depth,
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                prefixes: msg
                    .prefixes
                    .iter()
                    .map(|elem| elem.as_str().into())
                    .collect(),
                depth: msg.depth,
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            prefixes: msg
                .prefixes
                .into_iter()
                .map(|elem| elem.to_string())
                .collect(),
            depth: msg.depth,
        }
    }
}

// Corresponds to rcl_interfaces__srv__ListParameters_Response

// This struct is not documented.
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ListParameters_Response {
    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::msg::ListParametersResult,
}

impl Default for ListParameters_Response {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::srv::rmw::ListParameters_Response::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for ListParameters_Response {
    type RmwMsg = super::srv::rmw::ListParameters_Response;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                result: super::msg::ListParametersResult::into_rmw_message(
                    std::borrow::Cow::Owned(msg.result),
                )
                .into_owned(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                result: super::msg::ListParametersResult::into_rmw_message(
                    std::borrow::Cow::Borrowed(&msg.result),
                )
                .into_owned(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            result: super::msg::ListParametersResult::from_rmw_message(msg.result),
        }
    }
}

// Corresponds to rcl_interfaces__srv__SetParametersAtomically_Request

// This struct is not documented.
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetParametersAtomically_Request {
    // This member is not documented.
    #[allow(missing_docs)]
    pub parameters: Vec<super::msg::Parameter>,
}

impl Default for SetParametersAtomically_Request {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::srv::rmw::SetParametersAtomically_Request::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for SetParametersAtomically_Request {
    type RmwMsg = super::srv::rmw::SetParametersAtomically_Request;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                parameters: msg
                    .parameters
                    .into_iter()
                    .map(|elem| {
                        super::msg::Parameter::into_rmw_message(std::borrow::Cow::Owned(elem))
                            .into_owned()
                    })
                    .collect(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                parameters: msg
                    .parameters
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
            parameters: msg
                .parameters
                .into_iter()
                .map(super::msg::Parameter::from_rmw_message)
                .collect(),
        }
    }
}

// Corresponds to rcl_interfaces__srv__SetParametersAtomically_Response

// This struct is not documented.
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetParametersAtomically_Response {
    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::msg::SetParametersResult,
}

impl Default for SetParametersAtomically_Response {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::srv::rmw::SetParametersAtomically_Response::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for SetParametersAtomically_Response {
    type RmwMsg = super::srv::rmw::SetParametersAtomically_Response;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                result: super::msg::SetParametersResult::into_rmw_message(std::borrow::Cow::Owned(
                    msg.result,
                ))
                .into_owned(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                result: super::msg::SetParametersResult::into_rmw_message(
                    std::borrow::Cow::Borrowed(&msg.result),
                )
                .into_owned(),
            }),
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self {
            result: super::msg::SetParametersResult::from_rmw_message(msg.result),
        }
    }
}

// Corresponds to rcl_interfaces__srv__SetParameters_Request

// This struct is not documented.
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetParameters_Request {
    // This member is not documented.
    #[allow(missing_docs)]
    pub parameters: Vec<super::msg::Parameter>,
}

impl Default for SetParameters_Request {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::srv::rmw::SetParameters_Request::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for SetParameters_Request {
    type RmwMsg = super::srv::rmw::SetParameters_Request;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                parameters: msg
                    .parameters
                    .into_iter()
                    .map(|elem| {
                        super::msg::Parameter::into_rmw_message(std::borrow::Cow::Owned(elem))
                            .into_owned()
                    })
                    .collect(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                parameters: msg
                    .parameters
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
            parameters: msg
                .parameters
                .into_iter()
                .map(super::msg::Parameter::from_rmw_message)
                .collect(),
        }
    }
}

// Corresponds to rcl_interfaces__srv__SetParameters_Response

// This struct is not documented.
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SetParameters_Response {
    // This member is not documented.
    #[allow(missing_docs)]
    pub results: Vec<super::msg::SetParametersResult>,
}

impl Default for SetParameters_Response {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(
            super::srv::rmw::SetParameters_Response::default(),
        )
    }
}

impl rosidl_runtime_rs::Message for SetParameters_Response {
    type RmwMsg = super::srv::rmw::SetParameters_Response;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                results: msg
                    .results
                    .into_iter()
                    .map(|elem| {
                        super::msg::SetParametersResult::into_rmw_message(std::borrow::Cow::Owned(
                            elem,
                        ))
                        .into_owned()
                    })
                    .collect(),
            }),
            std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
                results: msg
                    .results
                    .iter()
                    .map(|elem| {
                        super::msg::SetParametersResult::into_rmw_message(
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
            results: msg
                .results
                .into_iter()
                .map(super::msg::SetParametersResult::from_rmw_message)
                .collect(),
        }
    }
}

#[link(name = "rcl_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__DescribeParameters(
    ) -> *const std::ffi::c_void;
}

// Corresponds to rcl_interfaces__srv__DescribeParameters
#[allow(missing_docs, non_camel_case_types)]
pub struct DescribeParameters;

impl rosidl_runtime_rs::Service for DescribeParameters {
    type Request = DescribeParameters_Request;
    type Response = DescribeParameters_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__DescribeParameters()
        }
    }
}

#[link(name = "rcl_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__GetParameters(
    ) -> *const std::ffi::c_void;
}

// Corresponds to rcl_interfaces__srv__GetParameters
#[allow(missing_docs, non_camel_case_types)]
pub struct GetParameters;

impl rosidl_runtime_rs::Service for GetParameters {
    type Request = GetParameters_Request;
    type Response = GetParameters_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__GetParameters()
        }
    }
}

#[link(name = "rcl_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__GetParameterTypes(
    ) -> *const std::ffi::c_void;
}

// Corresponds to rcl_interfaces__srv__GetParameterTypes
#[allow(missing_docs, non_camel_case_types)]
pub struct GetParameterTypes;

impl rosidl_runtime_rs::Service for GetParameterTypes {
    type Request = GetParameterTypes_Request;
    type Response = GetParameterTypes_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__GetParameterTypes()
        }
    }
}

#[link(name = "rcl_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__ListParameters(
    ) -> *const std::ffi::c_void;
}

// Corresponds to rcl_interfaces__srv__ListParameters
#[allow(missing_docs, non_camel_case_types)]
pub struct ListParameters;

impl rosidl_runtime_rs::Service for ListParameters {
    type Request = ListParameters_Request;
    type Response = ListParameters_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__ListParameters()
        }
    }
}

#[link(name = "rcl_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__SetParametersAtomically(
    ) -> *const std::ffi::c_void;
}

// Corresponds to rcl_interfaces__srv__SetParametersAtomically
#[allow(missing_docs, non_camel_case_types)]
pub struct SetParametersAtomically;

impl rosidl_runtime_rs::Service for SetParametersAtomically {
    type Request = SetParametersAtomically_Request;
    type Response = SetParametersAtomically_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__SetParametersAtomically()
        }
    }
}

#[link(name = "rcl_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__SetParameters(
    ) -> *const std::ffi::c_void;
}

// Corresponds to rcl_interfaces__srv__SetParameters
#[allow(missing_docs, non_camel_case_types)]
pub struct SetParameters;

impl rosidl_runtime_rs::Service for SetParameters {
    type Request = SetParameters_Request;
    type Response = SetParameters_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__rcl_interfaces__srv__SetParameters()
        }
    }
}
