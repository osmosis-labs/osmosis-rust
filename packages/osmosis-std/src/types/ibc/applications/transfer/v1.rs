use osmosis_std_derive::CosmwasmExt;
/// Allocation defines the spend limit for a particular port and channel
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.Allocation")]
pub struct Allocation {
    /// the port on which the packet will be sent
    #[prost(string, tag = "1")]
    pub source_port: ::prost::alloc::string::String,
    /// the channel by which the packet will be sent
    #[prost(string, tag = "2")]
    pub source_channel: ::prost::alloc::string::String,
    /// spend limitation on the channel
    #[prost(message, repeated, tag = "3")]
    pub spend_limit:
        ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// allow list of receivers, an empty allow list permits any receiver address
    #[prost(string, repeated, tag = "4")]
    pub allow_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// TransferAuthorization allows the grantee to spend up to spend_limit coins from
/// the granter's account for ibc transfer on a specific channel
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.TransferAuthorization")]
pub struct TransferAuthorization {
    /// port and channel amounts
    #[prost(message, repeated, tag = "1")]
    pub allocations: ::prost::alloc::vec::Vec<Allocation>,
}
/// DenomTrace contains the base denomination for ICS20 fungible tokens and the
/// source tracing information path.
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.DenomTrace")]
pub struct DenomTrace {
    /// path defines the chain of port/channel identifiers used for tracing the
    /// source of the fungible token.
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// base denomination of the relayed fungible token.
    #[prost(string, tag = "2")]
    pub base_denom: ::prost::alloc::string::String,
}
/// Params defines the set of IBC transfer parameters.
/// NOTE: To prevent a single token from being transferred, set the
/// TransfersEnabled parameter to true and then set the bank module's SendEnabled
/// parameter for the denomination to false.
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.Params")]
pub struct Params {
    /// send_enabled enables or disables all cross-chain token transfers from this
    /// chain.
    #[prost(bool, tag = "1")]
    pub send_enabled: bool,
    /// receive_enabled enables or disables all cross-chain token transfers to this
    /// chain.
    #[prost(bool, tag = "2")]
    pub receive_enabled: bool,
}
/// GenesisState defines the ibc-transfer genesis state
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.GenesisState")]
pub struct GenesisState {
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub denom_traces: ::prost::alloc::vec::Vec<DenomTrace>,
    #[prost(message, optional, tag = "3")]
    pub params: ::core::option::Option<Params>,
    /// total_escrowed contains the total amount of tokens escrowed
    /// by the transfer module
    #[prost(message, repeated, tag = "4")]
    pub total_escrowed:
        ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryDenomTraceRequest is the request type for the Query/DenomTrace RPC
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.QueryDenomTraceRequest")]
#[proto_query(
    path = "/ibc.applications.transfer.v1.Query/DenomTrace",
    response_type = QueryDenomTraceResponse
)]
pub struct QueryDenomTraceRequest {
    /// hash (in hex format) or denom (full denom with ibc prefix) of the denomination trace information.
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
}
/// QueryDenomTraceResponse is the response type for the Query/DenomTrace RPC
/// method.
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.QueryDenomTraceResponse")]
pub struct QueryDenomTraceResponse {
    /// denom_trace returns the requested denomination trace information.
    #[prost(message, optional, tag = "1")]
    pub denom_trace: ::core::option::Option<DenomTrace>,
}
/// QueryConnectionsRequest is the request type for the Query/DenomTraces RPC
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.QueryDenomTracesRequest")]
#[proto_query(
    path = "/ibc.applications.transfer.v1.Query/DenomTraces",
    response_type = QueryDenomTracesResponse
)]
pub struct QueryDenomTracesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryConnectionsResponse is the response type for the Query/DenomTraces RPC
/// method.
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.QueryDenomTracesResponse")]
pub struct QueryDenomTracesResponse {
    /// denom_traces returns all denominations trace information.
    #[prost(message, repeated, tag = "1")]
    pub denom_traces: ::prost::alloc::vec::Vec<DenomTrace>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.QueryParamsRequest")]
#[proto_query(
    path = "/ibc.applications.transfer.v1.Query/Params",
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryDenomHashRequest is the request type for the Query/DenomHash RPC
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.QueryDenomHashRequest")]
#[proto_query(
    path = "/ibc.applications.transfer.v1.Query/DenomHash",
    response_type = QueryDenomHashResponse
)]
pub struct QueryDenomHashRequest {
    /// The denomination trace (\[port_id]/[channel_id])+/[denom\]
    #[prost(string, tag = "1")]
    pub trace: ::prost::alloc::string::String,
}
/// QueryDenomHashResponse is the response type for the Query/DenomHash RPC
/// method.
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.QueryDenomHashResponse")]
pub struct QueryDenomHashResponse {
    /// hash (in hex format) of the denomination trace information.
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
}
/// QueryEscrowAddressRequest is the request type for the EscrowAddress RPC method.
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.QueryEscrowAddressRequest")]
#[proto_query(
    path = "/ibc.applications.transfer.v1.Query/EscrowAddress",
    response_type = QueryEscrowAddressResponse
)]
pub struct QueryEscrowAddressRequest {
    /// unique port identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// unique channel identifier
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
}
/// QueryEscrowAddressResponse is the response type of the EscrowAddress RPC method.
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.QueryEscrowAddressResponse")]
pub struct QueryEscrowAddressResponse {
    /// the escrow account address
    #[prost(string, tag = "1")]
    pub escrow_address: ::prost::alloc::string::String,
}
/// QueryTotalEscrowForDenomRequest is the request type for TotalEscrowForDenom RPC method.
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.QueryTotalEscrowForDenomRequest")]
#[proto_query(
    path = "/ibc.applications.transfer.v1.Query/TotalEscrowForDenom",
    response_type = QueryTotalEscrowForDenomResponse
)]
pub struct QueryTotalEscrowForDenomRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryTotalEscrowForDenomResponse is the response type for TotalEscrowForDenom RPC method.
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.QueryTotalEscrowForDenomResponse")]
pub struct QueryTotalEscrowForDenomResponse {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgTransfer defines a msg to transfer fungible tokens (i.e Coins) between
/// ICS20 enabled chains. See ICS Spec here:
/// <https://github.com/cosmos/ibc/tree/master/spec/app/ics-020-fungible-token-transfer#data-structures>
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.MsgTransfer")]
pub struct MsgTransfer {
    /// the port on which the packet will be sent
    #[prost(string, tag = "1")]
    pub source_port: ::prost::alloc::string::String,
    /// the channel by which the packet will be sent
    #[prost(string, tag = "2")]
    pub source_channel: ::prost::alloc::string::String,
    /// the tokens to be transferred
    #[prost(message, optional, tag = "3")]
    pub token: ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// the sender address
    #[prost(string, tag = "4")]
    pub sender: ::prost::alloc::string::String,
    /// the recipient address on the destination chain
    #[prost(string, tag = "5")]
    pub receiver: ::prost::alloc::string::String,
    /// Timeout height relative to the current block height.
    /// The timeout is disabled when set to 0.
    #[prost(message, optional, tag = "6")]
    pub timeout_height: ::core::option::Option<super::super::super::core::client::v1::Height>,
    /// Timeout timestamp in absolute nanoseconds since unix epoch.
    /// The timeout is disabled when set to 0.
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub timeout_timestamp: u64,
    /// optional memo
    #[prost(string, tag = "8")]
    pub memo: ::prost::alloc::string::String,
}
/// MsgTransferResponse defines the Msg/Transfer response type.
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
#[proto_message(type_url = "/ibc.applications.transfer.v1.MsgTransferResponse")]
pub struct MsgTransferResponse {
    /// sequence number of the transfer packet sent
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sequence: u64,
}
pub struct TransferQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> TransferQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn denom_traces(
        &self,
        pagination: ::core::option::Option<
            super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryDenomTracesResponse, cosmwasm_std::StdError> {
        QueryDenomTracesRequest { pagination }.query(self.querier)
    }
    pub fn denom_trace(
        &self,
        hash: ::prost::alloc::string::String,
    ) -> Result<QueryDenomTraceResponse, cosmwasm_std::StdError> {
        QueryDenomTraceRequest { hash }.query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn denom_hash(
        &self,
        trace: ::prost::alloc::string::String,
    ) -> Result<QueryDenomHashResponse, cosmwasm_std::StdError> {
        QueryDenomHashRequest { trace }.query(self.querier)
    }
    pub fn escrow_address(
        &self,
        port_id: ::prost::alloc::string::String,
        channel_id: ::prost::alloc::string::String,
    ) -> Result<QueryEscrowAddressResponse, cosmwasm_std::StdError> {
        QueryEscrowAddressRequest {
            port_id,
            channel_id,
        }
        .query(self.querier)
    }
    pub fn total_escrow_for_denom(
        &self,
        denom: ::prost::alloc::string::String,
    ) -> Result<QueryTotalEscrowForDenomResponse, cosmwasm_std::StdError> {
        QueryTotalEscrowForDenomRequest { denom }.query(self.querier)
    }
}
