use osmosis_std_derive::CosmwasmExt;
/// Params defines params for x/bridge module.
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.Params")]
pub struct Params {
    /// Signers used to sign inbound and release outbound transactions
    #[prost(string, repeated, tag = "1")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Assets is a list used to create tokenfactory denoms
    /// for corresponding trading pairs
    #[prost(message, repeated, tag = "2")]
    pub assets: ::prost::alloc::vec::Vec<Asset>,
    /// VotesNeeded marks how many signers out of the list of signers need
    /// to sign until a tx can be considered finalized
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub votes_needed: u64,
    /// Fee defines a param for fee that go towards the validator set
    /// signing the incoming/outgoing txs. The fee is measured as a ratio,
    /// so its value lies between 0 and 1.
    #[prost(string, tag = "4")]
    pub fee: ::prost::alloc::string::String,
}
/// AssetID defines a pair of the source chain name and its Osmosis
/// representation denoted by denom. AssetID is a primary key for Asset.
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.AssetID")]
pub struct AssetId {
    /// SourceChain is a source chain name
    #[prost(string, tag = "1")]
    pub source_chain: ::prost::alloc::string::String,
    /// Denom is the Osmosis representation of the SourceChain
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
}
/// Asset is a representation of the asset.
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.Asset")]
pub struct Asset {
    /// ID is the asset's primary key
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "ID")]
    pub id: ::core::option::Option<AssetId>,
    /// Status is a current status of the asset
    #[prost(enumeration = "AssetStatus", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub status: i32,
    /// Exponent represents the power of 10 used for coin representation
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub exponent: u64,
    /// ExternalConfirmations is a number of the confirmations on the external
    /// chain needed to consider the transfer confirmed
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub external_confirmations: u64,
}
/// InboundTransfer is a representation of the inbound transfer.
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.InboundTransfer")]
pub struct InboundTransfer {
    /// ExternalId is a unique ID of the transfer coming from outside.
    /// Serves the purpose of uniquely identifying the transfer in another chain
    /// (e.g., this might be the BTC tx hash).
    #[prost(string, tag = "1")]
    #[serde(alias = "externalID")]
    pub external_id: ::prost::alloc::string::String,
    /// ExternalHeight is the height at which the transfer occurred in the external
    /// chain
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub external_height: u64,
    /// DestAddr is a destination Osmosis address
    #[prost(string, tag = "3")]
    pub dest_addr: ::prost::alloc::string::String,
    /// AssetID is the ID of the asset being transferred
    #[prost(message, optional, tag = "4")]
    #[serde(alias = "assetID")]
    pub asset_id: ::core::option::Option<AssetId>,
    /// Amount of coins to transfer
    #[prost(string, tag = "5")]
    pub amount: ::prost::alloc::string::String,
    /// Voters is a list of validators signed this transfer
    #[prost(string, repeated, tag = "6")]
    pub voters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Finalized indicates whether the transfer needs more votes or has
    /// already accumulated a sufficient number. The finalised flag is set
    /// to true as soon as length(voters) is greater than or equal to
    /// the module's param votes_needed.
    #[prost(bool, tag = "7")]
    pub finalized: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum AssetStatus {
    Unspecified = 0,
    Ok = 1,
    BlockedInbound = 2,
    BlockedOutbound = 3,
    BlockedBoth = 4,
}
impl AssetStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AssetStatus::Unspecified => "ASSET_STATUS_UNSPECIFIED",
            AssetStatus::Ok => "ASSET_STATUS_OK",
            AssetStatus::BlockedInbound => "ASSET_STATUS_BLOCKED_INBOUND",
            AssetStatus::BlockedOutbound => "ASSET_STATUS_BLOCKED_OUTBOUND",
            AssetStatus::BlockedBoth => "ASSET_STATUS_BLOCKED_BOTH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ASSET_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "ASSET_STATUS_OK" => Some(Self::Ok),
            "ASSET_STATUS_BLOCKED_INBOUND" => Some(Self::BlockedInbound),
            "ASSET_STATUS_BLOCKED_OUTBOUND" => Some(Self::BlockedOutbound),
            "ASSET_STATUS_BLOCKED_BOTH" => Some(Self::BlockedBoth),
            _ => None,
        }
    }
}
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.EventInboundTransfer")]
pub struct EventInboundTransfer {
    /// Sender is a sender's address
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// DestAddr is a destination Osmosis address
    #[prost(string, tag = "2")]
    pub dest_addr: ::prost::alloc::string::String,
    /// AssetID is the ID of the asset being transferred
    #[prost(message, optional, tag = "3")]
    #[serde(alias = "assetID")]
    pub asset_id: ::core::option::Option<AssetId>,
    /// Amount of coins to transfer
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.EventOutboundTransfer")]
pub struct EventOutboundTransfer {
    /// Sender is a sender's address
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// DestAddr is a destination Osmosis address
    #[prost(string, tag = "2")]
    pub dest_addr: ::prost::alloc::string::String,
    /// AssetID is the ID of the asset being transferred
    #[prost(message, optional, tag = "3")]
    #[serde(alias = "assetID")]
    pub asset_id: ::core::option::Option<AssetId>,
    /// Amount of coins to transfer
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.EventUpdateParams")]
pub struct EventUpdateParams {
    #[prost(string, repeated, tag = "1")]
    pub new_signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub created_signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub deleted_signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub new_assets: ::prost::alloc::vec::Vec<Asset>,
    #[prost(message, repeated, tag = "5")]
    pub created_assets: ::prost::alloc::vec::Vec<Asset>,
    #[prost(message, repeated, tag = "6")]
    pub deleted_assets: ::prost::alloc::vec::Vec<Asset>,
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub new_votes_needed: u64,
    #[prost(string, tag = "8")]
    pub new_fee: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.EventChangeAssetStatus")]
pub struct EventChangeAssetStatus {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    #[serde(alias = "assetID")]
    pub asset_id: ::core::option::Option<AssetId>,
    #[prost(enumeration = "AssetStatus", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub old_status: i32,
    #[prost(enumeration = "AssetStatus", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub new_status: i32,
}
/// GenesisState defines the mint module's genesis state.
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.GenesisState")]
pub struct GenesisState {
    /// Params defines params for x/bridge module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/osmosis.bridge.v1beta1.Query/Params",
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// LastTransferHeightRequest is the request type for the
/// Query/LastTransferHeight RPC method.
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.LastTransferHeightRequest")]
#[proto_query(
    path = "/osmosis.bridge.v1beta1.Query/LastTransferHeight",
    response_type = LastTransferHeightResponse
)]
pub struct LastTransferHeightRequest {
    #[prost(message, optional, tag = "1")]
    #[serde(alias = "assetID")]
    pub asset_id: ::core::option::Option<AssetId>,
}
/// LastTransferHeightResponse is the response type for the
/// Query/LastTransferHeight RPC method.
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.LastTransferHeightResponse")]
pub struct LastTransferHeightResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_transfer_height: u64,
}
/// MsgInboundTransfer defines the message structure for the InboundTransfer gRPC
/// service method. It allows a sender to perform an inbound cross-chain
/// transfer, i.e., to transfer their tokens from the source chain to Osmosis and
/// get the equivalent amount of the corresponding token (specified in subdenom)
/// on Osmosis in return. The tokens are minted through the x/tokenfactory module
/// to the destination address.
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.MsgInboundTransfer")]
pub struct MsgInboundTransfer {
    /// ExternalId is a unique ID of the transfer coming from outside.
    /// Serves the purpose of uniquely identifying the transfer in another chain
    /// (e.g., this might be the BTC tx hash)
    #[prost(string, tag = "1")]
    #[serde(alias = "externalID")]
    pub external_id: ::prost::alloc::string::String,
    /// ExternalHeight is the height at which the transfer occurred in the external
    /// chain
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub external_height: u64,
    /// Sender is a sender's address
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
    /// DestAddr is a destination Osmosis address
    #[prost(string, tag = "4")]
    pub dest_addr: ::prost::alloc::string::String,
    /// AssetID is the ID of the asset being transferred
    #[prost(message, optional, tag = "5")]
    #[serde(alias = "assetID")]
    pub asset_id: ::core::option::Option<AssetId>,
    /// Amount of coins to transfer
    #[prost(string, tag = "6")]
    pub amount: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.MsgInboundTransferResponse")]
pub struct MsgInboundTransferResponse {}
/// MsgOutboundTransfer defines the message structure for the OutboundTransfer
/// gRPC service method. It allows a sender to perform an outbound cross-chain
/// transfer, i.e., to transfer their tokens from Osmosis to the destination
/// chain. The tokens are burned through the x/tokenfactory module from the
/// sender's address.
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.MsgOutboundTransfer")]
pub struct MsgOutboundTransfer {
    /// Sender is a sender's Osmosis address
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// DestAddr is a destination address
    #[prost(string, tag = "2")]
    pub dest_addr: ::prost::alloc::string::String,
    /// AssetID is the ID of the asset being transferred
    #[prost(message, optional, tag = "3")]
    #[serde(alias = "assetID")]
    pub asset_id: ::core::option::Option<AssetId>,
    /// Amount of coins to transfer
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.MsgOutboundTransferResponse")]
pub struct MsgOutboundTransferResponse {}
/// MsgUpdateParams allows to update module params. It contains UpdateParams
/// instead of just Params to forbid status updating using this method.
/// All new assets introduced with this method have ASSET_STATUS_BLOCKED_BOTH
/// status by default.
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// Sender is a sender's address
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// NewParams should be fully populated
    #[prost(message, optional, tag = "2")]
    pub new_params: ::core::option::Option<Params>,
}
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
/// MsgChangeAssetStatus changes the status of the provided asset.
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.MsgChangeAssetStatus")]
pub struct MsgChangeAssetStatus {
    /// Sender is a sender's address
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// AssetID is the ID of the asset to update.
    /// The asset should be known; otherwise, the method will failed.
    #[prost(message, optional, tag = "2")]
    #[serde(alias = "assetID")]
    pub asset_id: ::core::option::Option<AssetId>,
    /// NewStatus is a new asset's status.
    #[prost(enumeration = "AssetStatus", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub new_status: i32,
}
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
#[proto_message(type_url = "/osmosis.bridge.v1beta1.MsgChangeAssetStatusResponse")]
pub struct MsgChangeAssetStatusResponse {}
pub struct BridgeQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> BridgeQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn last_transfer_height(
        &self,
        asset_id: ::core::option::Option<AssetId>,
    ) -> Result<LastTransferHeightResponse, cosmwasm_std::StdError> {
        LastTransferHeightRequest { asset_id }.query(self.querier)
    }
}
