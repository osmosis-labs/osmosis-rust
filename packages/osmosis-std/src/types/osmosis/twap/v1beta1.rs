use osmosis_std_derive::CosmwasmExt;
/// A TWAP record should be indexed in state by pool_id, (asset pair), timestamp
/// The asset pair assets should be lexicographically sorted.
/// Technically (pool_id, asset_0_denom, asset_1_denom, height) do not need to
/// appear in the struct however we view this as the wrong performance tradeoff
/// given SDK today. Would rather we optimize for readability and correctness,
/// than an optimal state storage format. The system bottleneck is elsewhere for
/// now.
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.twap.v1beta1.TwapRecord")]
pub struct TwapRecord {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    /// Lexicographically smaller denom of the pair
    #[prost(string, tag = "2")]
    pub asset0_denom: ::prost::alloc::string::String,
    /// Lexicographically larger denom of the pair
    #[prost(string, tag = "3")]
    pub asset1_denom: ::prost::alloc::string::String,
    /// height this record corresponds to, for debugging purposes
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    /// This field should only exist until we have a global registry in the state
    /// machine, mapping prior block heights within {TIME RANGE} to times.
    #[prost(message, optional, tag = "5")]
    pub time: ::core::option::Option<crate::shim::Timestamp>,
    /// We store the last spot prices in the struct, so that we can interpolate
    /// accumulator values for times between when accumulator records are stored.
    #[prost(string, tag = "6")]
    pub p0_last_spot_price: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub p1_last_spot_price: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub p0_arithmetic_twap_accumulator: ::prost::alloc::string::String,
    /// string geometric_twap_accumulator = 7 [(gogoproto.customtype) =
    /// "github.com/cosmos/cosmos-sdk/types.Dec",
    /// (gogoproto.nullable) = false];
    #[prost(string, tag = "9")]
    pub p1_arithmetic_twap_accumulator: ::prost::alloc::string::String,
}
/// GenesisState defines the gamm module's genesis state.
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.twap.v1beta1.GenesisState")]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub twaps: ::prost::alloc::vec::Vec<TwapRecord>,
}
