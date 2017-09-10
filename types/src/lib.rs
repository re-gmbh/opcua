//! Contains data types and enumerations for OPC UA.
//!
//! 1. All of the built-in data types described in OPC Part 6 Chapter 5 that are encodable
//! 2. All of the standard data types described in OPC Part 3 Chapter 8 (if not covered by 1.)
//! 3. Autogenerated data types and request / responses as described in OPC Part 4

#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;
extern crate byteorder;
extern crate chrono;
extern crate regex;
extern crate rand;
extern crate url;

///Contains constants recognized by OPC UA clients and servers to describe various protocols and
/// profiles used during communication and encryption.
pub mod profiles {
    pub const TRANSPORT_PROFILE_URI_BINARY: &'static str = "http://opcfoundation.org/UA-Profile/Transport/uatcp-uasc-uabinary";

    pub const SECURITY_USER_TOKEN_POLICY_ANONYMOUS: &'static str = "http://opcfoundation.org/UA-Profile/Security/UserToken/Anonymous";
    pub const SECURITY_USER_TOKEN_POLICY_USERPASS: &'static str = "http://opcfoundation.org/UA-Profile/ Security/UserToken-Server/UserNamePassword";
}

pub mod constants {
    /// Maximum number of elements in an array
    pub const MAX_ARRAY_LENGTH: u32 = 1000;
    /// Maximum size of a string in chars
    pub const MAX_STRING_LENGTH: u32 = 65536;
    /// Maximum size of a byte string in bytes
    pub const MAX_BYTE_STRING_LENGTH: u32 = 65536;
    /// Maximum size of a certificate to send
    pub const MAX_CERTIFICATE_LENGTH: u32 = 32768;
}

mod encoding;
mod basic_types;
mod data_value;
mod date_time;
mod node_id;
mod variant;
mod data_types;
mod notification_message;
mod generated;
mod attribute;
mod service_types;
mod supported_message;

pub use self::encoding::*;
pub use self::basic_types::*;
pub use self::data_value::*;
pub use self::date_time::*;
pub use self::node_id::*;
pub use self::variant::*;
pub use self::data_types::*;
pub use self::generated::*;
pub use self::attribute::*;
pub use self::service_types::*;
pub use self::supported_message::*;

#[macro_export]
macro_rules! supported_message_as {
    ($v: expr, $i: ident) => {
        if let SupportedMessage::$i(value) = $v {
            value
        } else {
            panic!("Failed to get a supported message of type {}", stringify!($i));
        }
    }
}

#[macro_export]
macro_rules! supported_message_as_ref {
    ($v: expr, $i: ident) => {
        if let SupportedMessage::$i(ref value) = $v {
            value
        } else {
            panic!("Failed to get a supported message of type {}", stringify!($i));
        }
    }
}

#[macro_export]
macro_rules! supported_message_as_ref_mut {
    ($v: expr, $i: ident) => {
        if let SupportedMessage::$i(ref mut v) = $v {
            v
        } else {
            panic!("Failed to get a supported message of type {}", stringify!($i));
        }
    }
}

#[cfg(test)]
mod tests;
