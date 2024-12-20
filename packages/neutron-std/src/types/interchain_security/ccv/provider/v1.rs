use neutron_std_derive::CosmwasmExt;
/// ConsumerAdditionProposal is a governance proposal on the provider chain to
/// spawn a new consumer chain. If it passes, then all validators on the provider
/// chain are expected to validate the consumer chain at spawn time or get
/// slashed. It is recommended that spawn time occurs after the proposal end
/// time.
/// Use MsgConsumerAddition to submit this proposal type.
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.ConsumerAdditionProposal")]
pub struct ConsumerAdditionProposal {
    /// the title of the proposal
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// the description of the proposal
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// the proposed chain-id of the new consumer chain, must be different from all
    /// other consumer chain ids of the executing provider chain.
    #[prost(string, tag = "3")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// the proposed initial height of new consumer chain.
    /// For a completely new chain, this will be {0,1}. However, it may be
    /// different if this is a chain that is converting to a consumer chain.
    #[prost(message, optional, tag = "4")]
    pub initial_height:
        ::core::option::Option<super::super::super::super::ibc::core::client::v1::Height>,
    /// The hash of the consumer chain genesis state without the consumer CCV
    /// module genesis params. It is used for off-chain confirmation of
    /// genesis.json validity by validators and other parties.
    #[prost(bytes = "vec", tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub genesis_hash: ::prost::alloc::vec::Vec<u8>,
    /// The hash of the consumer chain binary that should be run by validators on
    /// chain initialization. It is used for off-chain confirmation of binary
    /// validity by validators and other parties.
    #[prost(bytes = "vec", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub binary_hash: ::prost::alloc::vec::Vec<u8>,
    /// spawn time is the time on the provider chain at which the consumer chain
    /// genesis is finalized and all validators will be responsible for starting
    /// their consumer chain validator node.
    #[prost(message, optional, tag = "7")]
    pub spawn_time: ::core::option::Option<crate::shim::Timestamp>,
    /// Unbonding period for the consumer,
    /// which should be smaller than that of the provider in general.
    #[prost(message, optional, tag = "8")]
    pub unbonding_period: ::core::option::Option<crate::shim::Duration>,
    /// Sent CCV related IBC packets will timeout after this duration
    #[prost(message, optional, tag = "9")]
    pub ccv_timeout_period: ::core::option::Option<crate::shim::Duration>,
    /// Sent transfer related IBC packets will timeout after this duration
    #[prost(message, optional, tag = "10")]
    pub transfer_timeout_period: ::core::option::Option<crate::shim::Duration>,
    /// The fraction of tokens allocated to the consumer redistribution address
    /// during distribution events. The fraction is a string representing a
    /// decimal number. For example "0.75" would represent 75%.
    #[prost(string, tag = "11")]
    pub consumer_redistribution_fraction: ::prost::alloc::string::String,
    /// BlocksPerDistributionTransmission is the number of blocks between
    /// ibc-token-transfers from the consumer chain to the provider chain. On
    /// sending transmission event, `consumer_redistribution_fraction` of the
    /// accumulated tokens are sent to the consumer redistribution address.
    #[prost(int64, tag = "12")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub blocks_per_distribution_transmission: i64,
    /// The number of historical info entries to persist in store.
    /// This param is a part of the cosmos sdk staking module. In the case of
    /// a ccv enabled consumer chain, the ccv module acts as the staking module.
    #[prost(int64, tag = "13")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub historical_entries: i64,
    /// The ID of a token transfer channel used for the Reward Distribution
    /// sub-protocol. If DistributionTransmissionChannel == "", a new transfer
    /// channel is created on top of the same connection as the CCV channel.
    /// Note that transfer_channel_id is the ID of the channel end on the consumer
    /// chain. it is most relevant for chains performing a sovereign to consumer
    /// changeover in order to maintain the existing ibc transfer channel
    #[prost(string, tag = "14")]
    pub distribution_transmission_channel: ::prost::alloc::string::String,
    /// Corresponds to the percentage of validators that have to validate the chain under the Top N case.
    /// For example, 53 corresponds to a Top 53% chain, meaning that the top 53% provider validators by voting power
    /// have to validate the proposed consumer chain. top_N can either be 0 or any value in [50, 100].
    /// A chain can join with top_N == 0 as an Opt In chain, or with top_N ∈ [50, 100] as a Top N chain.
    #[prost(uint32, tag = "15")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub top_n: u32,
    /// Corresponds to the maximum power (percentage-wise) a validator can have on the consumer chain. For instance, if
    /// `validators_power_cap` is set to 32, it means that no validator can have more than 32% of the voting power on the
    /// consumer chain. Note that this might not be feasible. For example, think of a consumer chain with only
    /// 5 validators and with `validators_power_cap` set to 10%. In such a scenario, at least one validator would need
    /// to have more than 20% of the total voting power. Therefore, `validators_power_cap` operates on a best-effort basis.
    #[prost(uint32, tag = "16")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub validators_power_cap: u32,
    /// Corresponds to the maximum number of validators that can validate a consumer chain.
    /// Only applicable to Opt In chains. Setting `validator_set_cap` on a Top N chain is a no-op.
    #[prost(uint32, tag = "17")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub validator_set_cap: u32,
    /// Corresponds to a list of provider consensus addresses of validators that are the ONLY ones that can validate
    /// the consumer chain.
    #[prost(string, repeated, tag = "18")]
    pub allowlist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Corresponds to a list of provider consensus addresses of validators that CANNOT validate the consumer chain.
    #[prost(string, repeated, tag = "19")]
    pub denylist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ConsumerRemovalProposal is a governance proposal on the provider chain to
/// remove (and stop) a consumer chain. If it passes, all the consumer chain's
/// state is removed from the provider chain. The outstanding unbonding operation
/// funds are released.
/// Use MsgConsumerRemoval to submit this proposal type.
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.ConsumerRemovalProposal")]
pub struct ConsumerRemovalProposal {
    /// the title of the proposal
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// the description of the proposal
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// the chain-id of the consumer chain to be stopped
    #[prost(string, tag = "3")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// the time on the provider chain at which all validators are responsible to
    /// stop their consumer chain validator node
    #[prost(message, optional, tag = "4")]
    pub stop_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// ConsumerModificationProposal is a governance proposal on the provider chain to modify parameters of a running
/// consumer chain. If it passes, the consumer chain's state is updated to take into account the newest params.
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.ConsumerModificationProposal")]
pub struct ConsumerModificationProposal {
    /// the title of the proposal
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// the description of the proposal
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// the chain-id of the consumer chain to be modified
    #[prost(string, tag = "3")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// Corresponds to the percentage of validators that have to validate the chain under the Top N case.
    /// For example, 53 corresponds to a Top 53% chain, meaning that the top 53% provider validators by voting power
    /// have to validate the proposed consumer chain. top_N can either be 0 or any value in [50, 100].
    /// A chain can join with top_N == 0 as an Opt In chain, or with top_N ∈ [50, 100] as a Top N chain.
    #[prost(uint32, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub top_n: u32,
    /// Corresponds to the maximum power (percentage-wise) a validator can have on the consumer chain. For instance, if
    /// `validators_power_cap` is set to 32, it means that no validator can have more than 32% of the voting power on the
    /// consumer chain. Note that this might not be feasible. For example, think of a consumer chain with only
    /// 5 validators and with `validators_power_cap` set to 10%. In such a scenario, at least one validator would need
    /// to have more than 20% of the total voting power. Therefore, `validators_power_cap` operates on a best-effort basis.
    #[prost(uint32, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub validators_power_cap: u32,
    /// Corresponds to the maximum number of validators that can validate a consumer chain.
    /// Only applicable to Opt In chains. Setting `validator_set_cap` on a Top N chain is a no-op.
    #[prost(uint32, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub validator_set_cap: u32,
    /// Corresponds to a list of provider consensus addresses of validators that are the ONLY ones that can validate
    /// the consumer chain.
    #[prost(string, repeated, tag = "7")]
    pub allowlist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Corresponds to a list of provider consensus addresses of validators that CANNOT validate the consumer chain.
    #[prost(string, repeated, tag = "8")]
    pub denylist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// EquivocationProposal is a governance proposal on the provider chain to
/// punish a validator for equivocation on a consumer chain.
///
/// This type is only used internally to the consumer CCV module.
/// WARNING: This message is deprecated now that equivocations can be submitted
/// and verified automatically on the provider. (see SubmitConsumerDoubleVoting in proto/interchain-security/ccv/provider/v1/tx.proto).
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.EquivocationProposal")]
#[deprecated]
pub struct EquivocationProposal {
    /// the title of the proposal
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// the description of the proposal
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// the list of equivocations that will be processed
    #[prost(message, repeated, tag = "3")]
    pub equivocations: ::prost::alloc::vec::Vec<
        super::super::super::super::cosmos::evidence::v1beta1::Equivocation,
    >,
}
/// ChangeRewardDenomsProposal is a governance proposal on the provider chain to
/// mutate the set of denoms accepted by the provider as rewards.
/// Use MsgChangeRewardDenoms to submit this proposal type.
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.ChangeRewardDenomsProposal")]
pub struct ChangeRewardDenomsProposal {
    /// the title of the proposal
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// the description of the proposal
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// the list of consumer reward denoms to add
    #[prost(string, repeated, tag = "3")]
    pub denoms_to_add: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the list of consumer reward denoms to remove
    #[prost(string, repeated, tag = "4")]
    pub denoms_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A persisted queue entry indicating that a slash packet data instance needs to
/// be handled. This type belongs in the "global" queue, to coordinate slash
/// packet handling times between consumers.
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.GlobalSlashEntry")]
pub struct GlobalSlashEntry {
    /// Block time that slash packet was received by provider chain.
    /// This field is used for store key iteration ordering.
    #[prost(message, optional, tag = "1")]
    pub recv_time: ::core::option::Option<crate::shim::Timestamp>,
    /// The consumer that sent a slash packet.
    #[prost(string, tag = "2")]
    #[serde(alias = "consumer_chainID")]
    pub consumer_chain_id: ::prost::alloc::string::String,
    /// The IBC sequence number of the recv packet.
    /// This field is used in the store key to ensure uniqueness.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub ibc_seq_num: u64,
    /// The provider's consensus address of the validator being slashed.
    /// This field is used to obtain validator power in HandleThrottleQueues.
    ///
    /// This field is not used in the store key, but is persisted in value bytes,
    /// see QueueGlobalSlashEntry.
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub provider_val_cons_addr: ::prost::alloc::vec::Vec<u8>,
}
/// Params defines the parameters for CCV Provider module
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.Params")]
pub struct Params {
    #[prost(message, optional, tag = "1")]
    pub template_client: ::core::option::Option<
        super::super::super::super::ibc::lightclients::tendermint::v1::ClientState,
    >,
    /// TrustingPeriodFraction is used to compute the consumer and provider IBC
    /// client's TrustingPeriod from the chain defined UnbondingPeriod
    #[prost(string, tag = "2")]
    pub trusting_period_fraction: ::prost::alloc::string::String,
    /// Sent IBC packets will timeout after this duration
    #[prost(message, optional, tag = "3")]
    pub ccv_timeout_period: ::core::option::Option<crate::shim::Duration>,
    /// The channel initialization (IBC channel opening handshake) will timeout
    /// after this duration
    #[prost(message, optional, tag = "4")]
    pub init_timeout_period: ::core::option::Option<crate::shim::Duration>,
    /// The VSC packets sent by the provider will timeout after this duration.
    /// Note that unlike ccv_timeout_period which is an IBC param,
    /// the vsc_timeout_period is a provider-side param that enables the provider
    /// to timeout VSC packets even when a consumer chain is not live.
    #[prost(message, optional, tag = "5")]
    pub vsc_timeout_period: ::core::option::Option<crate::shim::Duration>,
    /// The period for which the slash meter is replenished
    #[prost(message, optional, tag = "6")]
    pub slash_meter_replenish_period: ::core::option::Option<crate::shim::Duration>,
    /// The fraction of total voting power that is replenished to the slash meter
    /// every replenish period. This param also serves as a maximum fraction of
    /// total voting power that the slash meter can hold.
    #[prost(string, tag = "7")]
    pub slash_meter_replenish_fraction: ::prost::alloc::string::String,
    /// The fee required to be paid to add a reward denom
    #[prost(message, optional, tag = "9")]
    pub consumer_reward_denom_registration_fee:
        ::core::option::Option<super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// The number of blocks that comprise an epoch.
    #[prost(int64, tag = "10")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub blocks_per_epoch: i64,
    /// The number of epochs a validator has to validate a consumer chain in order to start receiving rewards from that chain.
    #[prost(int64, tag = "11")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub number_of_epochs_to_start_receiving_rewards: i64,
}
/// SlashAcks contains cons addresses of consumer chain validators
/// successfully slashed on the provider chain.
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.SlashAcks")]
pub struct SlashAcks {
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ConsumerAdditionProposals holds pending governance proposals on the provider
/// chain to spawn a new chain.
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.ConsumerAdditionProposals")]
pub struct ConsumerAdditionProposals {
    /// proposals waiting for spawn_time to pass
    #[prost(message, repeated, tag = "1")]
    pub pending: ::prost::alloc::vec::Vec<ConsumerAdditionProposal>,
}
/// ConsumerRemovalProposals holds pending governance proposals on the provider
/// chain to remove (and stop) a consumer chain.
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.ConsumerRemovalProposals")]
pub struct ConsumerRemovalProposals {
    /// proposals waiting for stop_time to pass
    #[prost(message, repeated, tag = "1")]
    pub pending: ::prost::alloc::vec::Vec<ConsumerRemovalProposal>,
}
/// AddressList contains a list of consensus addresses
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.AddressList")]
pub struct AddressList {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// ChannelToChain is used to map a CCV channel ID to the consumer chainID
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.ChannelToChain")]
pub struct ChannelToChain {
    #[prost(string, tag = "1")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
}
/// VscUnbondingOps contains the IDs of unbonding operations that are waiting for
/// at least one VSCMaturedPacket with vscID from a consumer chain
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.VscUnbondingOps")]
pub struct VscUnbondingOps {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "vscID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub vsc_id: u64,
    #[prost(uint64, repeated, tag = "2")]
    #[serde(alias = "unbonding_opIDs")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub unbonding_op_ids: ::prost::alloc::vec::Vec<u64>,
}
/// UnbondingOp contains the ids of consumer chains that need to unbond before
/// the unbonding operation with the given ID can unbond
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.UnbondingOp")]
pub struct UnbondingOp {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// consumer chains that are still unbonding
    #[prost(string, repeated, tag = "2")]
    pub unbonding_consumer_chains: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.InitTimeoutTimestamp")]
pub struct InitTimeoutTimestamp {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub timestamp: u64,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.VscSendTimestamp")]
pub struct VscSendTimestamp {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "vscID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub vsc_id: u64,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<crate::shim::Timestamp>,
}
/// ValidatorSetChangePackets is a pb list of ccv.ValidatorSetChangePacketData.
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.ValidatorSetChangePackets")]
pub struct ValidatorSetChangePackets {
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<super::super::v1::ValidatorSetChangePacketData>,
}
/// MaturedUnbondingOps defines a list of ids corresponding to ids of matured
/// unbonding operations.
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MaturedUnbondingOps")]
pub struct MaturedUnbondingOps {
    #[prost(uint64, repeated, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub ids: ::prost::alloc::vec::Vec<u64>,
}
/// ExportedVscSendTimestamps is VscSendTimestamp with chainID info for exporting to genesis
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.ExportedVscSendTimestamp")]
pub struct ExportedVscSendTimestamp {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub vsc_send_timestamps: ::prost::alloc::vec::Vec<VscSendTimestamp>,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.KeyAssignmentReplacement")]
pub struct KeyAssignmentReplacement {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub provider_addr: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub prev_c_key:
        ::core::option::Option<super::super::super::super::tendermint::crypto::PublicKey>,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub power: i64,
}
/// Used to serialize the ValidatorConsumerPubKey index from key assignment
/// ValidatorConsumerPubKey: (chainID, providerAddr consAddr) -> consumerKey
/// tmprotocrypto.PublicKey
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.ValidatorConsumerPubKey")]
pub struct ValidatorConsumerPubKey {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub provider_addr: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub consumer_key:
        ::core::option::Option<super::super::super::super::tendermint::crypto::PublicKey>,
}
/// Used to serialize the ValidatorConsumerAddr index from key assignment
/// ValidatorByConsumerAddr: (chainID, consumerAddr consAddr) -> providerAddr
/// consAddr
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.ValidatorByConsumerAddr")]
pub struct ValidatorByConsumerAddr {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub consumer_addr: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub provider_addr: ::prost::alloc::vec::Vec<u8>,
}
/// Used to serialize the ConsumerAddrsToPrune index from key assignment
/// ConsumerAddrsToPrune: (chainID, vscID uint64) -> consumerAddrs AddressList
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.ConsumerAddrsToPrune")]
pub struct ConsumerAddrsToPrune {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(alias = "vscID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub vsc_id: u64,
    #[prost(message, optional, tag = "3")]
    pub consumer_addrs: ::core::option::Option<AddressList>,
}
/// ConsumerValidator is used to facilitate epoch-based transitions. It contains relevant info for
/// a validator that is expected to validate on a consumer chain during an epoch.
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.ConsumerValidator")]
pub struct ConsumerValidator {
    /// validator's consensus address on the provider chain
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub provider_cons_addr: ::prost::alloc::vec::Vec<u8>,
    /// voting power the validator has during this epoch
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub power: i64,
    /// public key the validator uses on the consumer chain during this epoch
    #[prost(message, optional, tag = "3")]
    pub consumer_public_key:
        ::core::option::Option<super::super::super::super::tendermint::crypto::PublicKey>,
    /// height the validator had when it FIRST became a consumer validator
    /// If a validator becomes a consumer validator at height `H` and is continuously a consumer validator for all the upcoming
    /// epochs, then the height of the validator SHOULD remain `H`. This height only resets to a different height if a validator
    /// stops being a consumer validator during an epoch and later becomes again a consumer validator.
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub join_height: i64,
}
/// ConsumerRewardsAllocation stores the rewards allocated by a consumer chain
/// to the consumer rewards pool. It is used to allocate the tokens to the consumer
/// opted-in validators and the community pool during BeginBlock.
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.ConsumerRewardsAllocation")]
pub struct ConsumerRewardsAllocation {
    #[prost(message, repeated, tag = "1")]
    pub rewards:
        ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::DecCoin>,
}
/// GenesisState defines the CCV provider chain genesis state
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.GenesisState")]
pub struct GenesisState {
    /// strictly positive and set to 1 (DefaultValsetUpdateID) for a new chain
    #[prost(uint64, tag = "1")]
    #[serde(alias = "valset_updateID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub valset_update_id: u64,
    /// empty for a new chain
    #[prost(message, repeated, tag = "2")]
    pub consumer_states: ::prost::alloc::vec::Vec<ConsumerState>,
    /// empty for a new chain
    #[prost(message, repeated, tag = "3")]
    pub unbonding_ops: ::prost::alloc::vec::Vec<UnbondingOp>,
    /// empty for a new chain
    #[prost(message, optional, tag = "4")]
    pub mature_unbonding_ops: ::core::option::Option<MaturedUnbondingOps>,
    /// empty for a new chain
    #[prost(message, repeated, tag = "5")]
    #[serde(alias = "valset_updateID_to_height")]
    pub valset_update_id_to_height: ::prost::alloc::vec::Vec<ValsetUpdateIdToHeight>,
    /// empty for a new chain
    #[prost(message, repeated, tag = "6")]
    pub consumer_addition_proposals: ::prost::alloc::vec::Vec<ConsumerAdditionProposal>,
    /// empty for a new chain
    #[prost(message, repeated, tag = "7")]
    pub consumer_removal_proposals: ::prost::alloc::vec::Vec<ConsumerRemovalProposal>,
    #[prost(message, optional, tag = "8")]
    pub params: ::core::option::Option<Params>,
    /// empty for a new chain
    #[prost(message, repeated, tag = "9")]
    pub validator_consumer_pubkeys: ::prost::alloc::vec::Vec<ValidatorConsumerPubKey>,
    /// empty for a new chain
    #[prost(message, repeated, tag = "10")]
    pub validators_by_consumer_addr: ::prost::alloc::vec::Vec<ValidatorByConsumerAddr>,
    /// empty for a new chain
    #[prost(message, repeated, tag = "11")]
    pub consumer_addrs_to_prune: ::prost::alloc::vec::Vec<ConsumerAddrsToPrune>,
    #[prost(message, repeated, tag = "12")]
    pub init_timeout_timestamps: ::prost::alloc::vec::Vec<InitTimeoutTimestamp>,
    #[prost(message, repeated, tag = "13")]
    pub exported_vsc_send_timestamps: ::prost::alloc::vec::Vec<ExportedVscSendTimestamp>,
}
/// The provider CCV module's knowledge of consumer state.
///
/// Note this type is only used internally to the provider CCV module.
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.ConsumerState")]
pub struct ConsumerState {
    /// ChainID defines the chain ID for the consumer chain
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// ChannelID defines the IBC channel ID for the consumer chain
    #[prost(string, tag = "2")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// ClientID defines the IBC client ID for the consumer chain
    #[prost(string, tag = "3")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    /// InitalHeight defines the initial block height for the consumer chain
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub initial_height: u64,
    /// ConsumerGenesis defines the initial consumer chain genesis states
    #[prost(message, optional, tag = "5")]
    pub consumer_genesis: ::core::option::Option<super::super::v1::ConsumerGenesisState>,
    /// PendingValsetChanges defines the pending validator set changes for the
    /// consumer chain
    #[prost(message, repeated, tag = "6")]
    pub pending_valset_changes:
        ::prost::alloc::vec::Vec<super::super::v1::ValidatorSetChangePacketData>,
    #[prost(string, repeated, tag = "7")]
    pub slash_downtime_ack: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// UnbondingOpsIndex defines the unbonding operations waiting on this consumer
    /// chain
    #[prost(message, repeated, tag = "8")]
    pub unbonding_ops_index: ::prost::alloc::vec::Vec<VscUnbondingOps>,
}
/// ValsetUpdateIdToHeight defines the genesis information for the mapping
/// of each valset update id to a block height
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.ValsetUpdateIdToHeight")]
pub struct ValsetUpdateIdToHeight {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "valset_updateID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub valset_update_id: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.QueryConsumerGenesisRequest")]
#[proto_query(
    path = "/interchain_security.ccv.provider.v1.Query/QueryConsumerGenesis",
    response_type = QueryConsumerGenesisResponse
)]
pub struct QueryConsumerGenesisRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.QueryConsumerGenesisResponse")]
pub struct QueryConsumerGenesisResponse {
    #[prost(message, optional, tag = "1")]
    pub genesis_state: ::core::option::Option<super::super::v1::ConsumerGenesisState>,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.QueryConsumerChainsRequest")]
#[proto_query(
    path = "/interchain_security.ccv.provider.v1.Query/QueryConsumerChains",
    response_type = QueryConsumerChainsResponse
)]
pub struct QueryConsumerChainsRequest {}
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.QueryConsumerChainsResponse")]
pub struct QueryConsumerChainsResponse {
    #[prost(message, repeated, tag = "1")]
    pub chains: ::prost::alloc::vec::Vec<Chain>,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.QueryConsumerChainStartProposalsRequest"
)]
#[proto_query(
    path = "/interchain_security.ccv.provider.v1.Query/QueryConsumerChainStarts",
    response_type = QueryConsumerChainStartProposalsResponse
)]
pub struct QueryConsumerChainStartProposalsRequest {}
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.QueryConsumerChainStartProposalsResponse"
)]
pub struct QueryConsumerChainStartProposalsResponse {
    #[prost(message, optional, tag = "1")]
    pub proposals: ::core::option::Option<ConsumerAdditionProposals>,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.QueryConsumerChainStopProposalsRequest"
)]
#[proto_query(
    path = "/interchain_security.ccv.provider.v1.Query/QueryConsumerChainStops",
    response_type = QueryConsumerChainStopProposalsResponse
)]
pub struct QueryConsumerChainStopProposalsRequest {}
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.QueryConsumerChainStopProposalsResponse"
)]
pub struct QueryConsumerChainStopProposalsResponse {
    #[prost(message, optional, tag = "1")]
    pub proposals: ::core::option::Option<ConsumerRemovalProposals>,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.Chain")]
pub struct Chain {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    /// If chain with `chainID` is a Top-N chain, i.e., enforces at least one validator to validate chain `chainID`
    #[prost(uint32, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub top_n: u32,
    /// If the chain is a Top-N chain, this is the minimum power required to be in the top N.
    /// Otherwise, this is -1.
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub min_power_in_top_n: i64,
    /// Corresponds to the maximum power (percentage-wise) a validator can have on the consumer chain.
    #[prost(uint32, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub validators_power_cap: u32,
    /// Corresponds to the maximum number of validators that can validate a consumer chain.
    /// Only applicable to Opt In chains. Setting `validator_set_cap` on a Top N chain is a no-op.
    #[prost(uint32, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub validator_set_cap: u32,
    /// Corresponds to a list of provider consensus addresses of validators that are the ONLY ones that can validate
    /// the consumer chain.
    #[prost(string, repeated, tag = "7")]
    pub allowlist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Corresponds to a list of provider consensus addresses of validators that CANNOT validate the consumer chain.
    #[prost(string, repeated, tag = "8")]
    pub denylist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.QueryValidatorConsumerAddrRequest"
)]
#[proto_query(
    path = "/interchain_security.ccv.provider.v1.Query/QueryValidatorConsumerAddr",
    response_type = QueryValidatorConsumerAddrResponse
)]
pub struct QueryValidatorConsumerAddrRequest {
    /// The id of the consumer chain
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// The consensus address of the validator on the provider chain
    #[prost(string, tag = "2")]
    pub provider_address: ::prost::alloc::string::String,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.QueryValidatorConsumerAddrResponse"
)]
pub struct QueryValidatorConsumerAddrResponse {
    /// The address of the validator on the consumer chain
    #[prost(string, tag = "1")]
    pub consumer_address: ::prost::alloc::string::String,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.QueryValidatorProviderAddrRequest"
)]
#[proto_query(
    path = "/interchain_security.ccv.provider.v1.Query/QueryValidatorProviderAddr",
    response_type = QueryValidatorProviderAddrResponse
)]
pub struct QueryValidatorProviderAddrRequest {
    /// The id of the provider chain
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// The consensus address of the validator on the consumer chain
    #[prost(string, tag = "2")]
    pub consumer_address: ::prost::alloc::string::String,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.QueryValidatorProviderAddrResponse"
)]
pub struct QueryValidatorProviderAddrResponse {
    /// The address of the validator on the provider chain
    #[prost(string, tag = "1")]
    pub provider_address: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.QueryThrottleStateRequest")]
#[proto_query(
    path = "/interchain_security.ccv.provider.v1.Query/QueryThrottleState",
    response_type = QueryThrottleStateResponse
)]
pub struct QueryThrottleStateRequest {}
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.QueryThrottleStateResponse")]
pub struct QueryThrottleStateResponse {
    /// current slash_meter state
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub slash_meter: i64,
    /// allowance of voting power units (int) that the slash meter is given per
    /// replenish period this also serves as the max value for the meter.
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub slash_meter_allowance: i64,
    /// next time the slash meter could potentially be replenished, iff it's not
    /// full
    #[prost(message, optional, tag = "3")]
    pub next_replenish_candidate: ::core::option::Option<crate::shim::Timestamp>,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.QueryRegisteredConsumerRewardDenomsRequest"
)]
#[proto_query(
    path = "/interchain_security.ccv.provider.v1.Query/QueryRegisteredConsumerRewardDenoms",
    response_type = QueryRegisteredConsumerRewardDenomsResponse
)]
pub struct QueryRegisteredConsumerRewardDenomsRequest {}
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.QueryRegisteredConsumerRewardDenomsResponse"
)]
pub struct QueryRegisteredConsumerRewardDenomsResponse {
    #[prost(string, repeated, tag = "1")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.QueryProposedChainIDsRequest")]
#[proto_query(
    path = "/interchain_security.ccv.provider.v1.Query/QueryProposedConsumerChainIDs",
    response_type = QueryProposedChainIDsResponse
)]
pub struct QueryProposedChainIDsRequest {}
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.QueryProposedChainIDsResponse")]
pub struct QueryProposedChainIDsResponse {
    #[prost(message, repeated, tag = "1")]
    pub proposed_chains: ::prost::alloc::vec::Vec<ProposedChain>,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.ProposedChain")]
pub struct ProposedChain {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.QueryAllPairsValConAddrByConsumerChainIDRequest"
)]
#[proto_query(
    path = "/interchain_security.ccv.provider.v1.Query/QueryAllPairsValConAddrByConsumerChainID",
    response_type = QueryAllPairsValConAddrByConsumerChainIdResponse
)]
pub struct QueryAllPairsValConAddrByConsumerChainIdRequest {
    /// The id of the consumer chain
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.QueryAllPairsValConAddrByConsumerChainIDResponse"
)]
pub struct QueryAllPairsValConAddrByConsumerChainIdResponse {
    #[prost(message, repeated, tag = "1")]
    pub pair_val_con_addr: ::prost::alloc::vec::Vec<PairValConAddrProviderAndConsumer>,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.PairValConAddrProviderAndConsumer"
)]
pub struct PairValConAddrProviderAndConsumer {
    /// The consensus address of the validator on the provider chain
    #[prost(string, tag = "1")]
    pub provider_address: ::prost::alloc::string::String,
    /// The consensus address of the validator on the consumer chain
    #[prost(string, tag = "2")]
    pub consumer_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub consumer_key:
        ::core::option::Option<super::super::super::super::tendermint::crypto::PublicKey>,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.QueryParamsRequest")]
#[proto_query(
    path = "/interchain_security.ccv.provider.v1.Query/QueryParams",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.QueryConsumerChainOptedInValidatorsRequest"
)]
#[proto_query(
    path = "/interchain_security.ccv.provider.v1.Query/QueryConsumerChainOptedInValidators",
    response_type = QueryConsumerChainOptedInValidatorsResponse
)]
pub struct QueryConsumerChainOptedInValidatorsRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.QueryConsumerChainOptedInValidatorsResponse"
)]
pub struct QueryConsumerChainOptedInValidatorsResponse {
    /// The consensus addresses of the validators on the provider chain
    #[prost(string, repeated, tag = "1")]
    pub validators_provider_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.QueryConsumerValidatorsRequest")]
#[proto_query(
    path = "/interchain_security.ccv.provider.v1.Query/QueryConsumerValidators",
    response_type = QueryConsumerValidatorsResponse
)]
pub struct QueryConsumerValidatorsRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.QueryConsumerValidatorsValidator")]
pub struct QueryConsumerValidatorsValidator {
    /// The consensus address of the validator on the provider chain
    #[prost(string, tag = "1")]
    pub provider_address: ::prost::alloc::string::String,
    /// The consumer public key of the validator used on the consumer chain
    #[prost(message, optional, tag = "2")]
    pub consumer_key:
        ::core::option::Option<super::super::super::super::tendermint::crypto::PublicKey>,
    /// The power of the validator used on the consumer chain
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub power: i64,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.QueryConsumerValidatorsResponse")]
pub struct QueryConsumerValidatorsResponse {
    #[prost(message, repeated, tag = "1")]
    pub validators: ::prost::alloc::vec::Vec<QueryConsumerValidatorsValidator>,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.QueryConsumerChainsValidatorHasToValidateRequest"
)]
#[proto_query(
    path = "/interchain_security.ccv.provider.v1.Query/QueryConsumerChainsValidatorHasToValidate",
    response_type = QueryConsumerChainsValidatorHasToValidateResponse
)]
pub struct QueryConsumerChainsValidatorHasToValidateRequest {
    /// The consensus address of the validator on the provider chain
    #[prost(string, tag = "1")]
    pub provider_address: ::prost::alloc::string::String,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.QueryConsumerChainsValidatorHasToValidateResponse"
)]
pub struct QueryConsumerChainsValidatorHasToValidateResponse {
    #[prost(string, repeated, tag = "1")]
    #[serde(alias = "consumer_chainIDs")]
    pub consumer_chain_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.QueryValidatorConsumerCommissionRateRequest"
)]
#[proto_query(
    path = "/interchain_security.ccv.provider.v1.Query/QueryValidatorConsumerCommissionRate",
    response_type = QueryValidatorConsumerCommissionRateResponse
)]
pub struct QueryValidatorConsumerCommissionRateRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// The consensus address of the validator on the provider chain
    #[prost(string, tag = "2")]
    pub provider_address: ::prost::alloc::string::String,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.QueryValidatorConsumerCommissionRateResponse"
)]
pub struct QueryValidatorConsumerCommissionRateResponse {
    /// The rate to charge delegators on the consumer chain, as a fraction
    #[prost(string, tag = "1")]
    pub rate: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.QueryOldestUnconfirmedVscRequest")]
#[proto_query(
    path = "/interchain_security.ccv.provider.v1.Query/QueryOldestUnconfirmedVsc",
    response_type = QueryOldestUnconfirmedVscResponse
)]
pub struct QueryOldestUnconfirmedVscRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.QueryOldestUnconfirmedVscResponse"
)]
pub struct QueryOldestUnconfirmedVscResponse {
    #[prost(message, optional, tag = "1")]
    pub vsc_send_timestamp: ::core::option::Option<VscSendTimestamp>,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MsgAssignConsumerKey")]
pub struct MsgAssignConsumerKey {
    /// The chain id of the consumer chain to assign a consensus public key to
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// The validator address on the provider
    #[prost(string, tag = "2")]
    pub provider_addr: ::prost::alloc::string::String,
    /// The consensus public key to use on the consumer.
    /// in json string format corresponding to proto-any, ex:
    /// `{"@type":"/cosmos.crypto.ed25519.PubKey","key":"Ui5Gf1+mtWUdH8u3xlmzdKID+F3PK0sfXZ73GZ6q6is="}`
    #[prost(string, tag = "3")]
    pub consumer_key: ::prost::alloc::string::String,
    /// Tx signer address
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MsgAssignConsumerKeyResponse")]
pub struct MsgAssignConsumerKeyResponse {}
/// MsgSubmitConsumerMisbehaviour defines a message that reports a light client attack,
/// also known as a misbehaviour, observed on a consumer chain
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MsgSubmitConsumerMisbehaviour")]
pub struct MsgSubmitConsumerMisbehaviour {
    #[prost(string, tag = "1")]
    pub submitter: ::prost::alloc::string::String,
    /// The Misbehaviour of the consumer chain wrapping
    /// two conflicting IBC headers
    #[prost(message, optional, tag = "2")]
    pub misbehaviour: ::core::option::Option<
        super::super::super::super::ibc::lightclients::tendermint::v1::Misbehaviour,
    >,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.MsgSubmitConsumerMisbehaviourResponse"
)]
pub struct MsgSubmitConsumerMisbehaviourResponse {}
/// MsgSubmitConsumerDoubleVoting defines a message that reports
/// a double signing infraction observed on a consumer chain
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MsgSubmitConsumerDoubleVoting")]
pub struct MsgSubmitConsumerDoubleVoting {
    #[prost(string, tag = "1")]
    pub submitter: ::prost::alloc::string::String,
    /// The equivocation of the consumer chain wrapping
    /// an evidence of a validator that signed two conflicting votes
    #[prost(message, optional, tag = "2")]
    pub duplicate_vote_evidence: ::core::option::Option<
        super::super::super::super::tendermint::types::DuplicateVoteEvidence,
    >,
    /// The light client header of the infraction block
    #[prost(message, optional, tag = "3")]
    pub infraction_block_header: ::core::option::Option<
        super::super::super::super::ibc::lightclients::tendermint::v1::Header,
    >,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.MsgSubmitConsumerDoubleVotingResponse"
)]
pub struct MsgSubmitConsumerDoubleVotingResponse {}
/// MsgUpdateParams is the Msg/UpdateParams request type
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// signer is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/provider parameters to update.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
/// MsgConsumerAddition defines the message used to spawn a new consumer chain using a v1 governance proposal.
/// If it passes, then all validators on the provider chain are expected to validate
/// the consumer chain at spawn time or get slashed.
/// It is recommended that spawn time occurs after the proposal end time.
///
/// Note: this replaces ConsumerAdditionProposal which is deprecated and will be removed soon
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MsgConsumerAddition")]
pub struct MsgConsumerAddition {
    /// the proposed chain-id of the new consumer chain, must be different from all
    /// other consumer chain ids of the executing provider chain.
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// the proposed initial height of new consumer chain.
    /// For a completely new chain, this will be {0,1}. However, it may be
    /// different if this is a chain that is converting to a consumer chain.
    #[prost(message, optional, tag = "2")]
    pub initial_height:
        ::core::option::Option<super::super::super::super::ibc::core::client::v1::Height>,
    /// The hash of the consumer chain genesis state without the consumer CCV
    /// module genesis params. It is used for off-chain confirmation of
    /// genesis.json validity by validators and other parties.
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub genesis_hash: ::prost::alloc::vec::Vec<u8>,
    /// The hash of the consumer chain binary that should be run by validators on
    /// chain initialization. It is used for off-chain confirmation of binary
    /// validity by validators and other parties.
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub binary_hash: ::prost::alloc::vec::Vec<u8>,
    /// spawn time is the time on the provider chain at which the consumer chain
    /// genesis is finalized and all validators will be responsible for starting
    /// their consumer chain validator node.
    #[prost(message, optional, tag = "5")]
    pub spawn_time: ::core::option::Option<crate::shim::Timestamp>,
    /// Unbonding period for the consumer,
    /// which should be smaller than that of the provider in general.
    #[prost(message, optional, tag = "6")]
    pub unbonding_period: ::core::option::Option<crate::shim::Duration>,
    /// Sent CCV related IBC packets will timeout after this duration
    #[prost(message, optional, tag = "7")]
    pub ccv_timeout_period: ::core::option::Option<crate::shim::Duration>,
    /// Sent transfer related IBC packets will timeout after this duration
    #[prost(message, optional, tag = "8")]
    pub transfer_timeout_period: ::core::option::Option<crate::shim::Duration>,
    /// The fraction of tokens allocated to the consumer redistribution address
    /// during distribution events. The fraction is a string representing a
    /// decimal number. For example "0.75" would represent 75%.
    #[prost(string, tag = "9")]
    pub consumer_redistribution_fraction: ::prost::alloc::string::String,
    /// BlocksPerDistributionTransmission is the number of blocks between
    /// ibc-token-transfers from the consumer chain to the provider chain. On
    /// sending transmission event, `consumer_redistribution_fraction` of the
    /// accumulated tokens are sent to the consumer redistribution address.
    #[prost(int64, tag = "10")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub blocks_per_distribution_transmission: i64,
    /// The number of historical info entries to persist in store.
    /// This param is a part of the cosmos sdk staking module. In the case of
    /// a ccv enabled consumer chain, the ccv module acts as the staking module.
    #[prost(int64, tag = "11")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub historical_entries: i64,
    /// The ID of a token transfer channel used for the Reward Distribution
    /// sub-protocol. If DistributionTransmissionChannel == "", a new transfer
    /// channel is created on top of the same connection as the CCV channel.
    /// Note that transfer_channel_id is the ID of the channel end on the consumer
    /// chain. it is most relevant for chains performing a sovereign to consumer
    /// changeover in order to maintan the existing ibc transfer channel
    #[prost(string, tag = "12")]
    pub distribution_transmission_channel: ::prost::alloc::string::String,
    /// Corresponds to the percentage of validators that have to validate the chain under the Top N case.
    /// For example, 53 corresponds to a Top 53% chain, meaning that the top 53% provider validators by voting power
    /// have to validate the proposed consumer chain. top_N can either be 0 or any value in [50, 100].
    /// A chain can join with top_N == 0 as an Opt In chain, or with top_N ∈ [50, 100] as a Top N chain.
    #[prost(uint32, tag = "13")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub top_n: u32,
    /// Corresponds to the maximum power (percentage-wise) a validator can have on the consumer chain. For instance, if
    /// `validators_power_cap` is set to 32, it means that no validator can have more than 32% of the voting power on the
    /// consumer chain. Note that this might not be feasible. For example, think of a consumer chain with only
    /// 5 validators and with `validators_power_cap` set to 10%. In such a scenario, at least one validator would need
    /// to have more than 20% of the total voting power. Therefore, `validators_power_cap` operates on a best-effort basis.
    #[prost(uint32, tag = "14")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub validators_power_cap: u32,
    /// Corresponds to the maximum number of validators that can validate a consumer chain.
    /// Only applicable to Opt In chains. Setting `validator_set_cap` on a Top N chain is a no-op.
    #[prost(uint32, tag = "15")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub validator_set_cap: u32,
    /// Corresponds to a list of provider consensus addresses of validators that are the ONLY ones that can validate
    /// the consumer chain.
    #[prost(string, repeated, tag = "16")]
    pub allowlist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Corresponds to a list of provider consensus addresses of validators that CANNOT validate the consumer chain.
    #[prost(string, repeated, tag = "17")]
    pub denylist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// signer address
    #[prost(string, tag = "18")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgConsumerAdditionResponse defines response type for MsgConsumerAddition messages
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MsgConsumerAdditionResponse")]
pub struct MsgConsumerAdditionResponse {}
/// MsgConsumerRemoval message contains a governance proposal on the provider chain to
/// remove (and stop) a consumer chain. If it passes, all the consumer chain's
/// state is removed from the provider chain. The outstanding unbonding operation
/// funds are released.
///
/// Note: this replaces ConsumerRemovalProposal which is deprecated and will be removed soon
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MsgConsumerRemoval")]
pub struct MsgConsumerRemoval {
    /// the chain-id of the consumer chain to be stopped
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// the time on the provider chain at which all validators are responsible to
    /// stop their consumer chain validator node
    #[prost(message, optional, tag = "2")]
    pub stop_time: ::core::option::Option<crate::shim::Timestamp>,
    /// signer address
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgConsumerRemovalResponse defines response type for MsgConsumerRemoval messages
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MsgConsumerRemovalResponse")]
pub struct MsgConsumerRemovalResponse {}
/// ChangeRewardDenomsProposal is a governance proposal on the provider chain to
/// mutate the set of denoms accepted by the provider as rewards.
///
/// Note: this replaces ChangeRewardDenomsProposal which is deprecated and will be removed soon
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MsgChangeRewardDenoms")]
pub struct MsgChangeRewardDenoms {
    /// the list of consumer reward denoms to add
    #[prost(string, repeated, tag = "1")]
    pub denoms_to_add: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the list of consumer reward denoms to remove
    #[prost(string, repeated, tag = "2")]
    pub denoms_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// signer address
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgChangeRewardDenomsResponse defines response type for MsgChangeRewardDenoms messages
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MsgChangeRewardDenomsResponse")]
pub struct MsgChangeRewardDenomsResponse {}
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MsgOptIn")]
pub struct MsgOptIn {
    /// the chain id of the consumer chain to opt in to
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// the validator address on the provider
    #[prost(string, tag = "2")]
    pub provider_addr: ::prost::alloc::string::String,
    /// (optional) The consensus public key to use on the consumer in json string format corresponding to proto-any,
    /// for example `{"@type":"/cosmos.crypto.ed25519.PubKey","key":"Ui5Gf1+mtWUdH8u3xlmzdKID+F3PK0sfXZ73GZ6q6is="}`.
    /// This field is optional and can remain empty (i.e., `consumer_key = ""`). A validator can always change the
    /// consumer public key at a later stage by issuing a `MsgAssignConsumerKey` message.
    #[prost(string, tag = "3")]
    pub consumer_key: ::prost::alloc::string::String,
    /// signer address
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MsgOptInResponse")]
pub struct MsgOptInResponse {}
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MsgOptOut")]
pub struct MsgOptOut {
    /// the chain id of the consumer chain to opt out from
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// the validator address on the provider
    #[prost(string, tag = "2")]
    pub provider_addr: ::prost::alloc::string::String,
    /// signer address
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MsgOptOutResponse")]
pub struct MsgOptOutResponse {}
/// MsgSetConsumerCommissionRate allows validators to set
/// a per-consumer chain commission rate
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MsgSetConsumerCommissionRate")]
pub struct MsgSetConsumerCommissionRate {
    /// The validator address on the provider
    #[prost(string, tag = "1")]
    pub provider_addr: ::prost::alloc::string::String,
    /// The chain id of the consumer chain to set a commission rate
    #[prost(string, tag = "2")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// The rate to charge delegators on the consumer chain, as a fraction
    /// TODO: migrate rate from sdk.Dec to math.LegacyDec
    #[prost(string, tag = "3")]
    pub rate: ::prost::alloc::string::String,
    /// signer address
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
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
#[proto_message(
    type_url = "/interchain_security.ccv.provider.v1.MsgSetConsumerCommissionRateResponse"
)]
pub struct MsgSetConsumerCommissionRateResponse {}
/// MsgConsumerModification message contains a governance proposal on the provider chain to
/// modify a running consumer chain. If it passes, the consumer chain's
/// parameters are updated.
///
/// Note: this replaces ConsumerModificationProposal which is deprecated and will be removed soon
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MsgConsumerModification")]
pub struct MsgConsumerModification {
    /// the title of the proposal
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// the description of the proposal
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// the chain-id of the consumer chain to be modified
    #[prost(string, tag = "3")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// Corresponds to the percentage of validators that have to validate the chain under the Top N case.
    /// For example, 53 corresponds to a Top 53% chain, meaning that the top 53% provider validators by voting power
    /// have to validate the proposed consumer chain. top_N can either be 0 or any value in [50, 100].
    /// A chain can join with top_N == 0 as an Opt In chain, or with top_N ∈ [50, 100] as a Top N chain.
    #[prost(uint32, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub top_n: u32,
    /// Corresponds to the maximum power (percentage-wise) a validator can have on the consumer chain. For instance, if
    /// `validators_power_cap` is set to 32, it means that no validator can have more than 32% of the voting power on the
    /// consumer chain. Note that this might not be feasible. For example, think of a consumer chain with only
    /// 5 validators and with `validators_power_cap` set to 10%. In such a scenario, at least one validator would need
    /// to have more than 20% of the total voting power. Therefore, `validators_power_cap` operates on a best-effort basis.
    #[prost(uint32, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub validators_power_cap: u32,
    /// Corresponds to the maximum number of validators that can validate a consumer chain.
    /// Only applicable to Opt In chains. Setting `validator_set_cap` on a Top N chain is a no-op.
    #[prost(uint32, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub validator_set_cap: u32,
    /// Corresponds to a list of provider consensus addresses of validators that are the ONLY ones that can validate
    /// the consumer chain.
    #[prost(string, repeated, tag = "7")]
    pub allowlist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Corresponds to a list of provider consensus addresses of validators that CANNOT validate the consumer chain.
    #[prost(string, repeated, tag = "8")]
    pub denylist: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// signer address
    #[prost(string, tag = "9")]
    pub authority: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/interchain_security.ccv.provider.v1.MsgConsumerModificationResponse")]
pub struct MsgConsumerModificationResponse {}
pub struct ProviderQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ProviderQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn query_consumer_genesis(
        &self,
        chain_id: ::prost::alloc::string::String,
    ) -> Result<QueryConsumerGenesisResponse, cosmwasm_std::StdError> {
        QueryConsumerGenesisRequest { chain_id }.query(self.querier)
    }
    pub fn query_consumer_chains(
        &self,
    ) -> Result<QueryConsumerChainsResponse, cosmwasm_std::StdError> {
        QueryConsumerChainsRequest {}.query(self.querier)
    }
    pub fn query_consumer_chain_starts(
        &self,
    ) -> Result<QueryConsumerChainStartProposalsResponse, cosmwasm_std::StdError> {
        QueryConsumerChainStartProposalsRequest {}.query(self.querier)
    }
    pub fn query_consumer_chain_stops(
        &self,
    ) -> Result<QueryConsumerChainStopProposalsResponse, cosmwasm_std::StdError> {
        QueryConsumerChainStopProposalsRequest {}.query(self.querier)
    }
    pub fn query_validator_consumer_addr(
        &self,
        chain_id: ::prost::alloc::string::String,
        provider_address: ::prost::alloc::string::String,
    ) -> Result<QueryValidatorConsumerAddrResponse, cosmwasm_std::StdError> {
        QueryValidatorConsumerAddrRequest {
            chain_id,
            provider_address,
        }
        .query(self.querier)
    }
    pub fn query_validator_provider_addr(
        &self,
        chain_id: ::prost::alloc::string::String,
        consumer_address: ::prost::alloc::string::String,
    ) -> Result<QueryValidatorProviderAddrResponse, cosmwasm_std::StdError> {
        QueryValidatorProviderAddrRequest {
            chain_id,
            consumer_address,
        }
        .query(self.querier)
    }
    pub fn query_throttle_state(
        &self,
    ) -> Result<QueryThrottleStateResponse, cosmwasm_std::StdError> {
        QueryThrottleStateRequest {}.query(self.querier)
    }
    pub fn query_registered_consumer_reward_denoms(
        &self,
    ) -> Result<QueryRegisteredConsumerRewardDenomsResponse, cosmwasm_std::StdError> {
        QueryRegisteredConsumerRewardDenomsRequest {}.query(self.querier)
    }
    pub fn query_proposed_consumer_chain_i_ds(
        &self,
    ) -> Result<QueryProposedChainIDsResponse, cosmwasm_std::StdError> {
        QueryProposedChainIDsRequest {}.query(self.querier)
    }
    pub fn query_all_pairs_val_con_addr_by_consumer_chain_id(
        &self,
        chain_id: ::prost::alloc::string::String,
    ) -> Result<QueryAllPairsValConAddrByConsumerChainIdResponse, cosmwasm_std::StdError> {
        QueryAllPairsValConAddrByConsumerChainIdRequest { chain_id }.query(self.querier)
    }
    pub fn query_params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn query_consumer_chain_opted_in_validators(
        &self,
        chain_id: ::prost::alloc::string::String,
    ) -> Result<QueryConsumerChainOptedInValidatorsResponse, cosmwasm_std::StdError> {
        QueryConsumerChainOptedInValidatorsRequest { chain_id }.query(self.querier)
    }
    pub fn query_consumer_chains_validator_has_to_validate(
        &self,
        provider_address: ::prost::alloc::string::String,
    ) -> Result<QueryConsumerChainsValidatorHasToValidateResponse, cosmwasm_std::StdError> {
        QueryConsumerChainsValidatorHasToValidateRequest { provider_address }.query(self.querier)
    }
    pub fn query_validator_consumer_commission_rate(
        &self,
        chain_id: ::prost::alloc::string::String,
        provider_address: ::prost::alloc::string::String,
    ) -> Result<QueryValidatorConsumerCommissionRateResponse, cosmwasm_std::StdError> {
        QueryValidatorConsumerCommissionRateRequest {
            chain_id,
            provider_address,
        }
        .query(self.querier)
    }
    pub fn query_oldest_unconfirmed_vsc(
        &self,
        chain_id: ::prost::alloc::string::String,
    ) -> Result<QueryOldestUnconfirmedVscResponse, cosmwasm_std::StdError> {
        QueryOldestUnconfirmedVscRequest { chain_id }.query(self.querier)
    }
    pub fn query_consumer_validators(
        &self,
        chain_id: ::prost::alloc::string::String,
    ) -> Result<QueryConsumerValidatorsResponse, cosmwasm_std::StdError> {
        QueryConsumerValidatorsRequest { chain_id }.query(self.querier)
    }
}
