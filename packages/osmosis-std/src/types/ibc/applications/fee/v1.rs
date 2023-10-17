use osmosis_std_derive::CosmwasmExt;
/// IncentivizedAcknowledgement is the acknowledgement format to be used by applications wrapped in the fee middleware
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
#[proto_message(type_url = "/ibc.applications.fee.v1.IncentivizedAcknowledgement")]
pub struct IncentivizedAcknowledgement {
    /// the underlying app acknowledgement bytes
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub app_acknowledgement: ::prost::alloc::vec::Vec<u8>,
    /// the relayer address which submits the recv packet message
    #[prost(string, tag = "2")]
    pub forward_relayer_address: ::prost::alloc::string::String,
    /// success flag of the base application callback
    #[prost(bool, tag = "3")]
    pub underlying_app_success: bool,
}
/// Fee defines the ICS29 receive, acknowledgement and timeout fees
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
#[proto_message(type_url = "/ibc.applications.fee.v1.Fee")]
pub struct Fee {
    /// the packet receive fee
    #[prost(message, repeated, tag = "1")]
    pub recv_fee: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// the packet acknowledgement fee
    #[prost(message, repeated, tag = "2")]
    pub ack_fee: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// the packet timeout fee
    #[prost(message, repeated, tag = "3")]
    pub timeout_fee:
        ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
/// PacketFee contains ICS29 relayer fees, refund address and optional list of permitted relayers
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
#[proto_message(type_url = "/ibc.applications.fee.v1.PacketFee")]
pub struct PacketFee {
    /// fee encapsulates the recv, ack and timeout fees associated with an IBC packet
    #[prost(message, optional, tag = "1")]
    pub fee: ::core::option::Option<Fee>,
    /// the refund address for unspent fees
    #[prost(string, tag = "2")]
    pub refund_address: ::prost::alloc::string::String,
    /// optional list of relayers permitted to receive fees
    #[prost(string, repeated, tag = "3")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PacketFees contains a list of type PacketFee
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
#[proto_message(type_url = "/ibc.applications.fee.v1.PacketFees")]
pub struct PacketFees {
    /// list of packet fees
    #[prost(message, repeated, tag = "1")]
    pub packet_fees: ::prost::alloc::vec::Vec<PacketFee>,
}
/// IdentifiedPacketFees contains a list of type PacketFee and associated PacketId
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
#[proto_message(type_url = "/ibc.applications.fee.v1.IdentifiedPacketFees")]
pub struct IdentifiedPacketFees {
    /// unique packet identifier comprised of the channel ID, port ID and sequence
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "packetID")]
    pub packet_id: ::core::option::Option<super::super::super::core::channel::v1::PacketId>,
    /// list of packet fees
    #[prost(message, repeated, tag = "2")]
    pub packet_fees: ::prost::alloc::vec::Vec<PacketFee>,
}
/// GenesisState defines the ICS29 fee middleware genesis state
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
#[proto_message(type_url = "/ibc.applications.fee.v1.GenesisState")]
pub struct GenesisState {
    /// list of identified packet fees
    #[prost(message, repeated, tag = "1")]
    pub identified_fees: ::prost::alloc::vec::Vec<IdentifiedPacketFees>,
    /// list of fee enabled channels
    #[prost(message, repeated, tag = "2")]
    pub fee_enabled_channels: ::prost::alloc::vec::Vec<FeeEnabledChannel>,
    /// list of registered payees
    #[prost(message, repeated, tag = "3")]
    pub registered_payees: ::prost::alloc::vec::Vec<RegisteredPayee>,
    /// list of registered counterparty payees
    #[prost(message, repeated, tag = "4")]
    pub registered_counterparty_payees: ::prost::alloc::vec::Vec<RegisteredCounterpartyPayee>,
    /// list of forward relayer addresses
    #[prost(message, repeated, tag = "5")]
    pub forward_relayers: ::prost::alloc::vec::Vec<ForwardRelayerAddress>,
}
/// FeeEnabledChannel contains the PortID & ChannelID for a fee enabled channel
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
#[proto_message(type_url = "/ibc.applications.fee.v1.FeeEnabledChannel")]
pub struct FeeEnabledChannel {
    /// unique port identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// unique channel identifier
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
}
/// RegisteredPayee contains the relayer address and payee address for a specific channel
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
#[proto_message(type_url = "/ibc.applications.fee.v1.RegisteredPayee")]
pub struct RegisteredPayee {
    /// unique channel identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// the relayer address
    #[prost(string, tag = "2")]
    pub relayer: ::prost::alloc::string::String,
    /// the payee address
    #[prost(string, tag = "3")]
    pub payee: ::prost::alloc::string::String,
}
/// RegisteredCounterpartyPayee contains the relayer address and counterparty payee address for a specific channel (used
/// for recv fee distribution)
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
#[proto_message(type_url = "/ibc.applications.fee.v1.RegisteredCounterpartyPayee")]
pub struct RegisteredCounterpartyPayee {
    /// unique channel identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// the relayer address
    #[prost(string, tag = "2")]
    pub relayer: ::prost::alloc::string::String,
    /// the counterparty payee address
    #[prost(string, tag = "3")]
    pub counterparty_payee: ::prost::alloc::string::String,
}
/// ForwardRelayerAddress contains the forward relayer address and PacketId used for async acknowledgements
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
#[proto_message(type_url = "/ibc.applications.fee.v1.ForwardRelayerAddress")]
pub struct ForwardRelayerAddress {
    /// the forward relayer address
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// unique packet identifer comprised of the channel ID, port ID and sequence
    #[prost(message, optional, tag = "2")]
    #[serde(alias = "packetID")]
    pub packet_id: ::core::option::Option<super::super::super::core::channel::v1::PacketId>,
}
/// Metadata defines the ICS29 channel specific metadata encoded into the channel version bytestring
/// See ICS004: <https://github.com/cosmos/ibc/tree/master/spec/core/ics-004-channel-and-packet-semantics#Versioning>
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
#[proto_message(type_url = "/ibc.applications.fee.v1.Metadata")]
pub struct Metadata {
    /// fee_version defines the ICS29 fee version
    #[prost(string, tag = "1")]
    pub fee_version: ::prost::alloc::string::String,
    /// app_version defines the underlying application version, which may or may not be a JSON encoded bytestring
    #[prost(string, tag = "2")]
    pub app_version: ::prost::alloc::string::String,
}
/// QueryIncentivizedPacketsRequest defines the request type for the IncentivizedPackets rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryIncentivizedPacketsRequest")]
#[proto_query(
    path = "/ibc.applications.fee.v1.Query/IncentivizedPackets",
    response_type = QueryIncentivizedPacketsResponse
)]
pub struct QueryIncentivizedPacketsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
    /// block height at which to query
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub query_height: u64,
}
/// QueryIncentivizedPacketsResponse defines the response type for the IncentivizedPackets rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryIncentivizedPacketsResponse")]
pub struct QueryIncentivizedPacketsResponse {
    /// list of identified fees for incentivized packets
    #[prost(message, repeated, tag = "1")]
    pub incentivized_packets: ::prost::alloc::vec::Vec<IdentifiedPacketFees>,
}
/// QueryIncentivizedPacketRequest defines the request type for the IncentivizedPacket rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryIncentivizedPacketRequest")]
#[proto_query(
    path = "/ibc.applications.fee.v1.Query/IncentivizedPacket",
    response_type = QueryIncentivizedPacketResponse
)]
pub struct QueryIncentivizedPacketRequest {
    /// unique packet identifier comprised of channel ID, port ID and sequence
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "packetID")]
    pub packet_id: ::core::option::Option<super::super::super::core::channel::v1::PacketId>,
    /// block height at which to query
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub query_height: u64,
}
/// QueryIncentivizedPacketsResponse defines the response type for the IncentivizedPacket rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryIncentivizedPacketResponse")]
pub struct QueryIncentivizedPacketResponse {
    /// the identified fees for the incentivized packet
    #[prost(message, optional, tag = "1")]
    pub incentivized_packet: ::core::option::Option<IdentifiedPacketFees>,
}
/// QueryIncentivizedPacketsForChannelRequest defines the request type for querying for all incentivized packets
/// for a specific channel
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryIncentivizedPacketsForChannelRequest")]
#[proto_query(
    path = "/ibc.applications.fee.v1.Query/IncentivizedPacketsForChannel",
    response_type = QueryIncentivizedPacketsForChannelResponse
)]
pub struct QueryIncentivizedPacketsForChannelRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
    #[prost(string, tag = "2")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// Height to query at
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub query_height: u64,
}
/// QueryIncentivizedPacketsResponse defines the response type for the incentivized packets RPC
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryIncentivizedPacketsForChannelResponse")]
pub struct QueryIncentivizedPacketsForChannelResponse {
    /// Map of all incentivized_packets
    #[prost(message, repeated, tag = "1")]
    pub incentivized_packets: ::prost::alloc::vec::Vec<IdentifiedPacketFees>,
}
/// QueryTotalRecvFeesRequest defines the request type for the TotalRecvFees rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryTotalRecvFeesRequest")]
#[proto_query(
    path = "/ibc.applications.fee.v1.Query/TotalRecvFees",
    response_type = QueryTotalRecvFeesResponse
)]
pub struct QueryTotalRecvFeesRequest {
    /// the packet identifier for the associated fees
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "packetID")]
    pub packet_id: ::core::option::Option<super::super::super::core::channel::v1::PacketId>,
}
/// QueryTotalRecvFeesResponse defines the response type for the TotalRecvFees rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryTotalRecvFeesResponse")]
pub struct QueryTotalRecvFeesResponse {
    /// the total packet receive fees
    #[prost(message, repeated, tag = "1")]
    pub recv_fees:
        ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryTotalAckFeesRequest defines the request type for the TotalAckFees rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryTotalAckFeesRequest")]
#[proto_query(
    path = "/ibc.applications.fee.v1.Query/TotalAckFees",
    response_type = QueryTotalAckFeesResponse
)]
pub struct QueryTotalAckFeesRequest {
    /// the packet identifier for the associated fees
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "packetID")]
    pub packet_id: ::core::option::Option<super::super::super::core::channel::v1::PacketId>,
}
/// QueryTotalAckFeesResponse defines the response type for the TotalAckFees rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryTotalAckFeesResponse")]
pub struct QueryTotalAckFeesResponse {
    /// the total packet acknowledgement fees
    #[prost(message, repeated, tag = "1")]
    pub ack_fees: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryTotalTimeoutFeesRequest defines the request type for the TotalTimeoutFees rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryTotalTimeoutFeesRequest")]
#[proto_query(
    path = "/ibc.applications.fee.v1.Query/TotalTimeoutFees",
    response_type = QueryTotalTimeoutFeesResponse
)]
pub struct QueryTotalTimeoutFeesRequest {
    /// the packet identifier for the associated fees
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "packetID")]
    pub packet_id: ::core::option::Option<super::super::super::core::channel::v1::PacketId>,
}
/// QueryTotalTimeoutFeesResponse defines the response type for the TotalTimeoutFees rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryTotalTimeoutFeesResponse")]
pub struct QueryTotalTimeoutFeesResponse {
    /// the total packet timeout fees
    #[prost(message, repeated, tag = "1")]
    pub timeout_fees:
        ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryPayeeRequest defines the request type for the Payee rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryPayeeRequest")]
#[proto_query(
    path = "/ibc.applications.fee.v1.Query/Payee",
    response_type = QueryPayeeResponse
)]
pub struct QueryPayeeRequest {
    /// unique channel identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// the relayer address to which the distribution address is registered
    #[prost(string, tag = "2")]
    pub relayer: ::prost::alloc::string::String,
}
/// QueryPayeeResponse defines the response type for the Payee rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryPayeeResponse")]
pub struct QueryPayeeResponse {
    /// the payee address to which packet fees are paid out
    #[prost(string, tag = "1")]
    pub payee_address: ::prost::alloc::string::String,
}
/// QueryCounterpartyPayeeRequest defines the request type for the CounterpartyPayee rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryCounterpartyPayeeRequest")]
#[proto_query(
    path = "/ibc.applications.fee.v1.Query/CounterpartyPayee",
    response_type = QueryCounterpartyPayeeResponse
)]
pub struct QueryCounterpartyPayeeRequest {
    /// unique channel identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// the relayer address to which the counterparty is registered
    #[prost(string, tag = "2")]
    pub relayer: ::prost::alloc::string::String,
}
/// QueryCounterpartyPayeeResponse defines the response type for the CounterpartyPayee rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryCounterpartyPayeeResponse")]
pub struct QueryCounterpartyPayeeResponse {
    /// the counterparty payee address used to compensate forward relaying
    #[prost(string, tag = "1")]
    pub counterparty_payee: ::prost::alloc::string::String,
}
/// QueryFeeEnabledChannelsRequest defines the request type for the FeeEnabledChannels rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryFeeEnabledChannelsRequest")]
#[proto_query(
    path = "/ibc.applications.fee.v1.Query/FeeEnabledChannels",
    response_type = QueryFeeEnabledChannelsResponse
)]
pub struct QueryFeeEnabledChannelsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
    /// block height at which to query
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub query_height: u64,
}
/// QueryFeeEnabledChannelsResponse defines the response type for the FeeEnabledChannels rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryFeeEnabledChannelsResponse")]
pub struct QueryFeeEnabledChannelsResponse {
    /// list of fee enabled channels
    #[prost(message, repeated, tag = "1")]
    pub fee_enabled_channels: ::prost::alloc::vec::Vec<FeeEnabledChannel>,
}
/// QueryFeeEnabledChannelRequest defines the request type for the FeeEnabledChannel rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryFeeEnabledChannelRequest")]
#[proto_query(
    path = "/ibc.applications.fee.v1.Query/FeeEnabledChannel",
    response_type = QueryFeeEnabledChannelResponse
)]
pub struct QueryFeeEnabledChannelRequest {
    /// unique port identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// unique channel identifier
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
}
/// QueryFeeEnabledChannelResponse defines the response type for the FeeEnabledChannel rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.QueryFeeEnabledChannelResponse")]
pub struct QueryFeeEnabledChannelResponse {
    /// boolean flag representing the fee enabled channel status
    #[prost(bool, tag = "1")]
    pub fee_enabled: bool,
}
/// MsgRegisterPayee defines the request type for the RegisterPayee rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.MsgRegisterPayee")]
pub struct MsgRegisterPayee {
    /// unique port identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// unique channel identifier
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// the relayer address
    #[prost(string, tag = "3")]
    pub relayer: ::prost::alloc::string::String,
    /// the payee address
    #[prost(string, tag = "4")]
    pub payee: ::prost::alloc::string::String,
}
/// MsgRegisterPayeeResponse defines the response type for the RegisterPayee rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.MsgRegisterPayeeResponse")]
pub struct MsgRegisterPayeeResponse {}
/// MsgRegisterCounterpartyPayee defines the request type for the RegisterCounterpartyPayee rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.MsgRegisterCounterpartyPayee")]
pub struct MsgRegisterCounterpartyPayee {
    /// unique port identifier
    #[prost(string, tag = "1")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// unique channel identifier
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// the relayer address
    #[prost(string, tag = "3")]
    pub relayer: ::prost::alloc::string::String,
    /// the counterparty payee address
    #[prost(string, tag = "4")]
    pub counterparty_payee: ::prost::alloc::string::String,
}
/// MsgRegisterCounterpartyPayeeResponse defines the response type for the RegisterCounterpartyPayee rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.MsgRegisterCounterpartyPayeeResponse")]
pub struct MsgRegisterCounterpartyPayeeResponse {}
/// MsgPayPacketFee defines the request type for the PayPacketFee rpc
/// This Msg can be used to pay for a packet at the next sequence send & should be combined with the Msg that will be
/// paid for
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
#[proto_message(type_url = "/ibc.applications.fee.v1.MsgPayPacketFee")]
pub struct MsgPayPacketFee {
    /// fee encapsulates the recv, ack and timeout fees associated with an IBC packet
    #[prost(message, optional, tag = "1")]
    pub fee: ::core::option::Option<Fee>,
    /// the source port unique identifier
    #[prost(string, tag = "2")]
    #[serde(alias = "source_portID")]
    pub source_port_id: ::prost::alloc::string::String,
    /// the source channel unique identifer
    #[prost(string, tag = "3")]
    #[serde(alias = "source_channelID")]
    pub source_channel_id: ::prost::alloc::string::String,
    /// account address to refund fee if necessary
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
    /// optional list of relayers permitted to the receive packet fees
    #[prost(string, repeated, tag = "5")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgPayPacketFeeResponse defines the response type for the PayPacketFee rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.MsgPayPacketFeeResponse")]
pub struct MsgPayPacketFeeResponse {}
/// MsgPayPacketFeeAsync defines the request type for the PayPacketFeeAsync rpc
/// This Msg can be used to pay for a packet at a specified sequence (instead of the next sequence send)
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
#[proto_message(type_url = "/ibc.applications.fee.v1.MsgPayPacketFeeAsync")]
pub struct MsgPayPacketFeeAsync {
    /// unique packet identifier comprised of the channel ID, port ID and sequence
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "packetID")]
    pub packet_id: ::core::option::Option<super::super::super::core::channel::v1::PacketId>,
    /// the packet fee associated with a particular IBC packet
    #[prost(message, optional, tag = "2")]
    pub packet_fee: ::core::option::Option<PacketFee>,
}
/// MsgPayPacketFeeAsyncResponse defines the response type for the PayPacketFeeAsync rpc
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
#[proto_message(type_url = "/ibc.applications.fee.v1.MsgPayPacketFeeAsyncResponse")]
pub struct MsgPayPacketFeeAsyncResponse {}
pub struct FeeQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> FeeQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn incentivized_packets(
        &self,
        pagination: ::core::option::Option<
            super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
        query_height: u64,
    ) -> Result<QueryIncentivizedPacketsResponse, cosmwasm_std::StdError> {
        QueryIncentivizedPacketsRequest {
            pagination,
            query_height,
        }
        .query(self.querier)
    }
    pub fn incentivized_packet(
        &self,
        packet_id: ::core::option::Option<super::super::super::core::channel::v1::PacketId>,
        query_height: u64,
    ) -> Result<QueryIncentivizedPacketResponse, cosmwasm_std::StdError> {
        QueryIncentivizedPacketRequest {
            packet_id,
            query_height,
        }
        .query(self.querier)
    }
    pub fn incentivized_packets_for_channel(
        &self,
        pagination: ::core::option::Option<
            super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
        port_id: ::prost::alloc::string::String,
        channel_id: ::prost::alloc::string::String,
        query_height: u64,
    ) -> Result<QueryIncentivizedPacketsForChannelResponse, cosmwasm_std::StdError> {
        QueryIncentivizedPacketsForChannelRequest {
            pagination,
            port_id,
            channel_id,
            query_height,
        }
        .query(self.querier)
    }
    pub fn total_recv_fees(
        &self,
        packet_id: ::core::option::Option<super::super::super::core::channel::v1::PacketId>,
    ) -> Result<QueryTotalRecvFeesResponse, cosmwasm_std::StdError> {
        QueryTotalRecvFeesRequest { packet_id }.query(self.querier)
    }
    pub fn total_ack_fees(
        &self,
        packet_id: ::core::option::Option<super::super::super::core::channel::v1::PacketId>,
    ) -> Result<QueryTotalAckFeesResponse, cosmwasm_std::StdError> {
        QueryTotalAckFeesRequest { packet_id }.query(self.querier)
    }
    pub fn total_timeout_fees(
        &self,
        packet_id: ::core::option::Option<super::super::super::core::channel::v1::PacketId>,
    ) -> Result<QueryTotalTimeoutFeesResponse, cosmwasm_std::StdError> {
        QueryTotalTimeoutFeesRequest { packet_id }.query(self.querier)
    }
    pub fn payee(
        &self,
        channel_id: ::prost::alloc::string::String,
        relayer: ::prost::alloc::string::String,
    ) -> Result<QueryPayeeResponse, cosmwasm_std::StdError> {
        QueryPayeeRequest {
            channel_id,
            relayer,
        }
        .query(self.querier)
    }
    pub fn counterparty_payee(
        &self,
        channel_id: ::prost::alloc::string::String,
        relayer: ::prost::alloc::string::String,
    ) -> Result<QueryCounterpartyPayeeResponse, cosmwasm_std::StdError> {
        QueryCounterpartyPayeeRequest {
            channel_id,
            relayer,
        }
        .query(self.querier)
    }
    pub fn fee_enabled_channels(
        &self,
        pagination: ::core::option::Option<
            super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
        query_height: u64,
    ) -> Result<QueryFeeEnabledChannelsResponse, cosmwasm_std::StdError> {
        QueryFeeEnabledChannelsRequest {
            pagination,
            query_height,
        }
        .query(self.querier)
    }
    pub fn fee_enabled_channel(
        &self,
        port_id: ::prost::alloc::string::String,
        channel_id: ::prost::alloc::string::String,
    ) -> Result<QueryFeeEnabledChannelResponse, cosmwasm_std::StdError> {
        QueryFeeEnabledChannelRequest {
            port_id,
            channel_id,
        }
        .query(self.querier)
    }
}
