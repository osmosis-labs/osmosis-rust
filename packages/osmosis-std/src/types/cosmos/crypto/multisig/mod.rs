pub mod v1beta1;
use osmosis_std_derive::CosmwasmExt;
/// LegacyAminoPubKey specifies a public key type
/// which nests multiple public keys and a threshold,
/// it uses legacy amino address rules.
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
#[proto_message(type_url = "/cosmos.crypto.multisig.LegacyAminoPubKey")]
pub struct LegacyAminoPubKey {
    #[prost(uint32, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub threshold: u32,
    #[prost(message, repeated, tag = "2")]
    pub public_keys: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
