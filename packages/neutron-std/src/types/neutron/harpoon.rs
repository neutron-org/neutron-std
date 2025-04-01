use neutron_std_derive::CosmwasmExt;
/// Specifies how subscribed contract_addresses are stored in the KV store for each hook type.
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
#[proto_message(type_url = "/neutron.harpoon.HookSubscriptions")]
pub struct HookSubscriptions {
    /// The hook type being subscribed to.
    #[prost(enumeration = "HookType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub hook_type: i32,
    /// Contract addresses subscribed to this hook type.
    #[prost(string, repeated, tag = "2")]
    pub contract_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Hook types that can be subscribed to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub enum HookType {
    Unspecified = 0,
    /// Triggered after validator is created
    AfterValidatorCreated = 1,
    /// Triggered before validator is modified
    BeforeValidatorModified = 2,
    /// Triggered after validator is removed
    AfterValidatorRemoved = 3,
    /// Triggered after validator is bonded
    AfterValidatorBonded = 4,
    /// Triggered after validator begins unbonding
    AfterValidatorBeginUnbonding = 5,
    /// Triggered before delegation is created
    BeforeDelegationCreated = 6,
    /// Triggered before delegation's shares are modified
    BeforeDelegationSharesModified = 7,
    /// Triggered before delegation is removed
    BeforeDelegationRemoved = 8,
    /// Triggered after delegation is modified
    AfterDelegationModified = 9,
    /// Triggered before validator is slashed
    BeforeValidatorSlashed = 10,
    /// Triggered after unbonding is initiated
    AfterUnbondingInitiated = 11,
}
impl HookType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HookType::Unspecified => "HOOK_TYPE_UNSPECIFIED",
            HookType::AfterValidatorCreated => "HOOK_TYPE_AFTER_VALIDATOR_CREATED",
            HookType::BeforeValidatorModified => "HOOK_TYPE_BEFORE_VALIDATOR_MODIFIED",
            HookType::AfterValidatorRemoved => "HOOK_TYPE_AFTER_VALIDATOR_REMOVED",
            HookType::AfterValidatorBonded => "HOOK_TYPE_AFTER_VALIDATOR_BONDED",
            HookType::AfterValidatorBeginUnbonding => "HOOK_TYPE_AFTER_VALIDATOR_BEGIN_UNBONDING",
            HookType::BeforeDelegationCreated => "HOOK_TYPE_BEFORE_DELEGATION_CREATED",
            HookType::BeforeDelegationSharesModified => {
                "HOOK_TYPE_BEFORE_DELEGATION_SHARES_MODIFIED"
            }
            HookType::BeforeDelegationRemoved => "HOOK_TYPE_BEFORE_DELEGATION_REMOVED",
            HookType::AfterDelegationModified => "HOOK_TYPE_AFTER_DELEGATION_MODIFIED",
            HookType::BeforeValidatorSlashed => "HOOK_TYPE_BEFORE_VALIDATOR_SLASHED",
            HookType::AfterUnbondingInitiated => "HOOK_TYPE_AFTER_UNBONDING_INITIATED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HOOK_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "HOOK_TYPE_AFTER_VALIDATOR_CREATED" => Some(Self::AfterValidatorCreated),
            "HOOK_TYPE_BEFORE_VALIDATOR_MODIFIED" => Some(Self::BeforeValidatorModified),
            "HOOK_TYPE_AFTER_VALIDATOR_REMOVED" => Some(Self::AfterValidatorRemoved),
            "HOOK_TYPE_AFTER_VALIDATOR_BONDED" => Some(Self::AfterValidatorBonded),
            "HOOK_TYPE_AFTER_VALIDATOR_BEGIN_UNBONDING" => Some(Self::AfterValidatorBeginUnbonding),
            "HOOK_TYPE_BEFORE_DELEGATION_CREATED" => Some(Self::BeforeDelegationCreated),
            "HOOK_TYPE_BEFORE_DELEGATION_SHARES_MODIFIED" => {
                Some(Self::BeforeDelegationSharesModified)
            }
            "HOOK_TYPE_BEFORE_DELEGATION_REMOVED" => Some(Self::BeforeDelegationRemoved),
            "HOOK_TYPE_AFTER_DELEGATION_MODIFIED" => Some(Self::AfterDelegationModified),
            "HOOK_TYPE_BEFORE_VALIDATOR_SLASHED" => Some(Self::BeforeValidatorSlashed),
            "HOOK_TYPE_AFTER_UNBONDING_INITIATED" => Some(Self::AfterUnbondingInitiated),
            _ => None,
        }
    }
}
/// Harpoon module genesis state.
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
#[proto_message(type_url = "/neutron.harpoon.GenesisState")]
pub struct GenesisState {
    /// List of hooks
    #[prost(message, repeated, tag = "1")]
    pub hook_subscriptions: ::prost::alloc::vec::Vec<HookSubscriptions>,
}
/// Request type for the Query/SubscribedContracts RPC method.
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
#[proto_message(type_url = "/neutron.harpoon.QuerySubscribedContractsRequest")]
#[proto_query(
    path = "/neutron.harpoon.Query/SubscribedContracts",
    response_type = QuerySubscribedContractsResponse
)]
pub struct QuerySubscribedContractsRequest {
    /// The response will include only contract addresses for this hook type.
    #[prost(enumeration = "HookType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub hook_type: i32,
}
/// Response type for the Query/SubscribedContracts RPC method.
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
#[proto_message(type_url = "/neutron.harpoon.QuerySubscribedContractsResponse")]
pub struct QuerySubscribedContractsResponse {
    /// List of contract addresses subscribed to a specific hook.
    #[prost(string, repeated, tag = "1")]
    pub contract_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Defines the Msg/ManageHookSubscription request type.
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
#[proto_message(type_url = "/neutron.harpoon.MsgManageHookSubscription")]
pub struct MsgManageHookSubscription {
    /// Address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// Hook subscription to be updated.
    #[prost(message, optional, tag = "2")]
    pub hook_subscription: ::core::option::Option<HookSubscription>,
}
/// Defines the response structure for executing a MsgManageHookSubscription message.
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
#[proto_message(type_url = "/neutron.harpoon.MsgManageHookSubscriptionResponse")]
pub struct MsgManageHookSubscriptionResponse {}
/// Specifies new hook subscriptions for the contract_address.
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
#[proto_message(type_url = "/neutron.harpoon.HookSubscription")]
pub struct HookSubscription {
    /// Contract address to update subscriptions for.
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
    /// List of hooks to subscribe to. Hooks not listed here will be removed.
    #[prost(enumeration = "HookType", repeated, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub hooks: ::prost::alloc::vec::Vec<i32>,
}
pub struct HarpoonQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> HarpoonQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn subscribed_contracts(
        &self,
        hook_type: i32,
    ) -> Result<QuerySubscribedContractsResponse, cosmwasm_std::StdError> {
        QuerySubscribedContractsRequest { hook_type }.query(self.querier)
    }
}
