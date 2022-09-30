use osmosis_std_derive::CosmwasmExt;
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.MsgCreateSale")]
pub struct MsgCreateSale {
    /// Sale creator and the account which provides token (token_out) to the sale.
    /// When processing this message, token_out
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    /// token_in is a denom used to buy `token_out`. May be referred as a
    /// "quote currency".
    #[prost(string, tag = "2")]
    pub token_in: ::prost::alloc::string::String,
    /// token_out is a coin supply (denom + amount) to sell. May be referred as
    /// "base currency". The whole supply will be transferred from the creator
    /// to the module and will be sold during the sale.
    #[prost(message, optional, tag = "3")]
    pub token_out: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// Maximum fee the creator is going to pay for creating a sale. The creator
    /// will be charged params.SaleCreationFee. Transaction will fail if
    /// max_fee is smaller than params.SaleCreationFee. If empty, the creator
    /// doesn't accept any fee.
    #[prost(message, repeated, tag = "4")]
    pub max_fee: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// start time when the token sale starts.
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<crate::shim::Timestamp>,
    /// duration time that the sale takes place over
    #[prost(message, optional, tag = "6")]
    pub duration: ::core::option::Option<crate::shim::Duration>,
    /// Recipient is the account which receives earned `token_in` from when the
    /// sale is finalized. If not defined (empty) the creator
    /// account will be used.
    #[prost(string, tag = "7")]
    pub recipient: ::prost::alloc::string::String,
    /// Name for the sale, max 40 characters, min 4. Required.
    #[prost(string, tag = "8")]
    pub name: ::prost::alloc::string::String,
    /// URL with sale and project details. Can be a link a link to IPFS,
    /// hackmd, project page, blog post... Max 120 characters. Must be
    /// valid agains Go url.ParseRequestURI. Required.
    #[prost(string, tag = "9")]
    pub url: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.MsgCreateSaleResponse")]
pub struct MsgCreateSaleResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sale_id: u64,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.MsgSubscribe")]
pub struct MsgSubscribe {
    /// sender is an account address adding a deposit
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// ID of an existing sale.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sale_id: u64,
    /// number of sale.token_in staked by a user.
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.MsgWithdraw")]
pub struct MsgWithdraw {
    /// sender is an account address subscribed to the sale_id
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// ID of a sale.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sale_id: u64,
    /// amount of tokens_in to withdraw. Must be at most the amount of not spent
    /// tokens, unless set to null - then all remaining balance will be withdrawn.
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.MsgExitSale")]
pub struct MsgExitSale {
    /// sender is an account address exiting a sale
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// ID of an existing sale.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sale_id: u64,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.MsgExitSaleResponse")]
pub struct MsgExitSaleResponse {
    /// Purchased amount of "out" tokens withdrawn to the user.
    #[prost(string, tag = "1")]
    pub purchased: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.MsgFinalizeSale")]
pub struct MsgFinalizeSale {
    /// sender is an account signing the message and triggering the finalization.
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// ID of an existing sale.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sale_id: u64,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.MsgFinalizeSaleResponse")]
pub struct MsgFinalizeSaleResponse {
    /// Income amount of token_in sent to the sale recipient.
    #[prost(string, tag = "1")]
    pub income: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.Sale")]
pub struct Sale {
    /// Destination for the earned token_in
    #[prost(string, tag = "1")]
    pub treasury: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// token_out is a token denom to be bootstraped. May be referred as base
    /// currency, or a sale token.
    #[prost(string, tag = "3")]
    pub token_out: ::prost::alloc::string::String,
    /// token_in is a token denom used to buy sale tokens (`token_out`). May be
    /// referred as quote_currency or payment token.
    #[prost(string, tag = "4")]
    pub token_in: ::prost::alloc::string::String,
    /// total number of `tokens_out` to be sold during the continuous sale.
    #[prost(string, tag = "5")]
    pub token_out_supply: ::prost::alloc::string::String,
    /// start time when the token emission starts.
    #[prost(message, optional, tag = "6")]
    pub start_time: ::core::option::Option<crate::shim::Timestamp>,
    /// end time when the token emission ends. Can't be bigger than start +
    /// 139years (to avoid round overflow)
    #[prost(message, optional, tag = "7")]
    pub end_time: ::core::option::Option<crate::shim::Timestamp>,
    /// Round number when the sale was last time updated.
    #[prost(int64, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub round: i64,
    /// Last round of the Sale;
    #[prost(int64, tag = "9")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub end_round: i64,
    /// amout of remaining token_out to sell
    #[prost(string, tag = "10")]
    pub out_remaining: ::prost::alloc::string::String,
    /// amount of token_out sold
    #[prost(string, tag = "11")]
    pub out_sold: ::prost::alloc::string::String,
    /// out token per share
    #[prost(string, tag = "12")]
    pub out_per_share: ::prost::alloc::string::String,
    /// total amount of currently staked coins (token_in) but not spent coins.
    #[prost(string, tag = "13")]
    pub staked: ::prost::alloc::string::String,
    /// total amount of earned coins (token_in)
    #[prost(string, tag = "14")]
    pub income: ::prost::alloc::string::String,
    /// total amount of shares
    #[prost(string, tag = "15")]
    pub shares: ::prost::alloc::string::String,
    /// Name for the sale.
    #[prost(string, tag = "20")]
    pub name: ::prost::alloc::string::String,
    /// URL with sale and project details.
    #[prost(string, tag = "21")]
    pub url: ::prost::alloc::string::String,
}
/// UserPosition represents user account in a sale
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.UserPosition")]
pub struct UserPosition {
    #[prost(string, tag = "1")]
    pub shares: ::prost::alloc::string::String,
    /// total number of currently staked tokens
    #[prost(string, tag = "2")]
    pub staked: ::prost::alloc::string::String,
    /// last token/share ratio
    #[prost(string, tag = "3")]
    pub out_per_share: ::prost::alloc::string::String,
    /// amount of token_in spent
    #[prost(string, tag = "4")]
    pub spent: ::prost::alloc::string::String,
    /// Amount of accumulated, not withdrawn, purchased tokens (token_out)
    #[prost(string, tag = "5")]
    pub purchased: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.QuerySales")]
#[proto_query(
    path = "/osmosis.streamswap.v1.Query/Sales",
    response_type = QuerySalesResponse
)]
pub struct QuerySales {
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.QuerySalesResponse")]
pub struct QuerySalesResponse {
    #[prost(message, repeated, tag = "1")]
    pub sales: ::prost::alloc::vec::Vec<Sale>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// Request type for Query/Sale
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.QuerySale")]
#[proto_query(
    path = "/osmosis.streamswap.v1.Query/Sale",
    response_type = QuerySaleResponse
)]
pub struct QuerySale {
    /// Sale ID
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sale_id: u64,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.QuerySaleResponse")]
pub struct QuerySaleResponse {
    #[prost(message, optional, tag = "1")]
    pub sale: ::core::option::Option<Sale>,
}
/// Request type for Query/Sale
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.QueryUserPosition")]
#[proto_query(
    path = "/osmosis.streamswap.v1.Query/UserPosition",
    response_type = QueryUserPositionResponse
)]
pub struct QueryUserPosition {
    /// ID of the Sale
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sale_id: u64,
    /// user account address
    #[prost(string, tag = "2")]
    pub user: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.QueryUserPositionResponse")]
pub struct QueryUserPositionResponse {
    #[prost(message, optional, tag = "1")]
    pub user_position: ::core::option::Option<UserPosition>,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.EventCreateSale")]
pub struct EventCreateSale {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub token_out: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.EventSubscribe")]
pub struct EventSubscribe {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sale_id: u64,
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.EventWithdraw")]
pub struct EventWithdraw {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sale_id: u64,
    /// amount of staked token_in withdrawn by user.
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.EventExit")]
pub struct EventExit {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sale_id: u64,
    /// amount of purchased token_out sent to the user
    #[prost(string, tag = "3")]
    pub purchased: ::prost::alloc::string::String,
}
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.EventFinalizeSale")]
pub struct EventFinalizeSale {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sale_id: u64,
    /// amount of earned tokens_in
    #[prost(string, tag = "3")]
    pub income: ::prost::alloc::string::String,
}
/// Params holds parameters for the streamswap module
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.Params")]
pub struct Params {
    /// fee charged when creating a new sale. The fee will go to the
    /// sale_fee_recipient unless it is not defined (empty).
    #[prost(message, repeated, tag = "1")]
    pub sale_creation_fee:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// bech32 address of the fee recipient
    #[prost(string, tag = "2")]
    pub sale_creation_fee_recipient: ::prost::alloc::string::String,
    /// minimum amount duration of time between the sale creation and the sale
    /// start time.
    #[prost(message, optional, tag = "3")]
    pub min_duration_until_start_time: ::core::option::Option<crate::shim::Duration>,
    /// minimum duration for every new sale.
    #[prost(message, optional, tag = "4")]
    pub min_sale_duration: ::core::option::Option<crate::shim::Duration>,
}
/// GenesisState defines the streamswap module's genesis state.
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.GenesisState")]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub sales: ::prost::alloc::vec::Vec<Sale>,
    #[prost(message, repeated, tag = "2")]
    pub user_positions: ::prost::alloc::vec::Vec<UserPositionKv>,
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub next_sale_id: u64,
    #[prost(message, optional, tag = "4")]
    pub params: ::core::option::Option<Params>,
}
/// UserPositionKV is a record in genesis representing acc_address user position
/// of a sale_id sale.
#[derive(
    Clone,
    PartialEq, Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.streamswap.v1.UserPositionKV")]
pub struct UserPositionKv {
    /// user account address
    #[prost(string, tag = "1")]
    pub acc_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub sale_id: u64,
    #[prost(message, optional, tag = "3")]
    pub user_position: ::core::option::Option<UserPosition>,
}
pub struct StreamswapQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> StreamswapQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn sales(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QuerySalesResponse, cosmwasm_std::StdError> {
        QuerySales { pagination }.query(self.querier)
    }
    pub fn sale(&self, sale_id: u64) -> Result<QuerySaleResponse, cosmwasm_std::StdError> {
        QuerySale { sale_id }.query(self.querier)
    }
    pub fn user_position(
        &self,
        sale_id: u64,
        user: ::prost::alloc::string::String,
    ) -> Result<QueryUserPositionResponse, cosmwasm_std::StdError> {
        QueryUserPosition { sale_id, user }.query(self.querier)
    }
}
