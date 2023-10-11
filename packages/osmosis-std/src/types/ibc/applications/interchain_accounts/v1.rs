use osmosis_std_derive::CosmwasmExt;
/// An InterchainAccount is defined as a BaseAccount & the address of the account owner on the controller chain
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
#[proto_message(type_url = "/ibc.applications.interchain_accounts.v1.InterchainAccount")]
pub struct InterchainAccount {
    #[prost(message, optional, tag = "1")]
    pub base_account:
        ::core::option::Option<super::super::super::super::cosmos::auth::v1beta1::BaseAccount>,
    #[prost(string, tag = "2")]
    pub account_owner: ::prost::alloc::string::String,
}
/// GenesisState defines the interchain accounts genesis state
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
#[proto_message(type_url = "/ibc.applications.interchain_accounts.v1.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub controller_genesis_state: ::core::option::Option<ControllerGenesisState>,
    #[prost(message, optional, tag = "2")]
    pub host_genesis_state: ::core::option::Option<HostGenesisState>,
}
/// ControllerGenesisState defines the interchain accounts controller genesis state
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
#[proto_message(type_url = "/ibc.applications.interchain_accounts.v1.ControllerGenesisState")]
pub struct ControllerGenesisState {
    #[prost(message, repeated, tag = "1")]
    pub active_channels: ::prost::alloc::vec::Vec<ActiveChannel>,
    #[prost(message, repeated, tag = "2")]
    pub interchain_accounts: ::prost::alloc::vec::Vec<RegisteredInterchainAccount>,
    #[prost(string, repeated, tag = "3")]
    pub ports: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub params: ::core::option::Option<super::controller::v1::Params>,
}
/// HostGenesisState defines the interchain accounts host genesis state
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
#[proto_message(type_url = "/ibc.applications.interchain_accounts.v1.HostGenesisState")]
pub struct HostGenesisState {
    #[prost(message, repeated, tag = "1")]
    pub active_channels: ::prost::alloc::vec::Vec<ActiveChannel>,
    #[prost(message, repeated, tag = "2")]
    pub interchain_accounts: ::prost::alloc::vec::Vec<RegisteredInterchainAccount>,
    #[prost(string, tag = "3")]
    pub port: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub params: ::core::option::Option<super::host::v1::Params>,
}
/// ActiveChannel contains a connection ID, port ID and associated active channel ID
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
#[proto_message(type_url = "/ibc.applications.interchain_accounts.v1.ActiveChannel")]
pub struct ActiveChannel {
    #[prost(string, tag = "1")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
}
/// RegisteredInterchainAccount contains a connection ID, port ID and associated interchain account address
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
#[proto_message(type_url = "/ibc.applications.interchain_accounts.v1.RegisteredInterchainAccount")]
pub struct RegisteredInterchainAccount {
    #[prost(string, tag = "1")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub account_address: ::prost::alloc::string::String,
}
/// Metadata defines a set of protocol specific data encoded into the ICS27 channel version bytestring
/// See ICS004: <https://github.com/cosmos/ibc/tree/master/spec/core/ics-004-channel-and-packet-semantics#Versioning>
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
#[proto_message(type_url = "/ibc.applications.interchain_accounts.v1.Metadata")]
pub struct Metadata {
    /// version defines the ICS27 protocol version
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// controller_connection_id is the connection identifier associated with the controller chain
    #[prost(string, tag = "2")]
    #[serde(alias = "controller_connectionID")]
    pub controller_connection_id: ::prost::alloc::string::String,
    /// host_connection_id is the connection identifier associated with the host chain
    #[prost(string, tag = "3")]
    #[serde(alias = "host_connectionID")]
    pub host_connection_id: ::prost::alloc::string::String,
    /// address defines the interchain account address to be fulfilled upon the OnChanOpenTry handshake step
    /// NOTE: the address field is empty on the OnChanOpenInit handshake step
    #[prost(string, tag = "4")]
    pub address: ::prost::alloc::string::String,
    /// encoding defines the supported codec format
    #[prost(string, tag = "5")]
    pub encoding: ::prost::alloc::string::String,
    /// tx_type defines the type of transactions the interchain account can execute
    #[prost(string, tag = "6")]
    pub tx_type: ::prost::alloc::string::String,
}
/// InterchainAccountPacketData is comprised of a raw transaction, type of transaction and optional memo field.
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
#[proto_message(type_url = "/ibc.applications.interchain_accounts.v1.InterchainAccountPacketData")]
pub struct InterchainAccountPacketData {
    #[prost(enumeration = "Type", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub r#type: i32,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub memo: ::prost::alloc::string::String,
}
/// CosmosTx contains a list of sdk.Msg's. It should be used when sending transactions to an SDK host chain.
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
#[proto_message(type_url = "/ibc.applications.interchain_accounts.v1.CosmosTx")]
pub struct CosmosTx {
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
/// Type defines a classification of message issued from a controller chain to its associated interchain accounts
/// host
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum Type {
    /// Default zero value enumeration
    Unspecified = 0,
    /// Execute a transaction on an interchain accounts host chain
    ExecuteTx = 1,
}
impl Type {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Type::Unspecified => "TYPE_UNSPECIFIED",
            Type::ExecuteTx => "TYPE_EXECUTE_TX",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "TYPE_EXECUTE_TX" => Some(Self::ExecuteTx),
            _ => None,
        }
    }
}
