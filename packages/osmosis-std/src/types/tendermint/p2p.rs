use osmosis_std_derive::CosmwasmExt;
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
#[proto_message(type_url = "/tendermint.p2p.NetAddress")]
pub struct NetAddress {
    #[prost(string, tag = "1")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub ip: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub port: u32,
}
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
#[proto_message(type_url = "/tendermint.p2p.ProtocolVersion")]
pub struct ProtocolVersion {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub p2p: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub block: u64,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub app: u64,
}
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
#[proto_message(type_url = "/tendermint.p2p.DefaultNodeInfo")]
pub struct DefaultNodeInfo {
    #[prost(message, optional, tag = "1")]
    pub protocol_version: ::core::option::Option<ProtocolVersion>,
    #[prost(string, tag = "2")]
    #[serde(alias = "default_nodeID")]
    pub default_node_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub listen_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub network: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub version: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub channels: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "7")]
    pub moniker: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "8")]
    pub other: ::core::option::Option<DefaultNodeInfoOther>,
}
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
#[proto_message(type_url = "/tendermint.p2p.DefaultNodeInfoOther")]
pub struct DefaultNodeInfoOther {
    #[prost(string, tag = "1")]
    pub tx_index: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub rpc_address: ::prost::alloc::string::String,
}
