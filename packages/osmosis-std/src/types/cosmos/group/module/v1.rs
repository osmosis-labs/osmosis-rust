use osmosis_std_derive::CosmwasmExt;
/// Module is the config object of the group module.
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
#[proto_message(type_url = "/cosmos.group.module.v1.Module")]
pub struct Module {
    /// max_execution_period defines the max duration after a proposal's voting period ends that members can send a MsgExec
    /// to execute the proposal.
    #[prost(message, optional, tag = "1")]
    pub max_execution_period: ::core::option::Option<crate::shim::Duration>,
    /// max_metadata_len defines the max length of the metadata bytes field for various entities within the group module.
    /// Defaults to 255 if not explicitly set.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_metadata_len: u64,
}
