use osmosis_std_derive::CosmwasmExt;
/// PeriodLock is a single unit of lock by period. It's a record of locked coin
/// at a specific time. It stores owner, duration, unlock time and the amount of
/// coins locked.
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
#[proto_message(type_url = "/osmosis.lockup.PeriodLock")]
pub struct PeriodLock {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<crate::shim::Duration>,
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(message, repeated, tag = "5")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
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
#[proto_message(type_url = "/osmosis.lockup.QueryCondition")]
pub struct QueryCondition {
    /// type of lock query, ByLockDuration | ByLockTime
    #[prost(enumeration = "LockQueryType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub lock_query_type: i32,
    /// What token denomination are we looking for lockups of
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    /// valid when query condition is ByDuration
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<crate::shim::Duration>,
    /// valid when query condition is ByTime
    #[prost(message, optional, tag = "4")]
    pub timestamp: ::core::option::Option<crate::shim::Timestamp>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LockQueryType {
    /// Queries for locks that are longer than a certain duration
    ByDuration = 0,
    /// Queries for lockups that started before a specific time
    ByTime = 1,
}
/// GenesisState defines the lockup module's genesis state.
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
#[proto_message(type_url = "/osmosis.lockup.GenesisState")]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
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
#[proto_message(type_url = "/osmosis.lockup.MsgLockTokens")]
pub struct MsgLockTokens {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<crate::shim::Duration>,
    #[prost(message, repeated, tag = "3")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
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
#[proto_message(type_url = "/osmosis.lockup.MsgLockTokensResponse")]
pub struct MsgLockTokensResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
}
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
#[proto_message(type_url = "/osmosis.lockup.MsgBeginUnlockingAll")]
pub struct MsgBeginUnlockingAll {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/osmosis.lockup.MsgBeginUnlockingAllResponse")]
pub struct MsgBeginUnlockingAllResponse {
    #[prost(message, repeated, tag = "1")]
    pub unlocks: ::prost::alloc::vec::Vec<PeriodLock>,
}
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
#[proto_message(type_url = "/osmosis.lockup.MsgUnlockTokens")]
pub struct MsgUnlockTokens {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/osmosis.lockup.MsgUnlockTokensResponse")]
pub struct MsgUnlockTokensResponse {
    #[prost(message, repeated, tag = "1")]
    pub unlocks: ::prost::alloc::vec::Vec<PeriodLock>,
}
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
#[proto_message(type_url = "/osmosis.lockup.MsgBeginUnlocking")]
pub struct MsgBeginUnlocking {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
}
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
#[proto_message(type_url = "/osmosis.lockup.MsgBeginUnlockingResponse")]
pub struct MsgBeginUnlockingResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
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
#[proto_message(type_url = "/osmosis.lockup.MsgUnlockPeriodLock")]
pub struct MsgUnlockPeriodLock {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
}
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
#[proto_message(type_url = "/osmosis.lockup.MsgUnlockPeriodLockResponse")]
pub struct MsgUnlockPeriodLockResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
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
#[proto_message(type_url = "/osmosis.lockup.ModuleBalanceRequest")]
#[proto_query(
    path = "/osmosis.lockup.Query/ModuleBalance",
    response_type = ModuleBalanceResponse
)]
pub struct ModuleBalanceRequest {}
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
#[proto_message(type_url = "/osmosis.lockup.ModuleBalanceResponse")]
pub struct ModuleBalanceResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
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
#[proto_message(type_url = "/osmosis.lockup.ModuleLockedAmountRequest")]
#[proto_query(
    path = "/osmosis.lockup.Query/ModuleLockedAmount",
    response_type = ModuleLockedAmountResponse
)]
pub struct ModuleLockedAmountRequest {}
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
#[proto_message(type_url = "/osmosis.lockup.ModuleLockedAmountResponse")]
pub struct ModuleLockedAmountResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountUnlockableCoinsRequest")]
#[proto_query(
    path = "/osmosis.lockup.Query/AccountUnlockableCoins",
    response_type = AccountUnlockableCoinsResponse
)]
pub struct AccountUnlockableCoinsRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountUnlockableCoinsResponse")]
pub struct AccountUnlockableCoinsResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountUnlockingCoinsRequest")]
#[proto_query(
    path = "/osmosis.lockup.Query/AccountUnlockingCoins",
    response_type = AccountUnlockingCoinsResponse
)]
pub struct AccountUnlockingCoinsRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountUnlockingCoinsResponse")]
pub struct AccountUnlockingCoinsResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountLockedCoinsRequest")]
#[proto_query(
    path = "/osmosis.lockup.Query/AccountLockedCoins",
    response_type = AccountLockedCoinsResponse
)]
pub struct AccountLockedCoinsRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountLockedCoinsResponse")]
pub struct AccountLockedCoinsResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountLockedPastTimeRequest")]
#[proto_query(
    path = "/osmosis.lockup.Query/AccountLockedPastTime",
    response_type = AccountLockedPastTimeResponse
)]
pub struct AccountLockedPastTimeRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<crate::shim::Timestamp>,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountLockedPastTimeResponse")]
pub struct AccountLockedPastTimeResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountLockedPastTimeNotUnlockingOnlyRequest")]
#[proto_query(
    path = "/osmosis.lockup.Query/AccountLockedPastTimeNotUnlockingOnly",
    response_type = AccountLockedPastTimeNotUnlockingOnlyResponse
)]
pub struct AccountLockedPastTimeNotUnlockingOnlyRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<crate::shim::Timestamp>,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountLockedPastTimeNotUnlockingOnlyResponse")]
pub struct AccountLockedPastTimeNotUnlockingOnlyResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountUnlockedBeforeTimeRequest")]
#[proto_query(
    path = "/osmosis.lockup.Query/AccountUnlockedBeforeTime",
    response_type = AccountUnlockedBeforeTimeResponse
)]
pub struct AccountUnlockedBeforeTimeRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<crate::shim::Timestamp>,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountUnlockedBeforeTimeResponse")]
pub struct AccountUnlockedBeforeTimeResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountLockedPastTimeDenomRequest")]
#[proto_query(
    path = "/osmosis.lockup.Query/AccountLockedPastTimeDenom",
    response_type = AccountLockedPastTimeDenomResponse
)]
pub struct AccountLockedPastTimeDenomRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountLockedPastTimeDenomResponse")]
pub struct AccountLockedPastTimeDenomResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
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
#[proto_message(type_url = "/osmosis.lockup.LockedRequest")]
#[proto_query(path = "/osmosis.lockup.Query/LockedByID", response_type = LockedResponse)]
pub struct LockedRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub lock_id: u64,
}
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
#[proto_message(type_url = "/osmosis.lockup.LockedResponse")]
pub struct LockedResponse {
    #[prost(message, optional, tag = "1")]
    pub lock: ::core::option::Option<PeriodLock>,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountLockedLongerDurationRequest")]
#[proto_query(
    path = "/osmosis.lockup.Query/AccountLockedLongerDuration",
    response_type = AccountLockedLongerDurationResponse
)]
pub struct AccountLockedLongerDurationRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<crate::shim::Duration>,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountLockedLongerDurationResponse")]
pub struct AccountLockedLongerDurationResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountLockedLongerDurationNotUnlockingOnlyRequest")]
#[proto_query(
    path = "/osmosis.lockup.Query/AccountLockedLongerDurationNotUnlockingOnly",
    response_type = AccountLockedLongerDurationNotUnlockingOnlyResponse
)]
pub struct AccountLockedLongerDurationNotUnlockingOnlyRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<crate::shim::Duration>,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountLockedLongerDurationNotUnlockingOnlyResponse")]
pub struct AccountLockedLongerDurationNotUnlockingOnlyResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountLockedLongerDurationDenomRequest")]
#[proto_query(
    path = "/osmosis.lockup.Query/AccountLockedLongerDurationDenom",
    response_type = AccountLockedLongerDurationDenomResponse
)]
pub struct AccountLockedLongerDurationDenomRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<crate::shim::Duration>,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/osmosis.lockup.AccountLockedLongerDurationDenomResponse")]
pub struct AccountLockedLongerDurationDenomResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
pub struct LockupQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> LockupQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn module_balance(&self) -> Result<ModuleBalanceResponse, cosmwasm_std::StdError> {
        ModuleBalanceRequest {}.query(self.querier)
    }
    pub fn module_locked_amount(
        &self,
    ) -> Result<ModuleLockedAmountResponse, cosmwasm_std::StdError> {
        ModuleLockedAmountRequest {}.query(self.querier)
    }
    pub fn account_unlockable_coins(
        &self,
        owner: ::prost::alloc::string::String,
    ) -> Result<AccountUnlockableCoinsResponse, cosmwasm_std::StdError> {
        AccountUnlockableCoinsRequest { owner }.query(self.querier)
    }
    pub fn account_unlocking_coins(
        &self,
        owner: ::prost::alloc::string::String,
    ) -> Result<AccountUnlockingCoinsResponse, cosmwasm_std::StdError> {
        AccountUnlockingCoinsRequest { owner }.query(self.querier)
    }
    pub fn account_locked_coins(
        &self,
        owner: ::prost::alloc::string::String,
    ) -> Result<AccountLockedCoinsResponse, cosmwasm_std::StdError> {
        AccountLockedCoinsRequest { owner }.query(self.querier)
    }
    pub fn account_locked_past_time(
        &self,
        owner: ::prost::alloc::string::String,
        timestamp: ::core::option::Option<crate::shim::Timestamp>,
    ) -> Result<AccountLockedPastTimeResponse, cosmwasm_std::StdError> {
        AccountLockedPastTimeRequest { owner, timestamp }.query(self.querier)
    }
    pub fn account_locked_past_time_not_unlocking_only(
        &self,
        owner: ::prost::alloc::string::String,
        timestamp: ::core::option::Option<crate::shim::Timestamp>,
    ) -> Result<AccountLockedPastTimeNotUnlockingOnlyResponse, cosmwasm_std::StdError> {
        AccountLockedPastTimeNotUnlockingOnlyRequest { owner, timestamp }.query(self.querier)
    }
    pub fn account_unlocked_before_time(
        &self,
        owner: ::prost::alloc::string::String,
        timestamp: ::core::option::Option<crate::shim::Timestamp>,
    ) -> Result<AccountUnlockedBeforeTimeResponse, cosmwasm_std::StdError> {
        AccountUnlockedBeforeTimeRequest { owner, timestamp }.query(self.querier)
    }
    pub fn account_locked_past_time_denom(
        &self,
        owner: ::prost::alloc::string::String,
        timestamp: ::core::option::Option<crate::shim::Timestamp>,
        denom: ::prost::alloc::string::String,
    ) -> Result<AccountLockedPastTimeDenomResponse, cosmwasm_std::StdError> {
        AccountLockedPastTimeDenomRequest {
            owner,
            timestamp,
            denom,
        }
        .query(self.querier)
    }
    pub fn locked_by_id(&self, lock_id: u64) -> Result<LockedResponse, cosmwasm_std::StdError> {
        LockedRequest { lock_id }.query(self.querier)
    }
    pub fn account_locked_longer_duration(
        &self,
        owner: ::prost::alloc::string::String,
        duration: ::core::option::Option<crate::shim::Duration>,
    ) -> Result<AccountLockedLongerDurationResponse, cosmwasm_std::StdError> {
        AccountLockedLongerDurationRequest { owner, duration }.query(self.querier)
    }
    pub fn account_locked_longer_duration_not_unlocking_only(
        &self,
        owner: ::prost::alloc::string::String,
        duration: ::core::option::Option<crate::shim::Duration>,
    ) -> Result<AccountLockedLongerDurationNotUnlockingOnlyResponse, cosmwasm_std::StdError> {
        AccountLockedLongerDurationNotUnlockingOnlyRequest { owner, duration }.query(self.querier)
    }
    pub fn account_locked_longer_duration_denom(
        &self,
        owner: ::prost::alloc::string::String,
        duration: ::core::option::Option<crate::shim::Duration>,
        denom: ::prost::alloc::string::String,
    ) -> Result<AccountLockedLongerDurationDenomResponse, cosmwasm_std::StdError> {
        AccountLockedLongerDurationDenomRequest {
            owner,
            duration,
            denom,
        }
        .query(self.querier)
    }
}
