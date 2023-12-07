use osmosis_std_derive::CosmwasmExt;
/// Params defines the set of on-chain interchain accounts parameters.
/// The following parameters may be used to disable the controller submodule.
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
#[proto_message(type_url = "/ibc.applications.interchain_accounts.controller.v1.Params")]
pub struct Params {
    /// controller_enabled enables or disables the controller submodule.
    #[prost(bool, tag = "1")]
    pub controller_enabled: bool,
}
/// QueryInterchainAccountRequest is the request type for the Query/InterchainAccount RPC method.
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
#[proto_message(
    type_url = "/ibc.applications.interchain_accounts.controller.v1.QueryInterchainAccountRequest"
)]
#[proto_query(
    path = "/ibc.applications.interchain_accounts.controller.v1.Query/InterchainAccount",
    response_type = QueryInterchainAccountResponse
)]
pub struct QueryInterchainAccountRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
}
/// QueryInterchainAccountResponse the response type for the Query/InterchainAccount RPC method.
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
#[proto_message(
    type_url = "/ibc.applications.interchain_accounts.controller.v1.QueryInterchainAccountResponse"
)]
pub struct QueryInterchainAccountResponse {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
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
#[proto_message(
    type_url = "/ibc.applications.interchain_accounts.controller.v1.QueryParamsRequest"
)]
#[proto_query(
    path = "/ibc.applications.interchain_accounts.controller.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
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
#[proto_message(
    type_url = "/ibc.applications.interchain_accounts.controller.v1.QueryParamsResponse"
)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// MsgRegisterInterchainAccount defines the payload for Msg/RegisterAccount
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
#[proto_message(
    type_url = "/ibc.applications.interchain_accounts.controller.v1.MsgRegisterInterchainAccount"
)]
pub struct MsgRegisterInterchainAccount {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
}
/// MsgRegisterInterchainAccountResponse defines the response for Msg/RegisterAccount
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
#[proto_message(
    type_url = "/ibc.applications.interchain_accounts.controller.v1.MsgRegisterInterchainAccountResponse"
)]
pub struct MsgRegisterInterchainAccountResponse {
    #[prost(string, tag = "1")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
}
/// MsgSendTx defines the payload for Msg/SendTx
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
#[proto_message(type_url = "/ibc.applications.interchain_accounts.controller.v1.MsgSendTx")]
pub struct MsgSendTx {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub packet_data: ::core::option::Option<super::super::v1::InterchainAccountPacketData>,
    /// Relative timeout timestamp provided will be added to the current block time during transaction execution.
    /// The timeout timestamp must be non-zero.
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub relative_timeout: u64,
}
/// MsgSendTxResponse defines the response for MsgSendTx
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
#[proto_message(type_url = "/ibc.applications.interchain_accounts.controller.v1.MsgSendTxResponse")]
pub struct MsgSendTxResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence: u64,
}
pub struct ControllerQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ControllerQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn interchain_account(
        &self,
        owner: ::prost::alloc::string::String,
        connection_id: ::prost::alloc::string::String,
    ) -> Result<QueryInterchainAccountResponse, cosmwasm_std::StdError> {
        QueryInterchainAccountRequest {
            owner,
            connection_id,
        }
        .query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
}
