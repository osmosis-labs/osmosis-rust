use osmosis_std_derive::CosmwasmExt;
/// Params defines the parameters for the ibc-rate-limiting module.
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.ibcratelimit.v1beta1.Params")]
pub struct Params {
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
}
