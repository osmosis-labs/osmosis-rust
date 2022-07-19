/// PeriodLock is a single unit of lock by period. It's a record of locked coin
/// at a specific time. It stores owner, duration, unlock time and the amount of
/// coins locked.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeriodLock {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, repeated, tag = "5")]
    pub coins: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl crate::cosmwasm::ToCosmosMsg for PeriodLock {
    const TYPE_URL: &'static str = "/osmosis.lockup.PeriodLock";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCondition {
    /// type of lock query, ByLockDuration | ByLockTime
    #[prost(enumeration = "LockQueryType", tag = "1")]
    pub lock_query_type: i32,
    /// What token denomination are we looking for lockups of
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    /// valid when query condition is ByDuration
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// valid when query condition is ByTime
    #[prost(message, optional, tag = "4")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
impl crate::cosmwasm::ToCosmosMsg for QueryCondition {
    const TYPE_URL: &'static str = "/osmosis.lockup.QueryCondition";
}
/// SyntheticLock is a single unit of synthetic lockup
/// TODO: Change this to have
/// * underlying_lock_id
/// * synthetic_coin
/// * end_time
/// * duration
/// * owner
/// We then index synthetic locks by the denom, just like we do with normal
/// locks. Ideally we even get an interface, so we can re-use that same logic.
/// I currently have no idea how reward distribution is supposed to be working...
/// EVENTUALLY
/// we make a "constrained_coin" field, which is what the current "coins" field
/// is. Constrained coin field can be a #post-v7 feature, since we aren't
/// allowing partial unlocks of synthetic lockups.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyntheticLock {
    /// underlying native lockup id for this synthetic lockup
    #[prost(uint64, tag = "1")]
    pub underlying_lock_id: u64,
    #[prost(string, tag = "2")]
    pub synth_denom: ::prost::alloc::string::String,
    /// used for unbonding synthetic lockups, for active synthetic lockups, this
    /// value is set to uninitialized value
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "4")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
impl crate::cosmwasm::ToCosmosMsg for SyntheticLock {
    const TYPE_URL: &'static str = "/osmosis.lockup.SyntheticLock";
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LockQueryType {
    /// Queries for locks that are longer than a certain duration
    ByDuration = 0,
    /// Queries for lockups that started before a specific time
    ByTime = 1,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLockTokens {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, repeated, tag = "3")]
    pub coins: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl crate::cosmwasm::ToCosmosMsg for MsgLockTokens {
    const TYPE_URL: &'static str = "/osmosis.lockup.MsgLockTokens";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLockTokensResponse {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
impl crate::cosmwasm::ToCosmosMsg for MsgLockTokensResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.MsgLockTokensResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBeginUnlockingAll {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
impl crate::cosmwasm::ToCosmosMsg for MsgBeginUnlockingAll {
    const TYPE_URL: &'static str = "/osmosis.lockup.MsgBeginUnlockingAll";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBeginUnlockingAllResponse {
    #[prost(message, repeated, tag = "1")]
    pub unlocks: ::prost::alloc::vec::Vec<PeriodLock>,
}
impl crate::cosmwasm::ToCosmosMsg for MsgBeginUnlockingAllResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.MsgBeginUnlockingAllResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBeginUnlocking {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub id: u64,
    /// Amount of unlocking coins. Unlock all if not set.
    #[prost(message, repeated, tag = "3")]
    pub coins: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl crate::cosmwasm::ToCosmosMsg for MsgBeginUnlocking {
    const TYPE_URL: &'static str = "/osmosis.lockup.MsgBeginUnlocking";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBeginUnlockingResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
impl crate::cosmwasm::ToCosmosMsg for MsgBeginUnlockingResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.MsgBeginUnlockingResponse";
}
/// MsgExtendLockup extends the existing lockup's duration.
/// The new duration is longer than the original.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExtendLockup {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub id: u64,
    /// duration to be set. fails if lower than the current duration, or is
    /// unlocking
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
impl crate::cosmwasm::ToCosmosMsg for MsgExtendLockup {
    const TYPE_URL: &'static str = "/osmosis.lockup.MsgExtendLockup";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExtendLockupResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
impl crate::cosmwasm::ToCosmosMsg for MsgExtendLockupResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.MsgExtendLockupResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleBalanceRequest {}
impl crate::cosmwasm::ToCosmosMsg for ModuleBalanceRequest {
    const TYPE_URL: &'static str = "/osmosis.lockup.ModuleBalanceRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleBalanceResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl crate::cosmwasm::ToCosmosMsg for ModuleBalanceResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.ModuleBalanceResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleLockedAmountRequest {}
impl crate::cosmwasm::ToCosmosMsg for ModuleLockedAmountRequest {
    const TYPE_URL: &'static str = "/osmosis.lockup.ModuleLockedAmountRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleLockedAmountResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl crate::cosmwasm::ToCosmosMsg for ModuleLockedAmountResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.ModuleLockedAmountResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockableCoinsRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
impl crate::cosmwasm::ToCosmosMsg for AccountUnlockableCoinsRequest {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountUnlockableCoinsRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockableCoinsResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl crate::cosmwasm::ToCosmosMsg for AccountUnlockableCoinsResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountUnlockableCoinsResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockingCoinsRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
impl crate::cosmwasm::ToCosmosMsg for AccountUnlockingCoinsRequest {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountUnlockingCoinsRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockingCoinsResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl crate::cosmwasm::ToCosmosMsg for AccountUnlockingCoinsResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountUnlockingCoinsResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedCoinsRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
impl crate::cosmwasm::ToCosmosMsg for AccountLockedCoinsRequest {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountLockedCoinsRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedCoinsResponse {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
impl crate::cosmwasm::ToCosmosMsg for AccountLockedCoinsResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountLockedCoinsResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
impl crate::cosmwasm::ToCosmosMsg for AccountLockedPastTimeRequest {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountLockedPastTimeRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
impl crate::cosmwasm::ToCosmosMsg for AccountLockedPastTimeResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountLockedPastTimeResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeNotUnlockingOnlyRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
impl crate::cosmwasm::ToCosmosMsg for AccountLockedPastTimeNotUnlockingOnlyRequest {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountLockedPastTimeNotUnlockingOnlyRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeNotUnlockingOnlyResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
impl crate::cosmwasm::ToCosmosMsg for AccountLockedPastTimeNotUnlockingOnlyResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountLockedPastTimeNotUnlockingOnlyResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockedBeforeTimeRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
impl crate::cosmwasm::ToCosmosMsg for AccountUnlockedBeforeTimeRequest {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountUnlockedBeforeTimeRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUnlockedBeforeTimeResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
impl crate::cosmwasm::ToCosmosMsg for AccountUnlockedBeforeTimeResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountUnlockedBeforeTimeResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeDenomRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
}
impl crate::cosmwasm::ToCosmosMsg for AccountLockedPastTimeDenomRequest {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountLockedPastTimeDenomRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedPastTimeDenomResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
impl crate::cosmwasm::ToCosmosMsg for AccountLockedPastTimeDenomResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountLockedPastTimeDenomResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedDenomRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
impl crate::cosmwasm::ToCosmosMsg for LockedDenomRequest {
    const TYPE_URL: &'static str = "/osmosis.lockup.LockedDenomRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedDenomResponse {
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
}
impl crate::cosmwasm::ToCosmosMsg for LockedDenomResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.LockedDenomResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedRequest {
    #[prost(uint64, tag = "1")]
    pub lock_id: u64,
}
impl crate::cosmwasm::ToCosmosMsg for LockedRequest {
    const TYPE_URL: &'static str = "/osmosis.lockup.LockedRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedResponse {
    #[prost(message, optional, tag = "1")]
    pub lock: ::core::option::Option<PeriodLock>,
}
impl crate::cosmwasm::ToCosmosMsg for LockedResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.LockedResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyntheticLockupsByLockupIdRequest {
    #[prost(uint64, tag = "1")]
    pub lock_id: u64,
}
impl crate::cosmwasm::ToCosmosMsg for SyntheticLockupsByLockupIdRequest {
    const TYPE_URL: &'static str = "/osmosis.lockup.SyntheticLockupsByLockupIdRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyntheticLockupsByLockupIdResponse {
    #[prost(message, repeated, tag = "1")]
    pub synthetic_locks: ::prost::alloc::vec::Vec<SyntheticLock>,
}
impl crate::cosmwasm::ToCosmosMsg for SyntheticLockupsByLockupIdResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.SyntheticLockupsByLockupIdResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
impl crate::cosmwasm::ToCosmosMsg for AccountLockedLongerDurationRequest {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountLockedLongerDurationRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
impl crate::cosmwasm::ToCosmosMsg for AccountLockedLongerDurationResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountLockedLongerDurationResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedDurationRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
impl crate::cosmwasm::ToCosmosMsg for AccountLockedDurationRequest {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountLockedDurationRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedDurationResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
impl crate::cosmwasm::ToCosmosMsg for AccountLockedDurationResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountLockedDurationResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationNotUnlockingOnlyRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
impl crate::cosmwasm::ToCosmosMsg
for AccountLockedLongerDurationNotUnlockingOnlyRequest {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountLockedLongerDurationNotUnlockingOnlyRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationNotUnlockingOnlyResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
impl crate::cosmwasm::ToCosmosMsg
for AccountLockedLongerDurationNotUnlockingOnlyResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountLockedLongerDurationNotUnlockingOnlyResponse";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationDenomRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
}
impl crate::cosmwasm::ToCosmosMsg for AccountLockedLongerDurationDenomRequest {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountLockedLongerDurationDenomRequest";
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLockedLongerDurationDenomResponse {
    #[prost(message, repeated, tag = "1")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
}
impl crate::cosmwasm::ToCosmosMsg for AccountLockedLongerDurationDenomResponse {
    const TYPE_URL: &'static str = "/osmosis.lockup.AccountLockedLongerDurationDenomResponse";
}
/// GenesisState defines the lockup module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(uint64, tag = "1")]
    pub last_lock_id: u64,
    #[prost(message, repeated, tag = "2")]
    pub locks: ::prost::alloc::vec::Vec<PeriodLock>,
    #[prost(message, repeated, tag = "3")]
    pub synthetic_locks: ::prost::alloc::vec::Vec<SyntheticLock>,
}
impl crate::cosmwasm::ToCosmosMsg for GenesisState {
    const TYPE_URL: &'static str = "/osmosis.lockup.GenesisState";
}
