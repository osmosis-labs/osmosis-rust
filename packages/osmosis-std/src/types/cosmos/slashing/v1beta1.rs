use osmosis_std_derive::CosmwasmExt;
/// ValidatorSigningInfo defines a validator's signing info for monitoring their
/// liveness activity.
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
#[proto_message(type_url = "/cosmos.slashing.v1beta1.ValidatorSigningInfo")]
pub struct ValidatorSigningInfo {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Height at which validator was first a candidate OR was unjailed
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub start_height: i64,
    /// Index which is incremented each time the validator was a bonded
    /// in a block and may have signed a precommit or not. This in conjunction with the
    /// `SignedBlocksWindow` param determines the index in the `MissedBlocksBitArray`.
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub index_offset: i64,
    /// Timestamp until which the validator is jailed due to liveness downtime.
    #[prost(message, optional, tag = "4")]
    pub jailed_until: ::core::option::Option<crate::shim::Timestamp>,
    /// Whether or not a validator has been tombstoned (killed out of validator set). It is set
    /// once the validator commits an equivocation or for any other configured misbehiavor.
    #[prost(bool, tag = "5")]
    pub tombstoned: bool,
    /// A counter kept to avoid unnecessary array reads.
    /// Note that `Sum(MissedBlocksBitArray)` always equals `MissedBlocksCounter`.
    #[prost(int64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub missed_blocks_counter: i64,
}
/// Params represents the parameters used for by the slashing module.
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
#[proto_message(type_url = "/cosmos.slashing.v1beta1.Params")]
pub struct Params {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub signed_blocks_window: i64,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub min_signed_per_window: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub downtime_jail_duration: ::core::option::Option<crate::shim::Duration>,
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub slash_fraction_double_sign: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub slash_fraction_downtime: ::prost::alloc::vec::Vec<u8>,
}
/// GenesisState defines the slashing module's genesis state.
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
#[proto_message(type_url = "/cosmos.slashing.v1beta1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// signing_infos represents a map between validator addresses and their
    /// signing infos.
    #[prost(message, repeated, tag = "2")]
    pub signing_infos: ::prost::alloc::vec::Vec<SigningInfo>,
    /// missed_blocks represents a map between validator addresses and their
    /// missed blocks.
    #[prost(message, repeated, tag = "3")]
    pub missed_blocks: ::prost::alloc::vec::Vec<ValidatorMissedBlocks>,
}
/// SigningInfo stores validator signing info of corresponding address.
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
#[proto_message(type_url = "/cosmos.slashing.v1beta1.SigningInfo")]
pub struct SigningInfo {
    /// address is the validator address.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// validator_signing_info represents the signing info of this validator.
    #[prost(message, optional, tag = "2")]
    pub validator_signing_info: ::core::option::Option<ValidatorSigningInfo>,
}
/// ValidatorMissedBlocks contains array of missed blocks of corresponding
/// address.
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
#[proto_message(type_url = "/cosmos.slashing.v1beta1.ValidatorMissedBlocks")]
pub struct ValidatorMissedBlocks {
    /// address is the validator address.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// missed_blocks is an array of missed blocks by the validator.
    #[prost(message, repeated, tag = "2")]
    pub missed_blocks: ::prost::alloc::vec::Vec<MissedBlock>,
}
/// MissedBlock contains height and missed status as boolean.
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
#[proto_message(type_url = "/cosmos.slashing.v1beta1.MissedBlock")]
pub struct MissedBlock {
    /// index is the height at which the block was missed.
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub index: i64,
    /// missed is the missed status.
    #[prost(bool, tag = "2")]
    pub missed: bool,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method
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
#[proto_message(type_url = "/cosmos.slashing.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/cosmos.slashing.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method
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
#[proto_message(type_url = "/cosmos.slashing.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QuerySigningInfoRequest is the request type for the Query/SigningInfo RPC
/// method
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
#[proto_message(type_url = "/cosmos.slashing.v1beta1.QuerySigningInfoRequest")]
#[proto_query(
    path = "/cosmos.slashing.v1beta1.Query/SigningInfo",
    response_type = QuerySigningInfoResponse
)]
pub struct QuerySigningInfoRequest {
    /// cons_address is the address to query signing info of
    #[prost(string, tag = "1")]
    pub cons_address: ::prost::alloc::string::String,
}
/// QuerySigningInfoResponse is the response type for the Query/SigningInfo RPC
/// method
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
#[proto_message(type_url = "/cosmos.slashing.v1beta1.QuerySigningInfoResponse")]
pub struct QuerySigningInfoResponse {
    /// val_signing_info is the signing info of requested val cons address
    #[prost(message, optional, tag = "1")]
    pub val_signing_info: ::core::option::Option<ValidatorSigningInfo>,
}
/// QuerySigningInfosRequest is the request type for the Query/SigningInfos RPC
/// method
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
#[proto_message(type_url = "/cosmos.slashing.v1beta1.QuerySigningInfosRequest")]
#[proto_query(
    path = "/cosmos.slashing.v1beta1.Query/SigningInfos",
    response_type = QuerySigningInfosResponse
)]
pub struct QuerySigningInfosRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QuerySigningInfosResponse is the response type for the Query/SigningInfos RPC
/// method
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
#[proto_message(type_url = "/cosmos.slashing.v1beta1.QuerySigningInfosResponse")]
pub struct QuerySigningInfosResponse {
    /// info is the signing info of all validators
    #[prost(message, repeated, tag = "1")]
    pub info: ::prost::alloc::vec::Vec<ValidatorSigningInfo>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// MsgUnjail defines the Msg/Unjail request type
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
#[proto_message(type_url = "/cosmos.slashing.v1beta1.MsgUnjail")]
pub struct MsgUnjail {
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// MsgUnjailResponse defines the Msg/Unjail response type
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
#[proto_message(type_url = "/cosmos.slashing.v1beta1.MsgUnjailResponse")]
pub struct MsgUnjailResponse {}
/// MsgUpdateParams is the Msg/UpdateParams request type.
///
/// Since: cosmos-sdk 0.47
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
#[proto_message(type_url = "/cosmos.slashing.v1beta1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/slashing parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
///
/// Since: cosmos-sdk 0.47
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
#[proto_message(type_url = "/cosmos.slashing.v1beta1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct SlashingQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> SlashingQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn signing_info(
        &self,
        cons_address: ::prost::alloc::string::String,
    ) -> Result<QuerySigningInfoResponse, cosmwasm_std::StdError> {
        QuerySigningInfoRequest { cons_address }.query(self.querier)
    }
    pub fn signing_infos(
        &self,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QuerySigningInfosResponse, cosmwasm_std::StdError> {
        QuerySigningInfosRequest { pagination }.query(self.querier)
    }
}
