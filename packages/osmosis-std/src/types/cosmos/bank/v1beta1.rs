use osmosis_std_derive::CosmwasmExt;
/// SendAuthorization allows the grantee to spend up to spend_limit coins from
/// the granter's account.
///
/// Since: cosmos-sdk 0.43
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.SendAuthorization")]
pub struct SendAuthorization {
    #[prost(message, repeated, tag = "1")]
    pub spend_limit: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// allow_list specifies an optional list of addresses to whom the grantee can send tokens on behalf of the
    /// granter. If omitted, any recipient is allowed.
    ///
    /// Since: cosmos-sdk 0.47
    #[prost(string, repeated, tag = "2")]
    pub allow_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Params defines the parameters for the bank module.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.Params")]
pub struct Params {
    /// Deprecated: Use of SendEnabled in params is deprecated.
    /// For genesis, use the newly added send_enabled field in the genesis object.
    /// Storage, lookup, and manipulation of this information is now in the keeper.
    ///
    /// As of cosmos-sdk 0.47, this only exists for backwards compatibility of genesis files.
    #[deprecated]
    #[prost(message, repeated, tag = "1")]
    pub send_enabled: ::prost::alloc::vec::Vec<SendEnabled>,
    #[prost(bool, tag = "2")]
    pub default_send_enabled: bool,
}
/// SendEnabled maps coin denom to a send_enabled status (whether a denom is
/// sendable).
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.SendEnabled")]
pub struct SendEnabled {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub enabled: bool,
}
/// Input models transaction input.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.Input")]
pub struct Input {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// Output models transaction outputs.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.Output")]
pub struct Output {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// Supply represents a struct that passively keeps track of the total supply
/// amounts in the network.
/// This message is deprecated now that supply is indexed by denom.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.Supply")]
#[deprecated]
pub struct Supply {
    #[prost(message, repeated, tag = "1")]
    pub total: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// DenomUnit represents a struct that describes a given
/// denomination unit of the basic token.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.DenomUnit")]
pub struct DenomUnit {
    /// denom represents the string name of the given denom unit (e.g uatom).
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// exponent represents power of 10 exponent that one must
    /// raise the base_denom to in order to equal the given DenomUnit's denom
    /// 1 denom = 10^exponent base_denom
    /// (e.g. with a base_denom of uatom, one can create a DenomUnit of 'atom' with
    /// exponent = 6, thus: 1 atom = 10^6 uatom).
    #[prost(uint32, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub exponent: u32,
    /// aliases is a list of string aliases for the given denom
    #[prost(string, repeated, tag = "3")]
    pub aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Metadata represents a struct that describes
/// a basic token.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.Metadata")]
pub struct Metadata {
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    /// denom_units represents the list of DenomUnit's for a given coin
    #[prost(message, repeated, tag = "2")]
    pub denom_units: ::prost::alloc::vec::Vec<DenomUnit>,
    /// base represents the base denom (should be the DenomUnit with exponent = 0).
    #[prost(string, tag = "3")]
    pub base: ::prost::alloc::string::String,
    /// display indicates the suggested denom that should be
    /// displayed in clients.
    #[prost(string, tag = "4")]
    pub display: ::prost::alloc::string::String,
    /// name defines the name of the token (eg: Cosmos Atom)
    ///
    /// Since: cosmos-sdk 0.43
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    /// symbol is the token symbol usually shown on exchanges (eg: ATOM). This can
    /// be the same as the display.
    ///
    /// Since: cosmos-sdk 0.43
    #[prost(string, tag = "6")]
    pub symbol: ::prost::alloc::string::String,
    /// URI to a document (on or off-chain) that contains additional information. Optional.
    ///
    /// Since: cosmos-sdk 0.46
    #[prost(string, tag = "7")]
    pub uri: ::prost::alloc::string::String,
    /// URIHash is a sha256 hash of a document pointed by URI. It's used to verify that
    /// the document didn't change. Optional.
    ///
    /// Since: cosmos-sdk 0.46
    #[prost(string, tag = "8")]
    pub uri_hash: ::prost::alloc::string::String,
}
/// GenesisState defines the bank module's genesis state.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// balances is an array containing the balances of all the accounts.
    #[prost(message, repeated, tag = "2")]
    pub balances: ::prost::alloc::vec::Vec<Balance>,
    /// supply represents the total supply. If it is left empty, then supply will be calculated based on the provided
    /// balances. Otherwise, it will be used to validate that the sum of the balances equals this amount.
    #[prost(message, repeated, tag = "3")]
    pub supply: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// denom_metadata defines the metadata of the different coins.
    #[prost(message, repeated, tag = "4")]
    pub denom_metadata: ::prost::alloc::vec::Vec<Metadata>,
    /// send_enabled defines the denoms where send is enabled or disabled.
    ///
    /// Since: cosmos-sdk 0.47
    #[prost(message, repeated, tag = "5")]
    pub send_enabled: ::prost::alloc::vec::Vec<SendEnabled>,
}
/// Balance defines an account address and balance pair used in the bank module's
/// genesis state.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.Balance")]
pub struct Balance {
    /// address is the address of the balance holder.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// coins defines the different coins this balance holds.
    #[prost(message, repeated, tag = "2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// QueryBalanceRequest is the request type for the Query/Balance RPC method.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryBalanceRequest")]
#[proto_query(
    path = "/cosmos.bank.v1beta1.Query/Balance",
    response_type = QueryBalanceResponse
)]
pub struct QueryBalanceRequest {
    /// address is the address to query balances for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// denom is the coin denom to query balances for.
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryBalanceResponse is the response type for the Query/Balance RPC method.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryBalanceResponse")]
pub struct QueryBalanceResponse {
    /// balance is the balance of the coin.
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// QueryBalanceRequest is the request type for the Query/AllBalances RPC method.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryAllBalancesRequest")]
#[proto_query(
    path = "/cosmos.bank.v1beta1.Query/AllBalances",
    response_type = QueryAllBalancesResponse
)]
pub struct QueryAllBalancesRequest {
    /// address is the address to query balances for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryAllBalancesResponse is the response type for the Query/AllBalances RPC
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryAllBalancesResponse")]
pub struct QueryAllBalancesResponse {
    /// balances is the balances of all the coins.
    #[prost(message, repeated, tag = "1")]
    pub balances: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QuerySpendableBalancesRequest defines the gRPC request structure for querying
/// an account's spendable balances.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QuerySpendableBalancesRequest")]
#[proto_query(
    path = "/cosmos.bank.v1beta1.Query/SpendableBalances",
    response_type = QuerySpendableBalancesResponse
)]
pub struct QuerySpendableBalancesRequest {
    /// address is the address to query spendable balances for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QuerySpendableBalancesResponse defines the gRPC response structure for querying
/// an account's spendable balances.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QuerySpendableBalancesResponse")]
pub struct QuerySpendableBalancesResponse {
    /// balances is the spendable balances of all the coins.
    #[prost(message, repeated, tag = "1")]
    pub balances: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QuerySpendableBalanceByDenomRequest defines the gRPC request structure for
/// querying an account's spendable balance for a specific denom.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QuerySpendableBalanceByDenomRequest")]
#[proto_query(
    path = "/cosmos.bank.v1beta1.Query/SpendableBalanceByDenom",
    response_type = QuerySpendableBalanceByDenomResponse
)]
pub struct QuerySpendableBalanceByDenomRequest {
    /// address is the address to query balances for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// denom is the coin denom to query balances for.
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
}
/// QuerySpendableBalanceByDenomResponse defines the gRPC response structure for
/// querying an account's spendable balance for a specific denom.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QuerySpendableBalanceByDenomResponse")]
pub struct QuerySpendableBalanceByDenomResponse {
    /// balance is the balance of the coin.
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// QueryTotalSupplyRequest is the request type for the Query/TotalSupply RPC
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryTotalSupplyRequest")]
#[proto_query(
    path = "/cosmos.bank.v1beta1.Query/TotalSupply",
    response_type = QueryTotalSupplyResponse
)]
pub struct QueryTotalSupplyRequest {
    /// pagination defines an optional pagination for the request.
    ///
    /// Since: cosmos-sdk 0.43
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryTotalSupplyResponse is the response type for the Query/TotalSupply RPC
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryTotalSupplyResponse")]
pub struct QueryTotalSupplyResponse {
    /// supply is the supply of the coins
    #[prost(message, repeated, tag = "1")]
    pub supply: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// pagination defines the pagination in the response.
    ///
    /// Since: cosmos-sdk 0.43
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QuerySupplyOfRequest is the request type for the Query/SupplyOf RPC method.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QuerySupplyOfRequest")]
#[proto_query(
    path = "/cosmos.bank.v1beta1.Query/SupplyOf",
    response_type = QuerySupplyOfResponse
)]
pub struct QuerySupplyOfRequest {
    /// denom is the coin denom to query balances for.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QuerySupplyOfResponse is the response type for the Query/SupplyOf RPC method.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QuerySupplyOfResponse")]
pub struct QuerySupplyOfResponse {
    /// amount is the supply of the coin.
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// QueryTotalSupplyWithoutOffsetRequest is the request type for the Query/TotalSupplyWithoutOffset RPC
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryTotalSupplyWithoutOffsetRequest")]
#[proto_query(
    path = "/cosmos.bank.v1beta1.Query/TotalSupplyWithoutOffset",
    response_type = QueryTotalSupplyWithoutOffsetResponse
)]
pub struct QueryTotalSupplyWithoutOffsetRequest {
    /// pagination defines an optional pagination for the request.
    ///
    /// Since: cosmos-sdk 0.43
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryTotalSupplyWithoutOffsetResponse is the response type for the Query/TotalSupplyWithoutOffset RPC
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryTotalSupplyWithoutOffsetResponse")]
pub struct QueryTotalSupplyWithoutOffsetResponse {
    /// supply is the supply of the coins
    #[prost(message, repeated, tag = "1")]
    pub supply: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// pagination defines the pagination in the response.
    ///
    /// Since: cosmos-sdk 0.43
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QuerySupplyOfWithoutOffsetRequest is the request type for the Query/SupplyOfWithoutOffset RPC method.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QuerySupplyOfWithoutOffsetRequest")]
#[proto_query(
    path = "/cosmos.bank.v1beta1.Query/SupplyOfWithoutOffset",
    response_type = QuerySupplyOfWithoutOffsetResponse
)]
pub struct QuerySupplyOfWithoutOffsetRequest {
    /// denom is the coin denom to query balances for.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QuerySupplyOfWithoutOffsetResponse is the response type for the Query/SupplyOfWithoutOffset RPC method.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QuerySupplyOfWithoutOffsetResponse")]
pub struct QuerySupplyOfWithoutOffsetResponse {
    /// amount is the supply of the coin.
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// QueryParamsRequest defines the request type for querying x/bank parameters.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/cosmos.bank.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse defines the response type for querying x/bank parameters.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryDenomsMetadataRequest is the request type for the Query/DenomsMetadata RPC method.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryDenomsMetadataRequest")]
#[proto_query(
    path = "/cosmos.bank.v1beta1.Query/DenomsMetadata",
    response_type = QueryDenomsMetadataResponse
)]
pub struct QueryDenomsMetadataRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryDenomsMetadataResponse is the response type for the Query/DenomsMetadata RPC
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryDenomsMetadataResponse")]
pub struct QueryDenomsMetadataResponse {
    /// metadata provides the client information for all the registered tokens.
    #[prost(message, repeated, tag = "1")]
    pub metadatas: ::prost::alloc::vec::Vec<Metadata>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryDenomMetadataRequest is the request type for the Query/DenomMetadata RPC method.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryDenomMetadataRequest")]
#[proto_query(
    path = "/cosmos.bank.v1beta1.Query/DenomMetadata",
    response_type = QueryDenomMetadataResponse
)]
pub struct QueryDenomMetadataRequest {
    /// denom is the coin denom to query the metadata for.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryDenomMetadataResponse is the response type for the Query/DenomMetadata RPC
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryDenomMetadataResponse")]
pub struct QueryDenomMetadataResponse {
    /// metadata describes and provides all the client information for the requested token.
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<Metadata>,
}
/// QueryDenomOwnersRequest defines the request type for the DenomOwners RPC query,
/// which queries for a paginated set of all account holders of a particular
/// denomination.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryDenomOwnersRequest")]
#[proto_query(
    path = "/cosmos.bank.v1beta1.Query/DenomOwners",
    response_type = QueryDenomOwnersResponse
)]
pub struct QueryDenomOwnersRequest {
    /// denom defines the coin denomination to query all account holders for.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// DenomOwner defines structure representing an account that owns or holds a
/// particular denominated token. It contains the account address and account
/// balance of the denominated token.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.DenomOwner")]
pub struct DenomOwner {
    /// address defines the address that owns a particular denomination.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// balance is the balance of the denominated coin for an account.
    #[prost(message, optional, tag = "2")]
    pub balance: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// QueryDenomOwnersResponse defines the RPC response of a DenomOwners RPC query.
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QueryDenomOwnersResponse")]
pub struct QueryDenomOwnersResponse {
    #[prost(message, repeated, tag = "1")]
    pub denom_owners: ::prost::alloc::vec::Vec<DenomOwner>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QuerySendEnabledRequest defines the RPC request for looking up SendEnabled entries.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QuerySendEnabledRequest")]
#[proto_query(
    path = "/cosmos.bank.v1beta1.Query/SendEnabled",
    response_type = QuerySendEnabledResponse
)]
pub struct QuerySendEnabledRequest {
    /// denoms is the specific denoms you want look up. Leave empty to get all entries.
    #[prost(string, repeated, tag = "1")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// pagination defines an optional pagination for the request. This field is
    /// only read if the denoms field is empty.
    #[prost(message, optional, tag = "99")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QuerySendEnabledResponse defines the RPC response of a SendEnable query.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.QuerySendEnabledResponse")]
pub struct QuerySendEnabledResponse {
    #[prost(message, repeated, tag = "1")]
    pub send_enabled: ::prost::alloc::vec::Vec<SendEnabled>,
    /// pagination defines the pagination in the response. This field is only
    /// populated if the denoms field in the request is empty.
    #[prost(message, optional, tag = "99")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// MsgSend represents a message to send coins from one account to another.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.MsgSend")]
pub struct MsgSend {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// MsgSendResponse defines the Msg/Send response type.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.MsgSendResponse")]
pub struct MsgSendResponse {}
/// MsgMultiSend represents an arbitrary multi-in, multi-out send message.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.MsgMultiSend")]
pub struct MsgMultiSend {
    /// Inputs, despite being `repeated`, only allows one sender input. This is
    /// checked in MsgMultiSend's ValidateBasic.
    #[prost(message, repeated, tag = "1")]
    pub inputs: ::prost::alloc::vec::Vec<Input>,
    #[prost(message, repeated, tag = "2")]
    pub outputs: ::prost::alloc::vec::Vec<Output>,
}
/// MsgMultiSendResponse defines the Msg/MultiSend response type.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.MsgMultiSendResponse")]
pub struct MsgMultiSendResponse {}
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/bank parameters to update.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
/// MsgSetSendEnabled is the Msg/SetSendEnabled request type.
///
/// Only entries to add/update/delete need to be included.
/// Existing SendEnabled entries that are not included in this
/// message are left unchanged.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.MsgSetSendEnabled")]
pub struct MsgSetSendEnabled {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// send_enabled is the list of entries to add or update.
    #[prost(message, repeated, tag = "2")]
    pub send_enabled: ::prost::alloc::vec::Vec<SendEnabled>,
    /// use_default_for is a list of denoms that should use the params.default_send_enabled value.
    /// Denoms listed here will have their SendEnabled entries deleted.
    /// If a denom is included that doesn't have a SendEnabled entry,
    /// it will be ignored.
    #[prost(string, repeated, tag = "3")]
    pub use_default_for: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgSetSendEnabledResponse defines the Msg/SetSendEnabled response type.
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
#[proto_message(type_url = "/cosmos.bank.v1beta1.MsgSetSendEnabledResponse")]
pub struct MsgSetSendEnabledResponse {}
pub struct BankQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> BankQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn balance(
        &self,
        address: ::prost::alloc::string::String,
        denom: ::prost::alloc::string::String,
    ) -> Result<QueryBalanceResponse, cosmwasm_std::StdError> {
        QueryBalanceRequest { address, denom }.query(self.querier)
    }
    pub fn all_balances(
        &self,
        address: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryAllBalancesResponse, cosmwasm_std::StdError> {
        QueryAllBalancesRequest {
            address,
            pagination,
        }
        .query(self.querier)
    }
    pub fn spendable_balances(
        &self,
        address: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QuerySpendableBalancesResponse, cosmwasm_std::StdError> {
        QuerySpendableBalancesRequest {
            address,
            pagination,
        }
        .query(self.querier)
    }
    pub fn spendable_balance_by_denom(
        &self,
        address: ::prost::alloc::string::String,
        denom: ::prost::alloc::string::String,
    ) -> Result<QuerySpendableBalanceByDenomResponse, cosmwasm_std::StdError> {
        QuerySpendableBalanceByDenomRequest { address, denom }.query(self.querier)
    }
    pub fn total_supply(
        &self,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryTotalSupplyResponse, cosmwasm_std::StdError> {
        QueryTotalSupplyRequest { pagination }.query(self.querier)
    }
    pub fn supply_of(
        &self,
        denom: ::prost::alloc::string::String,
    ) -> Result<QuerySupplyOfResponse, cosmwasm_std::StdError> {
        QuerySupplyOfRequest { denom }.query(self.querier)
    }
    pub fn total_supply_without_offset(
        &self,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryTotalSupplyWithoutOffsetResponse, cosmwasm_std::StdError> {
        QueryTotalSupplyWithoutOffsetRequest { pagination }.query(self.querier)
    }
    pub fn supply_of_without_offset(
        &self,
        denom: ::prost::alloc::string::String,
    ) -> Result<QuerySupplyOfWithoutOffsetResponse, cosmwasm_std::StdError> {
        QuerySupplyOfWithoutOffsetRequest { denom }.query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn denom_metadata(
        &self,
        denom: ::prost::alloc::string::String,
    ) -> Result<QueryDenomMetadataResponse, cosmwasm_std::StdError> {
        QueryDenomMetadataRequest { denom }.query(self.querier)
    }
    pub fn denoms_metadata(
        &self,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryDenomsMetadataResponse, cosmwasm_std::StdError> {
        QueryDenomsMetadataRequest { pagination }.query(self.querier)
    }
    pub fn denom_owners(
        &self,
        denom: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryDenomOwnersResponse, cosmwasm_std::StdError> {
        QueryDenomOwnersRequest { denom, pagination }.query(self.querier)
    }
    pub fn send_enabled(
        &self,
        denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QuerySendEnabledResponse, cosmwasm_std::StdError> {
        QuerySendEnabledRequest { denoms, pagination }.query(self.querier)
    }
}
