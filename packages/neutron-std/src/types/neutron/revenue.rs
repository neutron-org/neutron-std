use neutron_std_derive::CosmwasmExt;
/// Defines the parameters for the module.
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
#[proto_message(type_url = "/neutron.revenue.Params")]
pub struct Params {
    /// The asset used in revenue payments to validators. Expected to be a native token of the chain
    /// with its denom metadata registered in the bank module. The denom metadata must have a defined
    /// symbol field and contain a denom unit with an alias equal to the symbol and a specified
    /// exponent.
    #[prost(string, tag = "1")]
    pub reward_asset: ::prost::alloc::string::String,
    /// Quotation of the reward asset.
    #[prost(message, optional, tag = "2")]
    pub reward_quote: ::core::option::Option<RewardQuote>,
    /// Specifies performance requirements for validators in scope of blocks signing and creation. If
    /// not met, the validator is not rewarded.
    #[prost(message, optional, tag = "3")]
    pub blocks_performance_requirement: ::core::option::Option<PerformanceRequirement>,
    /// Specifies performance requirements for validators in scope of the oracle price votes. If not
    /// met, the validator is not rewarded.
    #[prost(message, optional, tag = "4")]
    pub oracle_votes_performance_requirement: ::core::option::Option<PerformanceRequirement>,
    /// Indicates the currently active type of payment schedule.
    #[prost(message, optional, tag = "5")]
    pub payment_schedule_type: ::core::option::Option<PaymentScheduleType>,
    /// The time window, in seconds, used to calculate the TWAP of the reward asset.
    #[prost(int64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub twap_window: i64,
}
/// Defines information about the reward quote.
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
#[proto_message(type_url = "/neutron.revenue.RewardQuote")]
pub struct RewardQuote {
    /// The compensation amount measured in the quote asset. The amount is divided by the price of
    /// the reward asset to determine the base revenue amount.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub amount: u64,
    /// The name of the quote asset. It is used as a quote in price queries to the slinky oracle
    /// module to determine the price of the reward asset.
    #[prost(string, tag = "2")]
    pub asset: ::prost::alloc::string::String,
}
/// A model that contains information specific to the currently active payment schedule type. The
/// oneof implementations define the payment schedule that must be used currently.
/// This is a module's parameter. It's not updated automatically in runtime in contrast to the
/// payment schedule which is a state variable, but is updated via MsgUpdateParams.
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
#[proto_message(type_url = "/neutron.revenue.PaymentScheduleType")]
pub struct PaymentScheduleType {
    #[prost(oneof = "payment_schedule_type::PaymentScheduleType", tags = "4, 5, 6")]
    pub payment_schedule_type: ::core::option::Option<payment_schedule_type::PaymentScheduleType>,
}
/// Nested message and enum types in `PaymentScheduleType`.
pub mod payment_schedule_type {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Oneof,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
    )]
    pub enum PaymentScheduleType {
        #[prost(message, tag = "4")]
        MonthlyPaymentScheduleType(super::MonthlyPaymentScheduleType),
        #[prost(message, tag = "5")]
        BlockBasedPaymentScheduleType(super::BlockBasedPaymentScheduleType),
        #[prost(message, tag = "6")]
        EmptyPaymentScheduleType(super::EmptyPaymentScheduleType),
    }
}
/// Monthly periods with payments made at the end of each month.
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
#[proto_message(type_url = "/neutron.revenue.MonthlyPaymentScheduleType")]
pub struct MonthlyPaymentScheduleType {}
/// Periods defined by a specific number of blocks, with payments made when the required block
/// count is reached.
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
#[proto_message(type_url = "/neutron.revenue.BlockBasedPaymentScheduleType")]
pub struct BlockBasedPaymentScheduleType {
    /// The number of blocks in a payment period.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub blocks_per_period: u64,
}
/// Endless periods with payments never made.
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
#[proto_message(type_url = "/neutron.revenue.EmptyPaymentScheduleType")]
pub struct EmptyPaymentScheduleType {}
/// Specifies a performance criteria that validators must meet to qualify for network rewards.
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
#[proto_message(type_url = "/neutron.revenue.PerformanceRequirement")]
pub struct PerformanceRequirement {
    /// The fraction of the total performance a validator can miss without affecting their reward.
    /// Represented as a decimal value in the range [0.0, 1.0], where 1.0 corresponds to 100%.
    #[prost(string, tag = "1")]
    pub allowed_to_miss: ::prost::alloc::string::String,
    /// The minimum fraction of the total performance a validator must achieve to be eligible for
    /// network rewards. Validators falling below this threshold will not receive any rewards.
    /// Represented as a decimal value in the range [0.0, 1.0], where 1.0 corresponds to 100%.
    #[prost(string, tag = "2")]
    pub required_at_least: ::prost::alloc::string::String,
}
/// Defines the revenue module's genesis state.
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
#[proto_message(type_url = "/neutron.revenue.GenesisState")]
pub struct GenesisState {
    /// Revenue module parameters.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// The current payment schedule. If nil, the module will use the respective payment schedule for
    /// the payment schedule type specified in the params.
    #[prost(message, optional, tag = "2")]
    pub payment_schedule: ::core::option::Option<PaymentSchedule>,
    /// Revenue module list of validators.
    #[prost(message, repeated, tag = "3")]
    pub validators: ::prost::alloc::vec::Vec<ValidatorInfo>,
}
/// A model that contains information specific to the currently active payment schedule. The oneof
/// implementations define conditions for payment periods ending and track the progress of the
/// current payment period. This is a module's state variable.
/// The inner oneof must correspond with the respective payment schedule type defined in the module
/// params. In runtime, on a mismatch due to e.g. MsgUpdateParams execution, the module will switch
/// to the payment schedule that corresponds to the payment schedule type automatically.
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
#[proto_message(type_url = "/neutron.revenue.PaymentSchedule")]
pub struct PaymentSchedule {
    #[prost(oneof = "payment_schedule::PaymentSchedule", tags = "1, 2, 3")]
    pub payment_schedule: ::core::option::Option<payment_schedule::PaymentSchedule>,
}
/// Nested message and enum types in `PaymentSchedule`.
pub mod payment_schedule {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Oneof,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
    )]
    pub enum PaymentSchedule {
        #[prost(message, tag = "1")]
        MonthlyPaymentSchedule(super::MonthlyPaymentSchedule),
        #[prost(message, tag = "2")]
        BlockBasedPaymentSchedule(super::BlockBasedPaymentSchedule),
        #[prost(message, tag = "3")]
        EmptyPaymentSchedule(super::EmptyPaymentSchedule),
    }
}
/// Contains information about a validator.
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
#[proto_message(type_url = "/neutron.revenue.ValidatorInfo")]
pub struct ValidatorInfo {
    /// The validator's node operator address.
    #[prost(string, tag = "1")]
    pub val_oper_address: ::prost::alloc::string::String,
    /// The number of blocks the validator has committed in the current payment period.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub commited_blocks_in_period: u64,
    /// The number of oracle votes the validator has submitted in the current payment period.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub commited_oracle_votes_in_period: u64,
    /// The number of blocks the validator has remained in the active validator set for in the
    /// current payment period.
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub in_active_valset_for_blocks_in_period: u64,
}
/// Represents a payment schedule where revenue payments are processed once a month.
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
#[proto_message(type_url = "/neutron.revenue.MonthlyPaymentSchedule")]
pub struct MonthlyPaymentSchedule {
    /// The block height at which the current month started.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub current_month_start_block: u64,
    /// The timestamp of the block at which the current month started.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub current_month_start_block_ts: u64,
}
/// Represents a payment schedule where revenue payments are processed after a specified number
/// of blocks.
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
#[proto_message(type_url = "/neutron.revenue.BlockBasedPaymentSchedule")]
pub struct BlockBasedPaymentSchedule {
    /// The number of blocks in each payment period.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub blocks_per_period: u64,
    /// The block height at which the current payment period started.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub current_period_start_block: u64,
}
/// Represents a payment schedule where revenue is never distributed.
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
#[proto_message(type_url = "/neutron.revenue.EmptyPaymentSchedule")]
pub struct EmptyPaymentSchedule {}
/// Represents a data structure that tracks the cumulative price of an asset over the entire
/// observation period, along with the last absolute asset price and the timestamp when this
/// price was last recorded.
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
#[proto_message(type_url = "/neutron.revenue.RewardAssetPrice")]
pub struct RewardAssetPrice {
    /// The cumulative price of the reward asset within the TWAP window. It is calculated as:
    /// `cumulative_price_at_timestamp_t(n)` = `last_price_at_t(n-1)` * (t(n) - t(n-1)) + `cumulative_price_at_timestamp_t(n-1)`
    #[prost(string, tag = "1")]
    pub cumulative_price: ::prost::alloc::string::String,
    /// The price of the reward asset in reward quote asset that corresponds to the timestamp.
    #[prost(string, tag = "2")]
    pub absolute_price: ::prost::alloc::string::String,
    /// The timestamp of the last update of the absolute and cumulative price.
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub timestamp: i64,
}
/// Request type for the Msg/UpdateParams RPC method.
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
#[proto_message(type_url = "/neutron.revenue.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// The address of the authority of the module.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// The new parameters of the module. All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// Response type for the Msg/UpdateParams RPC method.
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
#[proto_message(type_url = "/neutron.revenue.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
/// Request type for the Msg/FundTreasury RPC method.
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
#[proto_message(type_url = "/neutron.revenue.MsgFundTreasury")]
pub struct MsgFundTreasury {
    /// The signer of the message.
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// The amount of coins to fund the revenue treasury pool with. Must match the reward asset denom.
    #[prost(message, repeated, tag = "2")]
    pub amount: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
/// Response type for the Msg/FundTreasury RPC method.
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
#[proto_message(type_url = "/neutron.revenue.MsgFundTreasuryResponse")]
pub struct MsgFundTreasuryResponse {}
/// Request type for the Query/Params RPC method.
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
#[proto_message(type_url = "/neutron.revenue.QueryParamsRequest")]
#[proto_query(
    path = "/neutron.revenue.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// Response type for the Query/Params RPC method.
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
#[proto_message(type_url = "/neutron.revenue.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// Contains all parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// Request type for the Query/PaymentInfo RPC method.
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
#[proto_message(type_url = "/neutron.revenue.QueryPaymentInfoRequest")]
#[proto_query(
    path = "/neutron.revenue.Query/PaymentInfo",
    response_type = QueryPaymentInfoResponse
)]
pub struct QueryPaymentInfoRequest {}
/// Response type for the Query/PaymentInfo RPC method.
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
#[proto_message(type_url = "/neutron.revenue.QueryPaymentInfoResponse")]
pub struct QueryPaymentInfoResponse {
    /// The current payment schedule.
    #[prost(message, optional, tag = "1")]
    pub payment_schedule: ::core::option::Option<PaymentSchedule>,
    /// Revenue amount multiplier value that corresponds to the effective payment period progress.
    #[prost(string, tag = "2")]
    pub effective_period_progress: ::prost::alloc::string::String,
    /// The current TWAP of the reward asset in quote asset. Is calculated as:
    /// twap_from_time_t(n)_to_time_t(m) = (cumulative_price_at_t(n) - cumulative_price_at_t(m)) / (t(n) - t(m))
    #[prost(string, tag = "3")]
    pub reward_asset_twap: ::prost::alloc::string::String,
    /// The current evaluation of the base revenue amount. This is the maximum amount a validator can
    /// receive in the current price condition if not affected with reducing factors (e.g. imperfect
    /// performance, incomplete payment period, partial validator set presence).
    #[prost(message, optional, tag = "4")]
    pub base_revenue_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// Request type for the Query/ValidatorStats RPC method.
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
#[proto_message(type_url = "/neutron.revenue.QueryValidatorStatsRequest")]
#[proto_query(
    path = "/neutron.revenue.Query/ValidatorStats",
    response_type = QueryValidatorStatsResponse
)]
pub struct QueryValidatorStatsRequest {
    /// The validator's node operator address.
    #[prost(string, tag = "1")]
    pub val_oper_address: ::prost::alloc::string::String,
}
/// Response type for the Query/ValidatorStats RPC method.
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
#[proto_message(type_url = "/neutron.revenue.QueryValidatorStatsResponse")]
pub struct QueryValidatorStatsResponse {
    /// Contains the validator's information.
    #[prost(message, optional, tag = "1")]
    pub stats: ::core::option::Option<ValidatorStats>,
}
/// Request type for the Query/ValidatorsStats RPC method.
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
#[proto_message(type_url = "/neutron.revenue.QueryValidatorsStatsRequest")]
#[proto_query(
    path = "/neutron.revenue.Query/ValidatorsStats",
    response_type = QueryValidatorsStatsResponse
)]
pub struct QueryValidatorsStatsRequest {}
/// Response type for the Query/ValidatorsStats RPC method.
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
#[proto_message(type_url = "/neutron.revenue.QueryValidatorsStatsResponse")]
pub struct QueryValidatorsStatsResponse {
    /// Contains the validators' information.
    #[prost(message, repeated, tag = "1")]
    pub stats: ::prost::alloc::vec::Vec<ValidatorStats>,
}
/// Contains validator's info and their performance rating.
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
#[proto_message(type_url = "/neutron.revenue.ValidatorStats")]
pub struct ValidatorStats {
    /// Contains the validator's information.
    #[prost(message, optional, tag = "1")]
    pub validator_info: ::core::option::Option<ValidatorInfo>,
    /// The total number of blocks produced by the chain in the current payment period.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub total_produced_blocks_in_period: u64,
    /// The validator's performance rating. Represented as a decimal value.
    #[prost(string, tag = "3")]
    pub performance_rating: ::prost::alloc::string::String,
    /// Contains expected revenue for the validator based on their performance rating in the current
    /// payment period, current reward asset TWAP, and duration of validator's presence in the active
    /// validator set. Does not take into account effective payment period progress.
    #[prost(message, optional, tag = "4")]
    pub expected_revenue: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
pub struct RevenueQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> RevenueQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn payment_info(&self) -> Result<QueryPaymentInfoResponse, cosmwasm_std::StdError> {
        QueryPaymentInfoRequest {}.query(self.querier)
    }
    pub fn validator_stats(
        &self,
        val_oper_address: ::prost::alloc::string::String,
    ) -> Result<QueryValidatorStatsResponse, cosmwasm_std::StdError> {
        QueryValidatorStatsRequest { val_oper_address }.query(self.querier)
    }
    pub fn validators_stats(&self) -> Result<QueryValidatorsStatsResponse, cosmwasm_std::StdError> {
        QueryValidatorsStatsRequest {}.query(self.querier)
    }
}
