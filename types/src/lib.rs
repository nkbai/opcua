//! The OPC UA Types module contains data types and enumerations for OPC UA.
//!
//! This includes:
//!
//! 1. All of the built-in data types described in OPC Part 6 Chapter 5 that are encodable.
//! 2. All of the standard data types described in OPC Part 3 Chapter 8 (if not covered by 1.).
//! 3. Autogenerated data types and request / responses as described in OPC Part 4.
//!
//! For the built-in data types, the module provides functions

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate serde_json;

#[macro_export]
macro_rules! supported_message_as {
    ($v: expr, $i: ident) => {
        if let SupportedMessage::$i(value) = $v {
            *value
        } else {
            panic!();
        }
    }
}

///Contains constants recognized by OPC UA clients and servers to describe various protocols and
/// profiles used during communication and encryption.
pub mod profiles {
    pub const TRANSPORT_PROFILE_URI_BINARY: &str = "http://opcfoundation.org/UA-Profile/Transport/uatcp-uasc-uabinary";

    pub const SECURITY_USER_TOKEN_POLICY_ANONYMOUS: &str = "http://opcfoundation.org/UA-Profile/Security/UserToken/Anonymous";
    pub const SECURITY_USER_TOKEN_POLICY_USERPASS: &str = "http://opcfoundation.org/UA-Profile/ Security/UserToken-Server/UserNamePassword";
}

pub mod constants {
    /// Default OPC UA port number. Used by a discovery server. Other servers would normally run
    /// on a different port. So OPC UA for Rust does not use this nr by default but it is used
    /// implicitly in opc.tcp:// urls and elsewhere.
    pub const DEFAULT_OPC_UA_SERVER_PORT: u16 = 4840;
    /// Maximum number of elements in an array
    pub const MAX_ARRAY_LENGTH: usize = 100000; //采样点个数可能会有几万个
    /// Maximum size of a string in chars
    pub const MAX_STRING_LENGTH: usize = 65536;
    /// Maximum size of a byte string in bytes
    pub const MAX_BYTE_STRING_LENGTH: usize = 65536;
    /// Maximum size of a certificate to send
    pub const MAX_CERTIFICATE_LENGTH: u32 = 32768;

    /// URI supplied for the None security policy
    pub const SECURITY_POLICY_NONE_URI: &str = "http://opcfoundation.org/UA/SecurityPolicy#None";
    /// URI supplied for the `Basic128Rsa15` security policy
    pub const SECURITY_POLICY_BASIC_128_RSA_15_URI: &str = "http://opcfoundation.org/UA/SecurityPolicy#Basic128Rsa15";
    /// URI supplied for the `Basic256` security policy
    pub const SECURITY_POLICY_BASIC_256_URI: &str = "http://opcfoundation.org/UA/SecurityPolicy#Basic256";
    /// URI supplied for the `Basic256Sha256` security policy
    pub const SECURITY_POLICY_BASIC_256_SHA_256_URI: &str = "http://opcfoundation.org/UA/SecurityPolicy#Basic256Sha256";

    /// String used as shorthand in config files, debug etc.for `None` security policy
    pub const SECURITY_POLICY_NONE: &str = "None";
    /// String used as shorthand in config files, debug etc.for `Basic128Rsa15` security policy
    pub const SECURITY_POLICY_BASIC_128_RSA_15: &str = "Basic128Rsa15";
    /// String used as shorthand in config files, debug etc.for `Basic256` security policy
    pub const SECURITY_POLICY_BASIC_256: &str = "Basic256";
    /// String used as shorthand in config files, debug etc.for `Basic256Sha256` security policy
    pub const SECURITY_POLICY_BASIC_256_SHA_256: &str = "Basic256Sha256";
}

// Attributes mask bits
bitflags! {
    pub struct AttributesMask: u32 {
        /// Indicates if the AccessLevel Attribute is set.
        const ACCESS_LEVEL = 1;
        /// Indicates if the ArrayDimensions Attribute is set.
        const ARRAY_DIMENSIONS = 1 << 1;
        /// Indicates if the ContainsNoLoops Attribute is set.
        const CONTAINS_NO_LOOPS = 1 << 3;
        /// Indicates if the DataType Attribute is set.
        const DATA_TYPE = 1 << 4;
        /// Indicates if the Description Attribute is set.
        const DESCRIPTION = 1 << 5;
        /// Indicates if the DisplayName Attribute is set.
        const DISPLAY_NAME = 1 << 6;
        /// Indicates if the EventNotifier Attribute is set.
        const EVENT_NOTIFIER = 1 << 7;
        /// Indicates if the Executable Attribute is set.
        const EXECUTABLE = 1 << 8;
        /// Indicates if the Historizing Attribute is set.
        const HISTORIZING = 1 << 9;
        /// Indicates if the InverseName Attribute is set.
        const INVERSE_NAME = 1 << 10;
        /// Indicates if the IsAbstract Attribute is set.
        const IS_ABSTRACT = 1 << 11;
        /// Indicates if the MinimumSamplingInterval Attribute is set.
        const MINIMUM_SAMPLING_INTERVAL = 1 << 12;
        /// Indicates if the Symmetric Attribute is set.
        const SYMMETRIC = 1 << 15;
        /// Indicates if the UserAccessLevel Attribute is set.
        const USER_ACCESS_LEVEL = 1 << 16;
        /// Indicates if the UserExecutable Attribute is set.
        const USER_EXECUTABLE = 1 << 17;
        /// Indicates if the UserWriteMask Attribute is set.
        const USER_WRITE_MASK = 1 << 18;
        /// Indicates if the ValueRank Attribute is set.
        const VALUE_RANK = 1 << 19;
        /// Indicates if the WriteMask Attribute is set.
        const WRITE_MASK = 1 << 20;
        /// Indicates if the Value Attribute is set
        const VALUE = 1 << 21;
    }
}

// Write mask bits (similar but different to AttributesMask)
bitflags! {
    pub struct WriteMask: u32 {
        /// Indicates if the AccessLevel Attribute is writable.
        const ACCESS_LEVEL = 1;
        /// Indicates if the ArrayDimensions Attribute is writable.
        const ARRAY_DIMENSIONS = 1 << 1;
        ///Indicates if the BrowseName Attribute is writable.
        const BROWSE_NAME = 1 << 2;
        /// Indicates if the ContainsNoLoops Attribute is writable.
        const CONTAINS_NO_LOOPS = 1 << 3;
        /// Indicates if the DataType Attribute is writable.
        const DATA_TYPE = 1 << 4;
        /// Indicates if the Description Attribute is writable.
        const DESCRIPTION = 1 << 5;
        /// Indicates if the DisplayName Attribute is writable.
        const DISPLAY_NAME = 1 << 6;
        /// Indicates if the EventNotifier Attribute is writable.
        const EVENT_NOTIFIER = 1 << 7;
        /// Indicates if the Executable Attribute is writable.
        const EXECUTABLE = 1 << 8;
        /// Indicates if the Historizing Attribute is writable.
        const HISTORIZING = 1 << 9;
        /// Indicates if the InverseName Attribute is writable.
        const INVERSE_NAME = 1 << 10;
        /// Indicates if the IsAbstract Attribute is writable.
        const IS_ABSTRACT = 1 << 11;
        /// Indicates if the MinimumSamplingInterval Attribute is writable.
        const MINIMUM_SAMPLING_INTERVAL = 1 << 12;
        /// Indicates if the NodeClass Attribute is writable.
        const NODE_CLASS = 1 << 13;
        /// Indicates if the NodeId Attribute is writable.
        const NODE_ID = 1 << 14;
        /// Indicates if the Symmetric Attribute is writable.
        const SYMMETRIC = 1 << 15;
        /// Indicates if the UserAccessLevel Attribute is writable.
        const USER_ACCESS_LEVEL = 1 << 16;
        /// Indicates if the UserExecutable Attribute is writable.
        const USER_EXECUTABLE = 1 << 17;
        /// Indicates if the UserWriteMask Attribute is writable.
        const USER_WRITE_MASK = 1 << 18;
        /// Indicates if the ValueRank Attribute is writable.
        const VALUE_RANK = 1 << 19;
        /// Indicates if the WriteMask Attribute is writable.
        const WRITE_MASK = 1 << 20;
        /// Indicates if the Value Attribute is writable for a VariableType. It does not apply for Variables
        /// since this is handled by the AccessLevel and UserAccessLevel Attributes for the Variable.
        /// For Variables this bit shall be set to 0.
        const VALUE_FOR_VARIABLE_TYPE = 1 << 21;
    }
}

mod status_codes;

pub mod encoding;
pub mod basic_types;
pub mod string;
pub mod extension_object;
pub mod byte_string;
pub mod data_value;
pub mod date_time;
pub mod diagnostic_info;
pub mod guid;
pub mod node_id;
pub mod node_ids;
pub mod variant;
pub mod data_types;
pub mod notification_message;
pub mod attribute;
pub mod supported_message;
pub mod numeric_range;
pub mod url;
pub mod argument;
pub mod tcp_types;
pub mod service_types;
pub mod status_code;
pub mod relative_path;
pub mod operand;
pub mod request_header;
pub mod response_header;

pub use crate::{
    encoding::*,
    basic_types::*,
    string::*,
    extension_object::*,
    byte_string::*,
    data_value::*,
    diagnostic_info::*,
    date_time::*,
    guid::*,
    node_id::*,
    node_ids::*,
    variant::*,
    data_types::*,
    attribute::*,
    supported_message::*,
    service_types::*,
    numeric_range::*,
    url::*,
    argument::*,
    operand::*,
    request_header::*,
    response_header::*,
};

#[cfg(test)]
mod tests;
