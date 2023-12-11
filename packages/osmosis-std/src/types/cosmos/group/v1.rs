use osmosis_std_derive::CosmwasmExt;
/// Member represents a group member with an account address,
/// non-zero weight, metadata and added_at timestamp.
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
#[proto_message(type_url = "/cosmos.group.v1.Member")]
pub struct Member {
    /// address is the member's account address.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// weight is the member's voting weight that should be greater than 0.
    #[prost(string, tag = "2")]
    pub weight: ::prost::alloc::string::String,
    /// metadata is any arbitrary metadata attached to the member.
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
    /// added_at is a timestamp specifying when a member was added.
    #[prost(message, optional, tag = "4")]
    pub added_at: ::core::option::Option<crate::shim::Timestamp>,
}
/// MemberRequest represents a group member to be used in Msg server requests.
/// Contrary to `Member`, it doesn't have any `added_at` field
/// since this field cannot be set as part of requests.
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
#[proto_message(type_url = "/cosmos.group.v1.MemberRequest")]
pub struct MemberRequest {
    /// address is the member's account address.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// weight is the member's voting weight that should be greater than 0.
    #[prost(string, tag = "2")]
    pub weight: ::prost::alloc::string::String,
    /// metadata is any arbitrary metadata attached to the member.
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
}
/// ThresholdDecisionPolicy is a decision policy where a proposal passes when it
/// satisfies the two following conditions:
/// 1. The sum of all `YES` voter's weights is greater or equal than the defined
///     `threshold`.
/// 2. The voting and execution periods of the proposal respect the parameters
///     given by `windows`.
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
#[proto_message(type_url = "/cosmos.group.v1.ThresholdDecisionPolicy")]
pub struct ThresholdDecisionPolicy {
    /// threshold is the minimum weighted sum of `YES` votes that must be met or
    /// exceeded for a proposal to succeed.
    #[prost(string, tag = "1")]
    pub threshold: ::prost::alloc::string::String,
    /// windows defines the different windows for voting and execution.
    #[prost(message, optional, tag = "2")]
    pub windows: ::core::option::Option<DecisionPolicyWindows>,
}
/// PercentageDecisionPolicy is a decision policy where a proposal passes when
/// it satisfies the two following conditions:
/// 1. The percentage of all `YES` voters' weights out of the total group weight
///     is greater or equal than the given `percentage`.
/// 2. The voting and execution periods of the proposal respect the parameters
///     given by `windows`.
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
#[proto_message(type_url = "/cosmos.group.v1.PercentageDecisionPolicy")]
pub struct PercentageDecisionPolicy {
    /// percentage is the minimum percentage of the weighted sum of `YES` votes must
    /// meet for a proposal to succeed.
    #[prost(string, tag = "1")]
    pub percentage: ::prost::alloc::string::String,
    /// windows defines the different windows for voting and execution.
    #[prost(message, optional, tag = "2")]
    pub windows: ::core::option::Option<DecisionPolicyWindows>,
}
/// DecisionPolicyWindows defines the different windows for voting and execution.
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
#[proto_message(type_url = "/cosmos.group.v1.DecisionPolicyWindows")]
pub struct DecisionPolicyWindows {
    /// voting_period is the duration from submission of a proposal to the end of voting period
    /// Within this times votes can be submitted with MsgVote.
    #[prost(message, optional, tag = "1")]
    pub voting_period: ::core::option::Option<crate::shim::Duration>,
    /// min_execution_period is the minimum duration after the proposal submission
    /// where members can start sending MsgExec. This means that the window for
    /// sending a MsgExec transaction is:
    /// `[ submission + min_execution_period ; submission + voting_period + max_execution_period]`
    /// where max_execution_period is a app-specific config, defined in the keeper.
    /// If not set, min_execution_period will default to 0.
    ///
    /// Please make sure to set a `min_execution_period` that is smaller than
    /// `voting_period + max_execution_period`, or else the above execution window
    /// is empty, meaning that all proposals created with this decision policy
    /// won't be able to be executed.
    #[prost(message, optional, tag = "2")]
    pub min_execution_period: ::core::option::Option<crate::shim::Duration>,
}
/// GroupInfo represents the high-level on-chain information for a group.
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
#[proto_message(type_url = "/cosmos.group.v1.GroupInfo")]
pub struct GroupInfo {
    /// id is the unique ID of the group.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// admin is the account address of the group's admin.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// metadata is any arbitrary metadata to attached to the group.
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
    /// version is used to track changes to a group's membership structure that
    /// would break existing proposals. Whenever any members weight is changed,
    /// or any member is added or removed this version is incremented and will
    /// cause proposals based on older versions of this group to fail
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub version: u64,
    /// total_weight is the sum of the group members' weights.
    #[prost(string, tag = "5")]
    pub total_weight: ::prost::alloc::string::String,
    /// created_at is a timestamp specifying when a group was created.
    #[prost(message, optional, tag = "6")]
    pub created_at: ::core::option::Option<crate::shim::Timestamp>,
}
/// GroupMember represents the relationship between a group and a member.
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
#[proto_message(type_url = "/cosmos.group.v1.GroupMember")]
pub struct GroupMember {
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "groupID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_id: u64,
    /// member is the member data.
    #[prost(message, optional, tag = "2")]
    pub member: ::core::option::Option<Member>,
}
/// GroupPolicyInfo represents the high-level on-chain information for a group policy.
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
#[proto_message(type_url = "/cosmos.group.v1.GroupPolicyInfo")]
pub struct GroupPolicyInfo {
    /// address is the account address of group policy.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "2")]
    #[serde(alias = "groupID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_id: u64,
    /// admin is the account address of the group admin.
    #[prost(string, tag = "3")]
    pub admin: ::prost::alloc::string::String,
    /// metadata is any arbitrary metadata attached to the group policy.
    /// the recommended format of the metadata is to be found here:
    /// <https://docs.cosmos.network/v0.47/modules/group#decision-policy-1>
    #[prost(string, tag = "4")]
    pub metadata: ::prost::alloc::string::String,
    /// version is used to track changes to a group's GroupPolicyInfo structure that
    /// would create a different result on a running proposal.
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub version: u64,
    /// decision_policy specifies the group policy's decision policy.
    #[prost(message, optional, tag = "6")]
    pub decision_policy: ::core::option::Option<crate::shim::Any>,
    /// created_at is a timestamp specifying when a group policy was created.
    #[prost(message, optional, tag = "7")]
    pub created_at: ::core::option::Option<crate::shim::Timestamp>,
}
/// Proposal defines a group proposal. Any member of a group can submit a proposal
/// for a group policy to decide upon.
/// A proposal consists of a set of `sdk.Msg`s that will be executed if the proposal
/// passes as well as some optional metadata associated with the proposal.
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
#[proto_message(type_url = "/cosmos.group.v1.Proposal")]
pub struct Proposal {
    /// id is the unique id of the proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// group_policy_address is the account address of group policy.
    #[prost(string, tag = "2")]
    pub group_policy_address: ::prost::alloc::string::String,
    /// metadata is any arbitrary metadata attached to the proposal.
    /// the recommended format of the metadata is to be found here:
    /// <https://docs.cosmos.network/v0.47/modules/group#proposal-4>
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
    /// proposers are the account addresses of the proposers.
    #[prost(string, repeated, tag = "4")]
    pub proposers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// submit_time is a timestamp specifying when a proposal was submitted.
    #[prost(message, optional, tag = "5")]
    pub submit_time: ::core::option::Option<crate::shim::Timestamp>,
    /// group_version tracks the version of the group at proposal submission.
    /// This field is here for informational purposes only.
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_version: u64,
    /// group_policy_version tracks the version of the group policy at proposal submission.
    /// When a decision policy is changed, existing proposals from previous policy
    /// versions will become invalid with the `ABORTED` status.
    /// This field is here for informational purposes only.
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_policy_version: u64,
    /// status represents the high level position in the life cycle of the proposal. Initial value is Submitted.
    #[prost(enumeration = "ProposalStatus", tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub status: i32,
    /// final_tally_result contains the sums of all weighted votes for this
    /// proposal for each vote option. It is empty at submission, and only
    /// populated after tallying, at voting period end or at proposal execution,
    /// whichever happens first.
    #[prost(message, optional, tag = "9")]
    pub final_tally_result: ::core::option::Option<TallyResult>,
    /// voting_period_end is the timestamp before which voting must be done.
    /// Unless a successful MsgExec is called before (to execute a proposal whose
    /// tally is successful before the voting period ends), tallying will be done
    /// at this point, and the `final_tally_result`and `status` fields will be
    /// accordingly updated.
    #[prost(message, optional, tag = "10")]
    pub voting_period_end: ::core::option::Option<crate::shim::Timestamp>,
    /// executor_result is the final result of the proposal execution. Initial value is NotRun.
    #[prost(enumeration = "ProposalExecutorResult", tag = "11")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub executor_result: i32,
    /// messages is a list of `sdk.Msg`s that will be executed if the proposal passes.
    #[prost(message, repeated, tag = "12")]
    pub messages: ::prost::alloc::vec::Vec<crate::shim::Any>,
    /// title is the title of the proposal
    ///
    /// Since: cosmos-sdk 0.47
    #[prost(string, tag = "13")]
    pub title: ::prost::alloc::string::String,
    /// summary is a short summary of the proposal
    ///
    /// Since: cosmos-sdk 0.47
    #[prost(string, tag = "14")]
    pub summary: ::prost::alloc::string::String,
}
/// TallyResult represents the sum of weighted votes for each vote option.
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
#[proto_message(type_url = "/cosmos.group.v1.TallyResult")]
pub struct TallyResult {
    /// yes_count is the weighted sum of yes votes.
    #[prost(string, tag = "1")]
    pub yes_count: ::prost::alloc::string::String,
    /// abstain_count is the weighted sum of abstainers.
    #[prost(string, tag = "2")]
    pub abstain_count: ::prost::alloc::string::String,
    /// no_count is the weighted sum of no votes.
    #[prost(string, tag = "3")]
    pub no_count: ::prost::alloc::string::String,
    /// no_with_veto_count is the weighted sum of veto.
    #[prost(string, tag = "4")]
    pub no_with_veto_count: ::prost::alloc::string::String,
}
/// Vote represents a vote for a proposal.
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
#[proto_message(type_url = "/cosmos.group.v1.Vote")]
pub struct Vote {
    /// proposal is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    /// voter is the account address of the voter.
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
    /// option is the voter's choice on the proposal.
    #[prost(enumeration = "VoteOption", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub option: i32,
    /// metadata is any arbitrary metadata attached to the vote.
    #[prost(string, tag = "4")]
    pub metadata: ::prost::alloc::string::String,
    /// submit_time is the timestamp when the vote was submitted.
    #[prost(message, optional, tag = "5")]
    pub submit_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// VoteOption enumerates the valid vote options for a given proposal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum VoteOption {
    /// VOTE_OPTION_UNSPECIFIED defines an unspecified vote option which will
    /// return an error.
    Unspecified = 0,
    /// VOTE_OPTION_YES defines a yes vote option.
    Yes = 1,
    /// VOTE_OPTION_ABSTAIN defines an abstain vote option.
    Abstain = 2,
    /// VOTE_OPTION_NO defines a no vote option.
    No = 3,
    /// VOTE_OPTION_NO_WITH_VETO defines a no with veto vote option.
    NoWithVeto = 4,
}
impl VoteOption {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VoteOption::Unspecified => "VOTE_OPTION_UNSPECIFIED",
            VoteOption::Yes => "VOTE_OPTION_YES",
            VoteOption::Abstain => "VOTE_OPTION_ABSTAIN",
            VoteOption::No => "VOTE_OPTION_NO",
            VoteOption::NoWithVeto => "VOTE_OPTION_NO_WITH_VETO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VOTE_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
            "VOTE_OPTION_YES" => Some(Self::Yes),
            "VOTE_OPTION_ABSTAIN" => Some(Self::Abstain),
            "VOTE_OPTION_NO" => Some(Self::No),
            "VOTE_OPTION_NO_WITH_VETO" => Some(Self::NoWithVeto),
            _ => None,
        }
    }
}
/// ProposalStatus defines proposal statuses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum ProposalStatus {
    /// An empty value is invalid and not allowed.
    Unspecified = 0,
    /// Initial status of a proposal when submitted.
    Submitted = 1,
    /// Final status of a proposal when the final tally is done and the outcome
    /// passes the group policy's decision policy.
    Accepted = 2,
    /// Final status of a proposal when the final tally is done and the outcome
    /// is rejected by the group policy's decision policy.
    Rejected = 3,
    /// Final status of a proposal when the group policy is modified before the
    /// final tally.
    Aborted = 4,
    /// A proposal can be withdrawn before the voting start time by the owner.
    /// When this happens the final status is Withdrawn.
    Withdrawn = 5,
}
impl ProposalStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProposalStatus::Unspecified => "PROPOSAL_STATUS_UNSPECIFIED",
            ProposalStatus::Submitted => "PROPOSAL_STATUS_SUBMITTED",
            ProposalStatus::Accepted => "PROPOSAL_STATUS_ACCEPTED",
            ProposalStatus::Rejected => "PROPOSAL_STATUS_REJECTED",
            ProposalStatus::Aborted => "PROPOSAL_STATUS_ABORTED",
            ProposalStatus::Withdrawn => "PROPOSAL_STATUS_WITHDRAWN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROPOSAL_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "PROPOSAL_STATUS_SUBMITTED" => Some(Self::Submitted),
            "PROPOSAL_STATUS_ACCEPTED" => Some(Self::Accepted),
            "PROPOSAL_STATUS_REJECTED" => Some(Self::Rejected),
            "PROPOSAL_STATUS_ABORTED" => Some(Self::Aborted),
            "PROPOSAL_STATUS_WITHDRAWN" => Some(Self::Withdrawn),
            _ => None,
        }
    }
}
/// ProposalExecutorResult defines types of proposal executor results.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum ProposalExecutorResult {
    /// An empty value is not allowed.
    Unspecified = 0,
    /// We have not yet run the executor.
    NotRun = 1,
    /// The executor was successful and proposed action updated state.
    Success = 2,
    /// The executor returned an error and proposed action didn't update state.
    Failure = 3,
}
impl ProposalExecutorResult {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProposalExecutorResult::Unspecified => "PROPOSAL_EXECUTOR_RESULT_UNSPECIFIED",
            ProposalExecutorResult::NotRun => "PROPOSAL_EXECUTOR_RESULT_NOT_RUN",
            ProposalExecutorResult::Success => "PROPOSAL_EXECUTOR_RESULT_SUCCESS",
            ProposalExecutorResult::Failure => "PROPOSAL_EXECUTOR_RESULT_FAILURE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROPOSAL_EXECUTOR_RESULT_UNSPECIFIED" => Some(Self::Unspecified),
            "PROPOSAL_EXECUTOR_RESULT_NOT_RUN" => Some(Self::NotRun),
            "PROPOSAL_EXECUTOR_RESULT_SUCCESS" => Some(Self::Success),
            "PROPOSAL_EXECUTOR_RESULT_FAILURE" => Some(Self::Failure),
            _ => None,
        }
    }
}
/// EventCreateGroup is an event emitted when a group is created.
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
#[proto_message(type_url = "/cosmos.group.v1.EventCreateGroup")]
pub struct EventCreateGroup {
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "groupID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_id: u64,
}
/// EventUpdateGroup is an event emitted when a group is updated.
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
#[proto_message(type_url = "/cosmos.group.v1.EventUpdateGroup")]
pub struct EventUpdateGroup {
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "groupID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_id: u64,
}
/// EventCreateGroupPolicy is an event emitted when a group policy is created.
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
#[proto_message(type_url = "/cosmos.group.v1.EventCreateGroupPolicy")]
pub struct EventCreateGroupPolicy {
    /// address is the account address of the group policy.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// EventUpdateGroupPolicy is an event emitted when a group policy is updated.
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
#[proto_message(type_url = "/cosmos.group.v1.EventUpdateGroupPolicy")]
pub struct EventUpdateGroupPolicy {
    /// address is the account address of the group policy.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// EventSubmitProposal is an event emitted when a proposal is created.
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
#[proto_message(type_url = "/cosmos.group.v1.EventSubmitProposal")]
pub struct EventSubmitProposal {
    /// proposal_id is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
}
/// EventWithdrawProposal is an event emitted when a proposal is withdrawn.
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
#[proto_message(type_url = "/cosmos.group.v1.EventWithdrawProposal")]
pub struct EventWithdrawProposal {
    /// proposal_id is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
}
/// EventVote is an event emitted when a voter votes on a proposal.
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
#[proto_message(type_url = "/cosmos.group.v1.EventVote")]
pub struct EventVote {
    /// proposal_id is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
}
/// EventExec is an event emitted when a proposal is executed.
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
#[proto_message(type_url = "/cosmos.group.v1.EventExec")]
pub struct EventExec {
    /// proposal_id is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    /// result is the proposal execution result.
    #[prost(enumeration = "ProposalExecutorResult", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub result: i32,
    /// logs contains error logs in case the execution result is FAILURE.
    #[prost(string, tag = "3")]
    pub logs: ::prost::alloc::string::String,
}
/// EventLeaveGroup is an event emitted when group member leaves the group.
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
#[proto_message(type_url = "/cosmos.group.v1.EventLeaveGroup")]
pub struct EventLeaveGroup {
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "groupID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_id: u64,
    /// address is the account address of the group member.
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
/// EventProposalPruned is an event emitted when a proposal is pruned.
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
#[proto_message(type_url = "/cosmos.group.v1.EventProposalPruned")]
pub struct EventProposalPruned {
    /// proposal_id is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    /// status is the proposal status (UNSPECIFIED, SUBMITTED, ACCEPTED, REJECTED, ABORTED, WITHDRAWN).
    #[prost(enumeration = "ProposalStatus", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub status: i32,
    /// tally_result is the proposal tally result (when applicable).
    #[prost(message, optional, tag = "3")]
    pub tally_result: ::core::option::Option<TallyResult>,
}
/// GenesisState defines the group module's genesis state.
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
#[proto_message(type_url = "/cosmos.group.v1.GenesisState")]
pub struct GenesisState {
    /// group_seq is the group table orm.Sequence,
    /// it is used to get the next group ID.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_seq: u64,
    /// groups is the list of groups info.
    #[prost(message, repeated, tag = "2")]
    pub groups: ::prost::alloc::vec::Vec<GroupInfo>,
    /// group_members is the list of groups members.
    #[prost(message, repeated, tag = "3")]
    pub group_members: ::prost::alloc::vec::Vec<GroupMember>,
    /// group_policy_seq is the group policy table orm.Sequence,
    /// it is used to generate the next group policy account address.
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_policy_seq: u64,
    /// group_policies is the list of group policies info.
    #[prost(message, repeated, tag = "5")]
    pub group_policies: ::prost::alloc::vec::Vec<GroupPolicyInfo>,
    /// proposal_seq is the proposal table orm.Sequence,
    /// it is used to get the next proposal ID.
    #[prost(uint64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_seq: u64,
    /// proposals is the list of proposals.
    #[prost(message, repeated, tag = "7")]
    pub proposals: ::prost::alloc::vec::Vec<Proposal>,
    /// votes is the list of votes.
    #[prost(message, repeated, tag = "8")]
    pub votes: ::prost::alloc::vec::Vec<Vote>,
}
/// QueryGroupInfoRequest is the Query/GroupInfo request type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryGroupInfoRequest")]
#[proto_query(
    path = "/cosmos.group.v1.Query/GroupInfo",
    response_type = QueryGroupInfoResponse
)]
pub struct QueryGroupInfoRequest {
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "groupID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_id: u64,
}
/// QueryGroupInfoResponse is the Query/GroupInfo response type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryGroupInfoResponse")]
pub struct QueryGroupInfoResponse {
    /// info is the GroupInfo of the group.
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<GroupInfo>,
}
/// QueryGroupPolicyInfoRequest is the Query/GroupPolicyInfo request type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryGroupPolicyInfoRequest")]
#[proto_query(
    path = "/cosmos.group.v1.Query/GroupPolicyInfo",
    response_type = QueryGroupPolicyInfoResponse
)]
pub struct QueryGroupPolicyInfoRequest {
    /// address is the account address of the group policy.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryGroupPolicyInfoResponse is the Query/GroupPolicyInfo response type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryGroupPolicyInfoResponse")]
pub struct QueryGroupPolicyInfoResponse {
    /// info is the GroupPolicyInfo of the group policy.
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<GroupPolicyInfo>,
}
/// QueryGroupMembersRequest is the Query/GroupMembers request type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryGroupMembersRequest")]
#[proto_query(
    path = "/cosmos.group.v1.Query/GroupMembers",
    response_type = QueryGroupMembersResponse
)]
pub struct QueryGroupMembersRequest {
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "groupID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_id: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryGroupMembersResponse is the Query/GroupMembersResponse response type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryGroupMembersResponse")]
pub struct QueryGroupMembersResponse {
    /// members are the members of the group with given group_id.
    #[prost(message, repeated, tag = "1")]
    pub members: ::prost::alloc::vec::Vec<GroupMember>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryGroupsByAdminRequest is the Query/GroupsByAdmin request type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryGroupsByAdminRequest")]
#[proto_query(
    path = "/cosmos.group.v1.Query/GroupsByAdmin",
    response_type = QueryGroupsByAdminResponse
)]
pub struct QueryGroupsByAdminRequest {
    /// admin is the account address of a group's admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryGroupsByAdminResponse is the Query/GroupsByAdminResponse response type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryGroupsByAdminResponse")]
pub struct QueryGroupsByAdminResponse {
    /// groups are the groups info with the provided admin.
    #[prost(message, repeated, tag = "1")]
    pub groups: ::prost::alloc::vec::Vec<GroupInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryGroupPoliciesByGroupRequest is the Query/GroupPoliciesByGroup request type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryGroupPoliciesByGroupRequest")]
#[proto_query(
    path = "/cosmos.group.v1.Query/GroupPoliciesByGroup",
    response_type = QueryGroupPoliciesByGroupResponse
)]
pub struct QueryGroupPoliciesByGroupRequest {
    /// group_id is the unique ID of the group policy's group.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "groupID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_id: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryGroupPoliciesByGroupResponse is the Query/GroupPoliciesByGroup response type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryGroupPoliciesByGroupResponse")]
pub struct QueryGroupPoliciesByGroupResponse {
    /// group_policies are the group policies info associated with the provided group.
    #[prost(message, repeated, tag = "1")]
    pub group_policies: ::prost::alloc::vec::Vec<GroupPolicyInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryGroupPoliciesByAdminRequest is the Query/GroupPoliciesByAdmin request type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryGroupPoliciesByAdminRequest")]
#[proto_query(
    path = "/cosmos.group.v1.Query/GroupPoliciesByAdmin",
    response_type = QueryGroupPoliciesByAdminResponse
)]
pub struct QueryGroupPoliciesByAdminRequest {
    /// admin is the admin address of the group policy.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryGroupPoliciesByAdminResponse is the Query/GroupPoliciesByAdmin response type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryGroupPoliciesByAdminResponse")]
pub struct QueryGroupPoliciesByAdminResponse {
    /// group_policies are the group policies info with provided admin.
    #[prost(message, repeated, tag = "1")]
    pub group_policies: ::prost::alloc::vec::Vec<GroupPolicyInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryProposalRequest is the Query/Proposal request type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryProposalRequest")]
#[proto_query(
    path = "/cosmos.group.v1.Query/Proposal",
    response_type = QueryProposalResponse
)]
pub struct QueryProposalRequest {
    /// proposal_id is the unique ID of a proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
}
/// QueryProposalResponse is the Query/Proposal response type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryProposalResponse")]
pub struct QueryProposalResponse {
    /// proposal is the proposal info.
    #[prost(message, optional, tag = "1")]
    pub proposal: ::core::option::Option<Proposal>,
}
/// QueryProposalsByGroupPolicyRequest is the Query/ProposalByGroupPolicy request type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryProposalsByGroupPolicyRequest")]
#[proto_query(
    path = "/cosmos.group.v1.Query/ProposalsByGroupPolicy",
    response_type = QueryProposalsByGroupPolicyResponse
)]
pub struct QueryProposalsByGroupPolicyRequest {
    /// address is the account address of the group policy related to proposals.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryProposalsByGroupPolicyResponse is the Query/ProposalByGroupPolicy response type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryProposalsByGroupPolicyResponse")]
pub struct QueryProposalsByGroupPolicyResponse {
    /// proposals are the proposals with given group policy.
    #[prost(message, repeated, tag = "1")]
    pub proposals: ::prost::alloc::vec::Vec<Proposal>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryVoteByProposalVoterRequest is the Query/VoteByProposalVoter request type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryVoteByProposalVoterRequest")]
#[proto_query(
    path = "/cosmos.group.v1.Query/VoteByProposalVoter",
    response_type = QueryVoteByProposalVoterResponse
)]
pub struct QueryVoteByProposalVoterRequest {
    /// proposal_id is the unique ID of a proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    /// voter is a proposal voter account address.
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
}
/// QueryVoteByProposalVoterResponse is the Query/VoteByProposalVoter response type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryVoteByProposalVoterResponse")]
pub struct QueryVoteByProposalVoterResponse {
    /// vote is the vote with given proposal_id and voter.
    #[prost(message, optional, tag = "1")]
    pub vote: ::core::option::Option<Vote>,
}
/// QueryVotesByProposalRequest is the Query/VotesByProposal request type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryVotesByProposalRequest")]
#[proto_query(
    path = "/cosmos.group.v1.Query/VotesByProposal",
    response_type = QueryVotesByProposalResponse
)]
pub struct QueryVotesByProposalRequest {
    /// proposal_id is the unique ID of a proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryVotesByProposalResponse is the Query/VotesByProposal response type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryVotesByProposalResponse")]
pub struct QueryVotesByProposalResponse {
    /// votes are the list of votes for given proposal_id.
    #[prost(message, repeated, tag = "1")]
    pub votes: ::prost::alloc::vec::Vec<Vote>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryVotesByVoterRequest is the Query/VotesByVoter request type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryVotesByVoterRequest")]
#[proto_query(
    path = "/cosmos.group.v1.Query/VotesByVoter",
    response_type = QueryVotesByVoterResponse
)]
pub struct QueryVotesByVoterRequest {
    /// voter is a proposal voter account address.
    #[prost(string, tag = "1")]
    pub voter: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryVotesByVoterResponse is the Query/VotesByVoter response type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryVotesByVoterResponse")]
pub struct QueryVotesByVoterResponse {
    /// votes are the list of votes by given voter.
    #[prost(message, repeated, tag = "1")]
    pub votes: ::prost::alloc::vec::Vec<Vote>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryGroupsByMemberRequest is the Query/GroupsByMember request type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryGroupsByMemberRequest")]
#[proto_query(
    path = "/cosmos.group.v1.Query/GroupsByMember",
    response_type = QueryGroupsByMemberResponse
)]
pub struct QueryGroupsByMemberRequest {
    /// address is the group member address.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryGroupsByMemberResponse is the Query/GroupsByMember response type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryGroupsByMemberResponse")]
pub struct QueryGroupsByMemberResponse {
    /// groups are the groups info with the provided group member.
    #[prost(message, repeated, tag = "1")]
    pub groups: ::prost::alloc::vec::Vec<GroupInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryTallyResultRequest is the Query/TallyResult request type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryTallyResultRequest")]
#[proto_query(
    path = "/cosmos.group.v1.Query/TallyResult",
    response_type = QueryTallyResultResponse
)]
pub struct QueryTallyResultRequest {
    /// proposal_id is the unique id of a proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
}
/// QueryTallyResultResponse is the Query/TallyResult response type.
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
#[proto_message(type_url = "/cosmos.group.v1.QueryTallyResultResponse")]
pub struct QueryTallyResultResponse {
    /// tally defines the requested tally.
    #[prost(message, optional, tag = "1")]
    pub tally: ::core::option::Option<TallyResult>,
}
/// QueryGroupsRequest is the Query/Groups request type.
///
/// Since: cosmos-sdk 0.47.1
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
#[proto_message(type_url = "/cosmos.group.v1.QueryGroupsRequest")]
#[proto_query(
    path = "/cosmos.group.v1.Query/Groups",
    response_type = QueryGroupsResponse
)]
pub struct QueryGroupsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryGroupsResponse is the Query/Groups response type.
///
/// Since: cosmos-sdk 0.47.1
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
#[proto_message(type_url = "/cosmos.group.v1.QueryGroupsResponse")]
pub struct QueryGroupsResponse {
    /// `groups` is all the groups present in state.
    #[prost(message, repeated, tag = "1")]
    pub groups: ::prost::alloc::vec::Vec<GroupInfo>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// MsgCreateGroup is the Msg/CreateGroup request type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgCreateGroup")]
pub struct MsgCreateGroup {
    /// admin is the account address of the group admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// members defines the group members.
    #[prost(message, repeated, tag = "2")]
    pub members: ::prost::alloc::vec::Vec<MemberRequest>,
    /// metadata is any arbitrary metadata to attached to the group.
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
}
/// MsgCreateGroupResponse is the Msg/CreateGroup response type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgCreateGroupResponse")]
pub struct MsgCreateGroupResponse {
    /// group_id is the unique ID of the newly created group.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "groupID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_id: u64,
}
/// MsgUpdateGroupMembers is the Msg/UpdateGroupMembers request type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgUpdateGroupMembers")]
pub struct MsgUpdateGroupMembers {
    /// admin is the account address of the group admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "2")]
    #[serde(alias = "groupID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_id: u64,
    /// member_updates is the list of members to update,
    /// set weight to 0 to remove a member.
    #[prost(message, repeated, tag = "3")]
    pub member_updates: ::prost::alloc::vec::Vec<MemberRequest>,
}
/// MsgUpdateGroupMembersResponse is the Msg/UpdateGroupMembers response type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgUpdateGroupMembersResponse")]
pub struct MsgUpdateGroupMembersResponse {}
/// MsgUpdateGroupAdmin is the Msg/UpdateGroupAdmin request type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgUpdateGroupAdmin")]
pub struct MsgUpdateGroupAdmin {
    /// admin is the current account address of the group admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "2")]
    #[serde(alias = "groupID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_id: u64,
    /// new_admin is the group new admin account address.
    #[prost(string, tag = "3")]
    pub new_admin: ::prost::alloc::string::String,
}
/// MsgUpdateGroupAdminResponse is the Msg/UpdateGroupAdmin response type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgUpdateGroupAdminResponse")]
pub struct MsgUpdateGroupAdminResponse {}
/// MsgUpdateGroupMetadata is the Msg/UpdateGroupMetadata request type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgUpdateGroupMetadata")]
pub struct MsgUpdateGroupMetadata {
    /// admin is the account address of the group admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "2")]
    #[serde(alias = "groupID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_id: u64,
    /// metadata is the updated group's metadata.
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
}
/// MsgUpdateGroupMetadataResponse is the Msg/UpdateGroupMetadata response type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgUpdateGroupMetadataResponse")]
pub struct MsgUpdateGroupMetadataResponse {}
/// MsgCreateGroupPolicy is the Msg/CreateGroupPolicy request type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgCreateGroupPolicy")]
pub struct MsgCreateGroupPolicy {
    /// admin is the account address of the group admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "2")]
    #[serde(alias = "groupID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_id: u64,
    /// metadata is any arbitrary metadata attached to the group policy.
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
    /// decision_policy specifies the group policy's decision policy.
    #[prost(message, optional, tag = "4")]
    pub decision_policy: ::core::option::Option<crate::shim::Any>,
}
/// MsgCreateGroupPolicyResponse is the Msg/CreateGroupPolicy response type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgCreateGroupPolicyResponse")]
pub struct MsgCreateGroupPolicyResponse {
    /// address is the account address of the newly created group policy.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// MsgUpdateGroupPolicyAdmin is the Msg/UpdateGroupPolicyAdmin request type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgUpdateGroupPolicyAdmin")]
pub struct MsgUpdateGroupPolicyAdmin {
    /// admin is the account address of the group admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// group_policy_address is the account address of the group policy.
    #[prost(string, tag = "2")]
    pub group_policy_address: ::prost::alloc::string::String,
    /// new_admin is the new group policy admin.
    #[prost(string, tag = "3")]
    pub new_admin: ::prost::alloc::string::String,
}
/// MsgUpdateGroupPolicyAdminResponse is the Msg/UpdateGroupPolicyAdmin response type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgUpdateGroupPolicyAdminResponse")]
pub struct MsgUpdateGroupPolicyAdminResponse {}
/// MsgCreateGroupWithPolicy is the Msg/CreateGroupWithPolicy request type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgCreateGroupWithPolicy")]
pub struct MsgCreateGroupWithPolicy {
    /// admin is the account address of the group and group policy admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// members defines the group members.
    #[prost(message, repeated, tag = "2")]
    pub members: ::prost::alloc::vec::Vec<MemberRequest>,
    /// group_metadata is any arbitrary metadata attached to the group.
    #[prost(string, tag = "3")]
    pub group_metadata: ::prost::alloc::string::String,
    /// group_policy_metadata is any arbitrary metadata attached to the group policy.
    #[prost(string, tag = "4")]
    pub group_policy_metadata: ::prost::alloc::string::String,
    /// group_policy_as_admin is a boolean field, if set to true, the group policy account address will be used as group
    /// and group policy admin.
    #[prost(bool, tag = "5")]
    pub group_policy_as_admin: bool,
    /// decision_policy specifies the group policy's decision policy.
    #[prost(message, optional, tag = "6")]
    pub decision_policy: ::core::option::Option<crate::shim::Any>,
}
/// MsgCreateGroupWithPolicyResponse is the Msg/CreateGroupWithPolicy response type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgCreateGroupWithPolicyResponse")]
pub struct MsgCreateGroupWithPolicyResponse {
    /// group_id is the unique ID of the newly created group with policy.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "groupID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_id: u64,
    /// group_policy_address is the account address of the newly created group policy.
    #[prost(string, tag = "2")]
    pub group_policy_address: ::prost::alloc::string::String,
}
/// MsgUpdateGroupPolicyDecisionPolicy is the Msg/UpdateGroupPolicyDecisionPolicy request type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgUpdateGroupPolicyDecisionPolicy")]
pub struct MsgUpdateGroupPolicyDecisionPolicy {
    /// admin is the account address of the group admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// group_policy_address is the account address of group policy.
    #[prost(string, tag = "2")]
    pub group_policy_address: ::prost::alloc::string::String,
    /// decision_policy is the updated group policy's decision policy.
    #[prost(message, optional, tag = "3")]
    pub decision_policy: ::core::option::Option<crate::shim::Any>,
}
/// MsgUpdateGroupPolicyDecisionPolicyResponse is the Msg/UpdateGroupPolicyDecisionPolicy response type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgUpdateGroupPolicyDecisionPolicyResponse")]
pub struct MsgUpdateGroupPolicyDecisionPolicyResponse {}
/// MsgUpdateGroupPolicyMetadata is the Msg/UpdateGroupPolicyMetadata request type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgUpdateGroupPolicyMetadata")]
pub struct MsgUpdateGroupPolicyMetadata {
    /// admin is the account address of the group admin.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// group_policy_address is the account address of group policy.
    #[prost(string, tag = "2")]
    pub group_policy_address: ::prost::alloc::string::String,
    /// metadata is the group policy metadata to be updated.
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
}
/// MsgUpdateGroupPolicyMetadataResponse is the Msg/UpdateGroupPolicyMetadata response type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgUpdateGroupPolicyMetadataResponse")]
pub struct MsgUpdateGroupPolicyMetadataResponse {}
/// MsgSubmitProposal is the Msg/SubmitProposal request type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgSubmitProposal")]
pub struct MsgSubmitProposal {
    /// group_policy_address is the account address of group policy.
    #[prost(string, tag = "1")]
    pub group_policy_address: ::prost::alloc::string::String,
    /// proposers are the account addresses of the proposers.
    /// Proposers signatures will be counted as yes votes.
    #[prost(string, repeated, tag = "2")]
    pub proposers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// metadata is any arbitrary metadata attached to the proposal.
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
    /// messages is a list of `sdk.Msg`s that will be executed if the proposal passes.
    #[prost(message, repeated, tag = "4")]
    pub messages: ::prost::alloc::vec::Vec<crate::shim::Any>,
    /// exec defines the mode of execution of the proposal,
    /// whether it should be executed immediately on creation or not.
    /// If so, proposers signatures are considered as Yes votes.
    #[prost(enumeration = "Exec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub exec: i32,
    /// title is the title of the proposal.
    ///
    /// Since: cosmos-sdk 0.47
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
    /// summary is the summary of the proposal.
    ///
    /// Since: cosmos-sdk 0.47
    #[prost(string, tag = "7")]
    pub summary: ::prost::alloc::string::String,
}
/// MsgSubmitProposalResponse is the Msg/SubmitProposal response type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgSubmitProposalResponse")]
pub struct MsgSubmitProposalResponse {
    /// proposal is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
}
/// MsgWithdrawProposal is the Msg/WithdrawProposal request type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgWithdrawProposal")]
pub struct MsgWithdrawProposal {
    /// proposal is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    /// address is the admin of the group policy or one of the proposer of the proposal.
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
}
/// MsgWithdrawProposalResponse is the Msg/WithdrawProposal response type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgWithdrawProposalResponse")]
pub struct MsgWithdrawProposalResponse {}
/// MsgVote is the Msg/Vote request type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgVote")]
pub struct MsgVote {
    /// proposal is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    /// voter is the voter account address.
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
    /// option is the voter's choice on the proposal.
    #[prost(enumeration = "VoteOption", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub option: i32,
    /// metadata is any arbitrary metadata attached to the vote.
    #[prost(string, tag = "4")]
    pub metadata: ::prost::alloc::string::String,
    /// exec defines whether the proposal should be executed
    /// immediately after voting or not.
    #[prost(enumeration = "Exec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub exec: i32,
}
/// MsgVoteResponse is the Msg/Vote response type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgVoteResponse")]
pub struct MsgVoteResponse {}
/// MsgExec is the Msg/Exec request type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgExec")]
pub struct MsgExec {
    /// proposal is the unique ID of the proposal.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    /// executor is the account address used to execute the proposal.
    #[prost(string, tag = "2")]
    pub executor: ::prost::alloc::string::String,
}
/// MsgExecResponse is the Msg/Exec request type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgExecResponse")]
pub struct MsgExecResponse {
    /// result is the final result of the proposal execution.
    #[prost(enumeration = "ProposalExecutorResult", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub result: i32,
}
/// MsgLeaveGroup is the Msg/LeaveGroup request type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgLeaveGroup")]
pub struct MsgLeaveGroup {
    /// address is the account address of the group member.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// group_id is the unique ID of the group.
    #[prost(uint64, tag = "2")]
    #[serde(alias = "groupID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub group_id: u64,
}
/// MsgLeaveGroupResponse is the Msg/LeaveGroup response type.
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
#[proto_message(type_url = "/cosmos.group.v1.MsgLeaveGroupResponse")]
pub struct MsgLeaveGroupResponse {}
/// Exec defines modes of execution of a proposal on creation or on new vote.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum Exec {
    /// An empty value means that there should be a separate
    /// MsgExec request for the proposal to execute.
    Unspecified = 0,
    /// Try to execute the proposal immediately.
    /// If the proposal is not allowed per the DecisionPolicy,
    /// the proposal will still be open and could
    /// be executed at a later point.
    Try = 1,
}
impl Exec {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Exec::Unspecified => "EXEC_UNSPECIFIED",
            Exec::Try => "EXEC_TRY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXEC_UNSPECIFIED" => Some(Self::Unspecified),
            "EXEC_TRY" => Some(Self::Try),
            _ => None,
        }
    }
}
pub struct GroupQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> GroupQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn group_info(
        &self,
        group_id: u64,
    ) -> Result<QueryGroupInfoResponse, cosmwasm_std::StdError> {
        QueryGroupInfoRequest { group_id }.query(self.querier)
    }
    pub fn group_policy_info(
        &self,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryGroupPolicyInfoResponse, cosmwasm_std::StdError> {
        QueryGroupPolicyInfoRequest { address }.query(self.querier)
    }
    pub fn group_members(
        &self,
        group_id: u64,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryGroupMembersResponse, cosmwasm_std::StdError> {
        QueryGroupMembersRequest {
            group_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn groups_by_admin(
        &self,
        admin: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryGroupsByAdminResponse, cosmwasm_std::StdError> {
        QueryGroupsByAdminRequest { admin, pagination }.query(self.querier)
    }
    pub fn group_policies_by_group(
        &self,
        group_id: u64,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryGroupPoliciesByGroupResponse, cosmwasm_std::StdError> {
        QueryGroupPoliciesByGroupRequest {
            group_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn group_policies_by_admin(
        &self,
        admin: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryGroupPoliciesByAdminResponse, cosmwasm_std::StdError> {
        QueryGroupPoliciesByAdminRequest { admin, pagination }.query(self.querier)
    }
    pub fn proposal(
        &self,
        proposal_id: u64,
    ) -> Result<QueryProposalResponse, cosmwasm_std::StdError> {
        QueryProposalRequest { proposal_id }.query(self.querier)
    }
    pub fn proposals_by_group_policy(
        &self,
        address: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryProposalsByGroupPolicyResponse, cosmwasm_std::StdError> {
        QueryProposalsByGroupPolicyRequest {
            address,
            pagination,
        }
        .query(self.querier)
    }
    pub fn vote_by_proposal_voter(
        &self,
        proposal_id: u64,
        voter: ::prost::alloc::string::String,
    ) -> Result<QueryVoteByProposalVoterResponse, cosmwasm_std::StdError> {
        QueryVoteByProposalVoterRequest { proposal_id, voter }.query(self.querier)
    }
    pub fn votes_by_proposal(
        &self,
        proposal_id: u64,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryVotesByProposalResponse, cosmwasm_std::StdError> {
        QueryVotesByProposalRequest {
            proposal_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn votes_by_voter(
        &self,
        voter: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryVotesByVoterResponse, cosmwasm_std::StdError> {
        QueryVotesByVoterRequest { voter, pagination }.query(self.querier)
    }
    pub fn groups_by_member(
        &self,
        address: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryGroupsByMemberResponse, cosmwasm_std::StdError> {
        QueryGroupsByMemberRequest {
            address,
            pagination,
        }
        .query(self.querier)
    }
    pub fn tally_result(
        &self,
        proposal_id: u64,
    ) -> Result<QueryTallyResultResponse, cosmwasm_std::StdError> {
        QueryTallyResultRequest { proposal_id }.query(self.querier)
    }
    pub fn groups(
        &self,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryGroupsResponse, cosmwasm_std::StdError> {
        QueryGroupsRequest { pagination }.query(self.querier)
    }
}
