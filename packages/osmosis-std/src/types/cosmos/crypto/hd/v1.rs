use osmosis_std_derive::CosmwasmExt;
/// BIP44Params is used as path field in ledger item in Record.
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
#[proto_message(type_url = "/cosmos.crypto.hd.v1.BIP44Params")]
pub struct Bip44Params {
    /// purpose is a constant set to 44' (or 0x8000002C) following the BIP43 recommendation
    #[prost(uint32, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub purpose: u32,
    /// coin_type is a constant that improves privacy
    #[prost(uint32, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub coin_type: u32,
    /// account splits the key space into independent user identities
    #[prost(uint32, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub account: u32,
    /// change is a constant used for public derivation. Constant 0 is used for external chain and constant 1 for internal
    /// chain.
    #[prost(bool, tag = "4")]
    pub change: bool,
    /// address_index is used as child index in BIP32 derivation
    #[prost(uint32, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub address_index: u32,
}
