use osmosis_std_derive::CosmwasmExt;
/// Capability defines an implementation of an object capability. The index
/// provided to a Capability must be globally unique.
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
#[proto_message(type_url = "/cosmos.capability.v1beta1.Capability")]
pub struct Capability {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub index: u64,
}
/// Owner defines a single capability owner. An owner is defined by the name of
/// capability and the module name.
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
#[proto_message(type_url = "/cosmos.capability.v1beta1.Owner")]
pub struct Owner {
    #[prost(string, tag = "1")]
    pub module: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// CapabilityOwners defines a set of owners of a single Capability. The set of
/// owners must be unique.
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
#[proto_message(type_url = "/cosmos.capability.v1beta1.CapabilityOwners")]
pub struct CapabilityOwners {
    #[prost(message, repeated, tag = "1")]
    pub owners: ::prost::alloc::vec::Vec<Owner>,
}
/// GenesisOwners defines the capability owners with their corresponding index.
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
#[proto_message(type_url = "/cosmos.capability.v1beta1.GenesisOwners")]
pub struct GenesisOwners {
    /// index is the index of the capability owner.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub index: u64,
    /// index_owners are the owners at the given index.
    #[prost(message, optional, tag = "2")]
    pub index_owners: ::core::option::Option<CapabilityOwners>,
}
/// GenesisState defines the capability module's genesis state.
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
#[proto_message(type_url = "/cosmos.capability.v1beta1.GenesisState")]
pub struct GenesisState {
    /// index is the capability global index.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub index: u64,
    /// owners represents a map from index to owners of the capability index
    /// index key is string to allow amino marshalling.
    #[prost(message, repeated, tag = "2")]
    pub owners: ::prost::alloc::vec::Vec<GenesisOwners>,
}
