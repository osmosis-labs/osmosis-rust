use osmosis_std_derive::CosmwasmExt;
/// Module is the config object of the capability module.
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
#[proto_message(type_url = "/cosmos.capability.module.v1.Module")]
pub struct Module {
    /// seal_keeper defines if keeper.Seal() will run on BeginBlock() to prevent further modules from creating a scoped
    /// keeper. For more details check x/capability/keeper.go.
    #[prost(bool, tag = "1")]
    pub seal_keeper: bool,
}
