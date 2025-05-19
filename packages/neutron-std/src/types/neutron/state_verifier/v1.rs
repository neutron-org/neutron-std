use neutron_std_derive::CosmwasmExt;
/// Describes a "light" consensus state of the chain at a particular height
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
#[proto_message(type_url = "/neutron.state_verifier.v1.ConsensusState")]
pub struct ConsensusState {
    /// Describes a block height for which the consensus height is saved
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    /// ConsensusState defines the consensus state from Tendermint
    #[prost(message, optional, tag = "2")]
    pub cs: ::core::option::Option<
        super::super::super::ibc::lightclients::tendermint::v1::ConsensusState,
    >,
}
/// Defines the state verifier module's genesis state.
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
#[proto_message(type_url = "/neutron.state_verifier.v1.GenesisState")]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub states: ::prost::alloc::vec::Vec<ConsensusState>,
}
/// Describes a structure to verify storage values from the chain state from a particular height in the past
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
#[proto_message(type_url = "/neutron.state_verifier.v1.QueryVerifyStateValuesRequest")]
#[proto_query(
    path = "/neutron.state_verifier.v1.Query/VerifyStateValues",
    response_type = QueryVerifyStateValuesResponse
)]
pub struct QueryVerifyStateValuesRequest {
    /// Refers to the block height to which the storage values belong.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
    /// A slice of neutron.interchainqueries.StorageValue which relate to the specified height and must be verified against
    #[prost(message, repeated, tag = "2")]
    pub storage_values: ::prost::alloc::vec::Vec<super::super::interchainqueries::StorageValue>,
}
/// Describes a response structure for `VerifyStateValues` query
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
#[proto_message(type_url = "/neutron.state_verifier.v1.QueryVerifyStateValuesResponse")]
pub struct QueryVerifyStateValuesResponse {
    /// The field describes a validity of all the storage values passed to the request at a specific height
    #[prost(bool, tag = "1")]
    pub valid: bool,
}
/// Describes a structure to query ConsensusState by the specified height
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
#[proto_message(type_url = "/neutron.state_verifier.v1.QueryConsensusStateRequest")]
#[proto_query(
    path = "/neutron.state_verifier.v1.Query/QueryConsensusState",
    response_type = QueryConsensusStateResponse
)]
pub struct QueryConsensusStateRequest {
    /// Refers to the block height for which you want to query ConsensusState
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
}
/// Describes a response structure for `QueryConsensusStateRequest` query
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
#[proto_message(type_url = "/neutron.state_verifier.v1.QueryConsensusStateResponse")]
pub struct QueryConsensusStateResponse {
    /// ConsensusState defines the consensus state from the state-verifier module
    #[prost(message, optional, tag = "2")]
    pub cs: ::core::option::Option<ConsensusState>,
}
pub struct StateVerifierQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> StateVerifierQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn verify_state_values(
        &self,
        height: u64,
        storage_values: ::prost::alloc::vec::Vec<super::super::interchainqueries::StorageValue>,
    ) -> Result<QueryVerifyStateValuesResponse, cosmwasm_std::StdError> {
        QueryVerifyStateValuesRequest {
            height,
            storage_values,
        }
        .query(self.querier)
    }
    pub fn query_consensus_state(
        &self,
        height: u64,
    ) -> Result<QueryConsensusStateResponse, cosmwasm_std::StdError> {
        QueryConsensusStateRequest { height }.query(self.querier)
    }
}
