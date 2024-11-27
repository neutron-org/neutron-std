use neutron_std_derive::CosmwasmExt;
/// CrossChainValidator defines the type used to store validator information
/// internal to the consumer CCV module.  Note one cross chain validator entry is
/// persisted for each consumer validator, where incoming VSC packets update this
/// data, which is eventually forwarded to comet for consumer chain consensus.
///
/// Note this type is only used internally to the consumer CCV module.
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
#[proto_message(type_url = "/interchain_security.ccv.consumer.v1.CrossChainValidator")]
pub struct CrossChainValidator {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub power: i64,
    /// pubkey is the consensus public key of the validator, as a Protobuf Any.
    #[prost(message, optional, tag = "3")]
    pub pubkey: ::core::option::Option<crate::shim::Any>,
    /// !!! DEPRECATED !!! opted_out is deprecated because after the introduction of Partial Set Security (PSS)
    /// we removed the soft opt-out feature.
    #[deprecated]
    #[prost(bool, tag = "4")]
    pub opted_out: bool,
}
/// A record storing the state of a slash packet sent to the provider chain
/// which may bounce back and forth until handled by the provider.
///
/// Note this type is only used internally to the consumer CCV module.
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
#[proto_message(type_url = "/interchain_security.ccv.consumer.v1.SlashRecord")]
pub struct SlashRecord {
    #[prost(bool, tag = "1")]
    pub waiting_on_reply: bool,
    #[prost(message, optional, tag = "2")]
    pub send_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// GenesisState defines the CCV consumer genesis state
///
/// Note: this type is only used on consumer side and references shared types with
/// provider
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
#[proto_message(type_url = "/interchain_security.ccv.consumer.v1.GenesisState")]
pub struct GenesisState {
    /// ConsumerParams is a shared type with provider module
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<super::super::v1::ConsumerParams>,
    /// Client ID of the provider. Empty for a new chain, filled in on restart.
    #[prost(string, tag = "2")]
    #[serde(alias = "provider_clientID")]
    pub provider_client_id: ::prost::alloc::string::String,
    /// Channel ID of the provider. Empty for a new chain, filled in on restart.
    #[prost(string, tag = "3")]
    #[serde(alias = "provider_channelID")]
    pub provider_channel_id: ::prost::alloc::string::String,
    /// true for new chain, false for chain restart.
    #[prost(bool, tag = "4")]
    pub new_chain: bool,
    /// !!! DEPRECATED !!! ProviderClientState is deprecated. Use provider.client_state instead
    #[deprecated]
    #[prost(message, optional, tag = "5")]
    pub provider_client_state: ::core::option::Option<
        super::super::super::super::ibc::lightclients::tendermint::v1::ClientState,
    >,
    /// !!! DEPRECATED !!! ProviderConsensusState is deprecated. Use provider.consensus_state instead
    #[deprecated]
    #[prost(message, optional, tag = "6")]
    pub provider_consensus_state: ::core::option::Option<
        super::super::super::super::ibc::lightclients::tendermint::v1::ConsensusState,
    >,
    /// MaturingPackets nil on new chain, filled in on restart.
    #[prost(message, repeated, tag = "7")]
    pub maturing_packets: ::prost::alloc::vec::Vec<MaturingVscPacket>,
    /// !!! DEPRECATED !!!! InitialValset is deprecated. Use provider.initial_val_set instead
    #[deprecated]
    #[prost(message, repeated, tag = "8")]
    pub initial_val_set:
        ::prost::alloc::vec::Vec<super::super::super::super::tendermint::abci::ValidatorUpdate>,
    /// HeightToValsetUpdateId nil on new chain, filled in on restart.
    #[prost(message, repeated, tag = "9")]
    #[serde(alias = "height_to_valset_updateID")]
    pub height_to_valset_update_id: ::prost::alloc::vec::Vec<HeightToValsetUpdateId>,
    /// OutstandingDowntimes nil on new chain, filled  in on restart.
    #[prost(message, repeated, tag = "10")]
    pub outstanding_downtime_slashing: ::prost::alloc::vec::Vec<OutstandingDowntime>,
    /// PendingConsumerPackets nil on new chain, filled in on restart.
    #[prost(message, optional, tag = "11")]
    pub pending_consumer_packets: ::core::option::Option<ConsumerPacketDataList>,
    /// LastTransmissionBlockHeight nil on new chain, filled in on restart.
    #[prost(message, optional, tag = "12")]
    pub last_transmission_block_height: ::core::option::Option<LastTransmissionBlockHeight>,
    /// flag indicating whether the consumer CCV module starts in pre-CCV state
    #[prost(bool, tag = "13")]
    pub pre_ccv: bool,
    #[prost(message, optional, tag = "14")]
    pub provider: ::core::option::Option<super::super::v1::ProviderInfo>,
}
/// HeightValsetUpdateID represents a mapping internal to the consumer CCV module
/// which links a block height to each recv valset update id.
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
#[proto_message(type_url = "/interchain_security.ccv.consumer.v1.HeightToValsetUpdateID")]
pub struct HeightToValsetUpdateId {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
    #[prost(uint64, tag = "2")]
    #[serde(alias = "valset_updateID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub valset_update_id: u64,
}
/// OutstandingDowntime defines the type used internally to the consumer CCV
/// module and is used in order to not send multiple slashing requests for
/// the same downtime infraction.
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
#[proto_message(type_url = "/interchain_security.ccv.consumer.v1.OutstandingDowntime")]
pub struct OutstandingDowntime {
    #[prost(string, tag = "1")]
    pub validator_consensus_address: ::prost::alloc::string::String,
}
/// LastTransmissionBlockHeight is the last time validator holding
/// pools were transmitted to the provider chain. This type is used internally
/// to the consumer CCV module.
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
#[proto_message(type_url = "/interchain_security.ccv.consumer.v1.LastTransmissionBlockHeight")]
pub struct LastTransmissionBlockHeight {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
}
/// MaturingVSCPacket represents a vsc packet that is maturing internal to the
/// consumer CCV module, where the consumer has not yet relayed a VSCMatured
/// packet back to the provider.
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
#[proto_message(type_url = "/interchain_security.ccv.consumer.v1.MaturingVSCPacket")]
pub struct MaturingVscPacket {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "vscID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub vsc_id: u64,
    #[prost(message, optional, tag = "2")]
    pub maturity_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// ConsumerPacketDataList is a list of consumer packet data packets.
///
/// Note this type is used internally to the consumer CCV module
/// for exporting / importing state in InitGenesis and ExportGenesis.
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
#[proto_message(type_url = "/interchain_security.ccv.consumer.v1.ConsumerPacketDataList")]
pub struct ConsumerPacketDataList {
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<super::super::v1::ConsumerPacketData>,
}
/// NextFeeDistributionEstimate holds information about next fee distribution
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
#[proto_message(type_url = "/interchain_security.ccv.consumer.v1.NextFeeDistributionEstimate")]
pub struct NextFeeDistributionEstimate {
    /// current block height at the time of querying
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub current_height: i64,
    /// block height at which last distribution took place
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_height: i64,
    /// block height at which next distribution will take place
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub next_height: i64,
    /// ratio between consumer and provider fee distribution
    #[prost(string, tag = "4")]
    pub distribution_fraction: ::prost::alloc::string::String,
    /// total accruead fees at the time of querying
    #[prost(string, tag = "5")]
    pub total: ::prost::alloc::string::String,
    /// amount distributed to provider chain
    #[prost(string, tag = "6")]
    pub to_provider: ::prost::alloc::string::String,
    /// amount distributed (kept) by consumer chain
    #[prost(string, tag = "7")]
    pub to_consumer: ::prost::alloc::string::String,
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
    type_url = "/interchain_security.ccv.consumer.v1.QueryNextFeeDistributionEstimateRequest"
)]
#[proto_query(
    path = "/interchain_security.ccv.consumer.v1.Query/QueryNextFeeDistribution",
    response_type = QueryNextFeeDistributionEstimateResponse
)]
pub struct QueryNextFeeDistributionEstimateRequest {}
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
    type_url = "/interchain_security.ccv.consumer.v1.QueryNextFeeDistributionEstimateResponse"
)]
pub struct QueryNextFeeDistributionEstimateResponse {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<NextFeeDistributionEstimate>,
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
#[proto_message(type_url = "/interchain_security.ccv.consumer.v1.QueryParamsRequest")]
#[proto_query(
    path = "/interchain_security.ccv.consumer.v1.Query/QueryParams",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
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
#[proto_message(type_url = "/interchain_security.ccv.consumer.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<super::super::v1::ConsumerParams>,
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
#[proto_message(type_url = "/interchain_security.ccv.consumer.v1.QueryProviderInfoRequest")]
#[proto_query(
    path = "/interchain_security.ccv.consumer.v1.Query/QueryProviderInfo",
    response_type = QueryProviderInfoResponse
)]
pub struct QueryProviderInfoRequest {}
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
#[proto_message(type_url = "/interchain_security.ccv.consumer.v1.QueryProviderInfoResponse")]
pub struct QueryProviderInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub consumer: ::core::option::Option<ChainInfo>,
    #[prost(message, optional, tag = "2")]
    pub provider: ::core::option::Option<ChainInfo>,
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
#[proto_message(type_url = "/interchain_security.ccv.consumer.v1.QueryThrottleStateRequest")]
#[proto_query(
    path = "/interchain_security.ccv.consumer.v1.Query/QueryThrottleState",
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
#[proto_message(type_url = "/interchain_security.ccv.consumer.v1.QueryThrottleStateResponse")]
pub struct QueryThrottleStateResponse {
    #[prost(message, optional, tag = "1")]
    pub slash_record: ::core::option::Option<SlashRecord>,
    #[prost(message, repeated, tag = "2")]
    pub packet_data_queue: ::prost::alloc::vec::Vec<super::super::v1::ConsumerPacketData>,
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
#[proto_message(type_url = "/interchain_security.ccv.consumer.v1.ChainInfo")]
pub struct ChainInfo {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
}
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
#[proto_message(type_url = "/interchain_security.ccv.consumer.v1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// signer is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/provider parameters to update.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<super::super::v1::ConsumerParams>,
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
#[proto_message(type_url = "/interchain_security.ccv.consumer.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct ConsumerQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ConsumerQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn query_next_fee_distribution(
        &self,
    ) -> Result<QueryNextFeeDistributionEstimateResponse, cosmwasm_std::StdError> {
        QueryNextFeeDistributionEstimateRequest {}.query(self.querier)
    }
    pub fn query_params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn query_provider_info(&self) -> Result<QueryProviderInfoResponse, cosmwasm_std::StdError> {
        QueryProviderInfoRequest {}.query(self.querier)
    }
    pub fn query_throttle_state(
        &self,
    ) -> Result<QueryThrottleStateResponse, cosmwasm_std::StdError> {
        QueryThrottleStateRequest {}.query(self.querier)
    }
}
