use osmosis_std_derive::CosmwasmExt;
/// GenesisState defines the ibc module's genesis state.
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
#[proto_message(type_url = "/ibc.core.types.v1.GenesisState")]
pub struct GenesisState {
    /// ICS002 - Clients genesis state
    #[prost(message, optional, tag = "1")]
    pub client_genesis: ::core::option::Option<super::super::client::v1::GenesisState>,
    /// ICS003 - Connections genesis state
    #[prost(message, optional, tag = "2")]
    pub connection_genesis: ::core::option::Option<super::super::connection::v1::GenesisState>,
    /// ICS004 - Channel genesis state
    #[prost(message, optional, tag = "3")]
    pub channel_genesis: ::core::option::Option<super::super::channel::v1::GenesisState>,
}
