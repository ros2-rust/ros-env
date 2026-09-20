// Copied verbatim from the generated Rust code for `unique_identifier_msgs` 2.2.1,
// built from the `humble` distro. Do not edit. Run `vendor_interfaces.py`
// to refresh it.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// Corresponds to unique_identifier_msgs__msg__UUID
/// A universally unique identifier (UUID).
///
///  http://en.wikipedia.org/wiki/Universally_unique_identifier
///  http://tools.ietf.org/html/rfc4122.html

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UUID {
    // This member is not documented.
    #[allow(missing_docs)]
    pub uuid: [u8; 16],
}

impl Default for UUID {
    fn default() -> Self {
        <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::UUID::default())
    }
}

impl rosidl_runtime_rs::Message for UUID {
    type RmwMsg = super::msg::rmw::UUID;

    fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
        match msg_cow {
            std::borrow::Cow::Owned(msg) => {
                std::borrow::Cow::Owned(Self::RmwMsg { uuid: msg.uuid })
            }
            std::borrow::Cow::Borrowed(msg) => {
                std::borrow::Cow::Owned(Self::RmwMsg { uuid: msg.uuid })
            }
        }
    }

    fn from_rmw_message(msg: Self::RmwMsg) -> Self {
        Self { uuid: msg.uuid }
    }
}
