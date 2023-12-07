use osmosis_std_derive::CosmwasmExt;
/// ClientState defines the 09-localhost client state
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
#[proto_message(type_url = "/ibc.lightclients.localhost.v2.ClientState")]
pub struct ClientState {
    /// the latest block height
    #[prost(message, optional, tag = "1")]
    pub latest_height: ::core::option::Option<super::super::super::core::client::v1::Height>,
}
