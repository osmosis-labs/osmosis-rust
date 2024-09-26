use osmosis_std_derive::CosmwasmExt;
/// ConfigRequest defines the request structure for the Config gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.base.node.v1beta1.ConfigRequest")]
pub struct ConfigRequest {}
/// ConfigResponse defines the response structure for the Config gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.base.node.v1beta1.ConfigResponse")]
pub struct ConfigResponse {
    #[prost(string, tag = "1")]
    pub minimum_gas_price: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pruning_keep_recent: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub pruning_interval: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub halt_height: u64,
}
/// StateRequest defines the request structure for the status of a node.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.base.node.v1beta1.StatusRequest")]
pub struct StatusRequest {}
/// StateResponse defines the response structure for the status of a node.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.base.node.v1beta1.StatusResponse")]
pub struct StatusResponse {
    /// earliest block height available in the store
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub earliest_store_height: u64,
    /// current block height
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
    /// block height timestamp
    #[prost(message, optional, tag = "3")]
    pub timestamp: ::core::option::Option<crate::shim::Timestamp>,
    /// app hash of the current block
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
    /// validator hash provided by the consensus header
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub validator_hash: ::prost::alloc::vec::Vec<u8>,
}
