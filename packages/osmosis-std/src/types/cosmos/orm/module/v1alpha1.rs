use osmosis_std_derive::CosmwasmExt;
/// Module defines the ORM module which adds providers to the app container for
/// ORM ModuleDB's and in the future will automatically register query
/// services for modules that use the ORM.
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
#[proto_message(type_url = "/cosmos.orm.module.v1alpha1.Module")]
pub struct Module {}
