// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditField {
    #[prost(enumeration = "audit_field::FieldAction", tag = "1")]
    pub action: i32,
    #[prost(int32, tag = "2")]
    pub size: i32,
    /// no longer calculated
    #[deprecated]
    #[prost(int32, tag = "3")]
    pub size_delta: i32,
    #[prost(string, tag = "4")]
    pub field_id: ::prost::alloc::string::String,
    #[prost(enumeration = "super::resources::FieldType", tag = "5")]
    pub field_type: i32,
    #[prost(string, tag = "6")]
    pub filename: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AuditField`.
pub mod audit_field {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum FieldAction {
        Added = 0,
        Modified = 1,
        Deleted = 2,
    }
    impl FieldAction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FieldAction::Added => "ADDED",
                FieldAction::Modified => "MODIFIED",
                FieldAction::Deleted => "DELETED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ADDED" => Some(Self::Added),
                "MODIFIED" => Some(Self::Modified),
                "DELETED" => Some(Self::Deleted),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditKbCounter {
    #[prost(int64, tag = "2")]
    pub paragraphs: i64,
    #[prost(int64, tag = "3")]
    pub fields: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatContext {
    #[prost(string, tag = "1")]
    pub author: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub text: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChatAudit {
    #[prost(string, tag = "1")]
    pub question: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub answer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub rephrased_question: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub context: ::prost::alloc::vec::Vec<ChatContext>,
    #[prost(string, tag = "5")]
    pub learning_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditRequest {
    #[prost(enumeration = "audit_request::AuditType", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub kbid: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub userid: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, repeated, tag = "6")]
    pub fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "7")]
    pub search: ::core::option::Option<super::nodereader::SearchRequest>,
    #[prost(float, tag = "8")]
    pub timeit: f32,
    #[prost(string, tag = "9")]
    pub origin: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub rid: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub task: ::prost::alloc::string::String,
    #[prost(int32, tag = "12")]
    pub resources: i32,
    #[prost(message, repeated, tag = "13")]
    pub field_metadata: ::prost::alloc::vec::Vec<super::resources::FieldId>,
    #[prost(message, repeated, tag = "14")]
    pub fields_audit: ::prost::alloc::vec::Vec<AuditField>,
    #[prost(enumeration = "ClientType", tag = "16")]
    pub client_type: i32,
    #[prost(string, tag = "17")]
    pub trace_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "18")]
    pub kb_counter: ::core::option::Option<AuditKbCounter>,
    #[prost(message, optional, tag = "19")]
    pub chat: ::core::option::Option<ChatAudit>,
    #[prost(bool, tag = "20")]
    pub success: bool,
}
/// Nested message and enum types in `AuditRequest`.
pub mod audit_request {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum AuditType {
        Visited = 0,
        Modified = 1,
        Deleted = 2,
        New = 3,
        Started = 4,
        Stopped = 5,
        Search = 6,
        Processed = 7,
        KbDeleted = 8,
        Suggest = 9,
        Indexed = 10,
        Chat = 11,
    }
    impl AuditType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AuditType::Visited => "VISITED",
                AuditType::Modified => "MODIFIED",
                AuditType::Deleted => "DELETED",
                AuditType::New => "NEW",
                AuditType::Started => "STARTED",
                AuditType::Stopped => "STOPPED",
                AuditType::Search => "SEARCH",
                AuditType::Processed => "PROCESSED",
                AuditType::KbDeleted => "KB_DELETED",
                AuditType::Suggest => "SUGGEST",
                AuditType::Indexed => "INDEXED",
                AuditType::Chat => "CHAT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VISITED" => Some(Self::Visited),
                "MODIFIED" => Some(Self::Modified),
                "DELETED" => Some(Self::Deleted),
                "NEW" => Some(Self::New),
                "STARTED" => Some(Self::Started),
                "STOPPED" => Some(Self::Stopped),
                "SEARCH" => Some(Self::Search),
                "PROCESSED" => Some(Self::Processed),
                "KB_DELETED" => Some(Self::KbDeleted),
                "SUGGEST" => Some(Self::Suggest),
                "INDEXED" => Some(Self::Indexed),
                "CHAT" => Some(Self::Chat),
                _ => None,
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClientType {
    Api = 0,
    Web = 1,
    Widget = 2,
    Desktop = 3,
    Dashboard = 4,
    ChromeExtension = 5,
}
impl ClientType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClientType::Api => "API",
            ClientType::Web => "WEB",
            ClientType::Widget => "WIDGET",
            ClientType::Desktop => "DESKTOP",
            ClientType::Dashboard => "DASHBOARD",
            ClientType::ChromeExtension => "CHROME_EXTENSION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "API" => Some(Self::Api),
            "WEB" => Some(Self::Web),
            "WIDGET" => Some(Self::Widget),
            "DESKTOP" => Some(Self::Desktop),
            "DASHBOARD" => Some(Self::Dashboard),
            "CHROME_EXTENSION" => Some(Self::ChromeExtension),
            _ => None,
        }
    }
}