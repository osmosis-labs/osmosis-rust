pub mod v1beta1;
use osmosis_std_derive::CosmwasmExt;
/// Params holds parameters for the incentives module
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.Params")]
pub struct Params {
    #[prost(message, repeated, tag = "1")]
    pub pool_creation_fee: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the gamm module's genesis state.
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.gamm.GenesisState")]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<crate::shim::Any>,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub next_pool_number: u64,
    #[prost(message, optional, tag = "3")]
    pub params: ::core::option::Option<Params>,
}
