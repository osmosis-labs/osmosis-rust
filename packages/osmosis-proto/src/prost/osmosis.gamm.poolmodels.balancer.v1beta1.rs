/// ===================== MsgCreatePool
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBalancerPool {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub pool_params: ::core::option::Option<super::super::super::v1beta1::PoolParams>,
    #[prost(message, repeated, tag="3")]
    pub pool_assets: ::prost::alloc::vec::Vec<super::super::super::v1beta1::PoolAsset>,
    #[prost(string, tag="4")]
    pub future_pool_governor: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateBalancerPoolResponse {
    #[prost(uint64, tag="1")]
    pub pool_id: u64,
}
