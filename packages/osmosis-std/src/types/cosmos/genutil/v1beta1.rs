use osmosis_std_derive::CosmwasmExt;
/// GenesisState defines the raw genesis transaction in JSON.
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
#[proto_message(type_url = "/cosmos.genutil.v1beta1.GenesisState")]
pub struct GenesisState {
    /// gen_txs defines the genesis transactions.
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub gen_txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
