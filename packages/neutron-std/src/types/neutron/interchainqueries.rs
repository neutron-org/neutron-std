use neutron_std_derive::CosmwasmExt;
/// The parameters for the module.
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
#[proto_message(type_url = "/neutron.interchainqueries.Params")]
pub struct Params {
    /// The amount of blocks required to pass since an Interchain Query registration/update for the
    /// query to become available for removal by anybody.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub query_submit_timeout: u64,
    /// Amount of coins required to be provided as deposit on Interchain Query registration.
    #[prost(message, repeated, tag = "2")]
    pub query_deposit: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// Amount of tx hashes to be removed during a single EndBlock. Can vary to balance between
    /// network cleaning speed and EndBlock duration. A zero value means no limit.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tx_query_removal_limit: u64,
    /// Maximum amount of keys in a registered key value query
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_kv_query_keys_count: u64,
    /// max_transactions_filters defines maximum allowed amount of tx filters in msgRegisterInterchainQuery
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_transactions_filters: u64,
}
/// Information about an Interchain Query registered in the interchainqueries module.
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
#[proto_message(type_url = "/neutron.interchainqueries.RegisteredQuery")]
pub struct RegisteredQuery {
    /// The unique id of the registered query.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// The address of the contract that registered the query.
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// The query type identifier: `kv` or `tx`.
    #[prost(string, tag = "3")]
    pub query_type: ::prost::alloc::string::String,
    /// The KV-storage keys for which to get values from the remote chain. Only applicable for the
    /// KV Interchain Queries. Max amount of keys is limited by the module's `max_kv_query_keys_count`
    /// parameters.
    #[prost(message, repeated, tag = "4")]
    pub keys: ::prost::alloc::vec::Vec<KvKey>,
    /// A stringified list of filters for remote transactions search. Only applicable for the TX
    /// Interchain Queries. Example: "\[{\"field\":\"tx.height\",\"op\":\"Gte\",\"value\":2644737}\]".
    /// Supported operators: "eq", "lt", "gt", "lte", "gte". Max amount of filter conditions is limited
    /// by the module's `max_transactions_filters` parameters.
    #[prost(string, tag = "5")]
    pub transactions_filter: ::prost::alloc::string::String,
    /// The IBC connection ID to the remote chain (the source of querying data). Is used for getting
    /// ConsensusState from the respective IBC client to verify query result proofs.
    #[prost(string, tag = "6")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    /// Parameter that defines the minimal delay between consecutive query executions (i.e. the
    /// minimal delay between query results update).
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub update_period: u64,
    /// The local chain block height of the last query results update.
    #[prost(uint64, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_submitted_result_local_height: u64,
    /// The remote chain block height that corresponds to the last query result update.
    #[prost(message, optional, tag = "9")]
    pub last_submitted_result_remote_height:
        ::core::option::Option<super::super::ibc::core::client::v1::Height>,
    /// Amount of coins paid for the Interchain Query registration. The deposit is paid back to the
    /// remover. The remover can be either the query owner (during the submit timeout) or anybody.
    #[prost(message, repeated, tag = "10")]
    pub deposit: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
    /// Duration in blocks that is required to pass since the query registration/update for the
    /// query to become available for anybody to be removed.
    #[prost(uint64, tag = "11")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub submit_timeout: u64,
    /// The local chain block height of the Interchain Query registration.
    #[prost(uint64, tag = "12")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub registered_at_height: u64,
}
/// A path to an IAVL storage node.
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
#[proto_message(type_url = "/neutron.interchainqueries.KVKey")]
pub struct KvKey {
    /// The first half of the storage path. It is supposed to be a substore name for the query
    /// (e.g. bank, staking, etc.).
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// The second half of the storage path. The remaining part of the full path to an IAVL storage node.
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
/// The interchainqueries module's genesis state model.
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
#[proto_message(type_url = "/neutron.interchainqueries.GenesisState")]
pub struct GenesisState {
    /// The parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// A list of registered Interchain Queries.
    #[prost(message, repeated, tag = "2")]
    pub registered_queries: ::prost::alloc::vec::Vec<RegisteredQuery>,
}
/// Request type for the Msg/RegisterInterchainQuery RPC method.
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
#[proto_message(type_url = "/neutron.interchainqueries.MsgRegisterInterchainQuery")]
pub struct MsgRegisterInterchainQuery {
    /// The query type identifier: `kv` or `tx`.
    #[prost(string, tag = "1")]
    pub query_type: ::prost::alloc::string::String,
    /// The KV-storage keys for which we want to get values from remote chain. Only applicable for the
    /// KV Interchain Queries. Max amount of keys is limited by the module's `max_kv_query_keys_count`
    /// parameters.
    #[prost(message, repeated, tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<KvKey>,
    /// A stringified list of filters for remote transactions search. Only applicable for the TX
    /// Interchain Queries. Example: "\[{\"field\":\"tx.height\",\"op\":\"Gte\",\"value\":2644737}\]".
    /// Supported operators: "eq", "lt", "gt", "lte", "gte". Max amount of filter conditions is
    /// limited by the module's `max_transactions_filters` parameters.
    #[prost(string, tag = "3")]
    pub transactions_filter: ::prost::alloc::string::String,
    /// The IBC connection ID to the remote chain (the source of querying data). Is used for getting
    /// ConsensusState from the respective IBC client to verify query result proofs.
    #[prost(string, tag = "4")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    /// Parameter that defines the minimal delay between consecutive query executions (i.e. the
    /// minimal delay between query results update).
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub update_period: u64,
    /// The signer of the message.
    #[prost(string, tag = "6")]
    pub sender: ::prost::alloc::string::String,
}
/// Response type for the Msg/RegisterInterchainQuery RPC method.
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
#[proto_message(type_url = "/neutron.interchainqueries.MsgRegisterInterchainQueryResponse")]
pub struct MsgRegisterInterchainQueryResponse {
    /// The ID assigned to the registered Interchain Query by the module.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
}
/// Request type for the Msg/SubmitQueryResult RPC method.
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
#[proto_message(type_url = "/neutron.interchainqueries.MsgSubmitQueryResult")]
pub struct MsgSubmitQueryResult {
    /// The ID of the Interchain Query.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "queryID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub query_id: u64,
    /// The signer of the message.
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// The IBC client ID that corresponds to the IBC connection to the remote chain (where the
    /// query result is coming from).
    /// Deprecated: populating this field does not make any affect
    #[deprecated]
    #[prost(string, tag = "3")]
    #[serde(alias = "clientID")]
    pub client_id: ::prost::alloc::string::String,
    /// The result of the Interchain Query execution.
    #[prost(message, optional, tag = "4")]
    pub result: ::core::option::Option<QueryResult>,
}
/// Contains different information about a single Interchain Query execution result. Currently,
/// this structure is used both in query result submission via an ICQ Relayer and as a query result
/// storage for read/write operations to interchainqueries module, but the structure fields are
/// populated in a bit different ways. When submitting a query result, all fields are populated and
/// provided to the interchainqueries module in order to verify the result against the IBC client's
/// state. But in order to lighten the chain state, the interchainqueries module removes the block
/// field and proofs from the kv_results.
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
#[proto_message(type_url = "/neutron.interchainqueries.QueryResult")]
pub struct QueryResult {
    /// A list of a KV Interchain Query execution results. Each result contains query parameters, a
    /// response value and a proof.
    #[prost(message, repeated, tag = "1")]
    pub kv_results: ::prost::alloc::vec::Vec<StorageValue>,
    /// A TX Interchain Query execution result. Contains metainformation about the blocks of the query
    /// execution height. Only populated when submitting an Interchain Query result for verification
    /// and emptied when saving the result on chain.
    #[prost(message, optional, tag = "2")]
    pub block: ::core::option::Option<Block>,
    /// The height of the chain at the moment of the Interchain Query execution.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
    /// The revision number of the chain at the moment of the Interchain Query execution.
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub revision: u64,
    /// Whether to send the query result to the owner contract as a sudo message. Only applicable for
    /// KV type of Interchain Queries.
    #[prost(bool, tag = "5")]
    pub allow_kv_callbacks: bool,
}
/// A verifiable result of performing a single KVKey read.
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
#[proto_message(type_url = "/neutron.interchainqueries.StorageValue")]
pub struct StorageValue {
    /// The first half of the storage path. It is supposed to be a substore name for the query
    /// (e.g. bank, staking, etc.).
    #[prost(string, tag = "1")]
    pub storage_prefix: ::prost::alloc::string::String,
    /// The second half of the storage path. The remaining part of the full path to an IAVL storage node.
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// A base64-encoded value read from the given storage path.
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// The Merkle Proof which proves existence of key-value pair in IAVL storage. Is used to verify
    /// the pair against the respective remote chain's header.
    #[prost(message, optional, tag = "4")]
    pub proof: ::core::option::Option<super::super::tendermint::crypto::ProofOps>,
}
/// A single verifiable result of an Interchain Query of TX type.
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
#[proto_message(type_url = "/neutron.interchainqueries.Block")]
pub struct Block {
    /// The header of the block next to the block the transaction is included in. It is needed to know
    /// block X+1 header to verify response of transaction for block X since LastResultsHash is root
    /// hash of all results of the txs from the previous block.
    #[prost(message, optional, tag = "1")]
    pub next_block_header: ::core::option::Option<crate::shim::Any>,
    /// The header of the block the transaction is included in. It is needed to know block header to
    /// verify inclusion of the transaction.
    #[prost(message, optional, tag = "2")]
    pub header: ::core::option::Option<crate::shim::Any>,
    /// The transaction matched by the Interchain Query's transaction filter.
    #[prost(message, optional, tag = "3")]
    pub tx: ::core::option::Option<TxValue>,
}
/// Contains transaction body, response, and proofs of inclusion and delivery.
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
#[proto_message(type_url = "/neutron.interchainqueries.TxValue")]
pub struct TxValue {
    /// The result of the transaction execution.
    #[prost(message, optional, tag = "1")]
    pub response: ::core::option::Option<super::super::tendermint::abci::ExecTxResult>,
    /// The Merkle Proof which proves existence of response in the block next to the block the
    /// transaction is included in.
    #[prost(message, optional, tag = "2")]
    pub delivery_proof: ::core::option::Option<super::super::tendermint::crypto::Proof>,
    /// The Merkle Proof which proves inclusion of the transaction in the block.
    #[prost(message, optional, tag = "3")]
    pub inclusion_proof: ::core::option::Option<super::super::tendermint::crypto::Proof>,
    /// The arbitrary data typed body of the transaction.
    #[prost(bytes = "vec", tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Response type for the Msg/SubmitQueryResult RPC method.
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
#[proto_message(type_url = "/neutron.interchainqueries.MsgSubmitQueryResultResponse")]
pub struct MsgSubmitQueryResultResponse {}
/// Request type for the Msg/RemoveInterchainQuery RPC method.
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
#[proto_message(type_url = "/neutron.interchainqueries.MsgRemoveInterchainQueryRequest")]
pub struct MsgRemoveInterchainQueryRequest {
    /// The ID of the query to remove.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "queryID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub query_id: u64,
    /// The signer of the message.
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
}
/// Response type for the Msg/RemoveInterchainQuery RPC method.
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
#[proto_message(type_url = "/neutron.interchainqueries.MsgRemoveInterchainQueryResponse")]
pub struct MsgRemoveInterchainQueryResponse {}
/// Request type for the Msg/UpdateInterchainQuery RPC method.
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
#[proto_message(type_url = "/neutron.interchainqueries.MsgUpdateInterchainQueryRequest")]
pub struct MsgUpdateInterchainQueryRequest {
    /// The ID of the query to update.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "queryID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub query_id: u64,
    /// A new list of KV-storage keys for which to get values from the remote chain. Only applicable
    /// for a KV Interchain Query. Max amount of keys is limited by the module's `max_kv_query_keys_count`
    /// parameters.
    #[prost(message, repeated, tag = "2")]
    pub new_keys: ::prost::alloc::vec::Vec<KvKey>,
    /// A new minimal delay between consecutive query executions.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub new_update_period: u64,
    /// A new list of filters for remote transactions search. Only applicable for a TX Interchain
    /// Query. Example: "\[{\"field\":\"tx.height\",\"op\":\"Gte\",\"value\":2644737}\]".
    /// Supported operators: "eq", "lt", "gt", "lte", "gte". Max amount of filter conditions is
    /// limited by the module's `max_transactions_filters` parameters.
    #[prost(string, tag = "4")]
    pub new_transactions_filter: ::prost::alloc::string::String,
    /// The signer of the message.
    #[prost(string, tag = "5")]
    pub sender: ::prost::alloc::string::String,
}
/// Response type for the Msg/UpdateInterchainQuery RPC method.
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
#[proto_message(type_url = "/neutron.interchainqueries.MsgUpdateInterchainQueryResponse")]
pub struct MsgUpdateInterchainQueryResponse {}
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
#[proto_message(type_url = "/neutron.interchainqueries.MsgUpdateParams")]
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
#[proto_message(type_url = "/neutron.interchainqueries.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
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
#[proto_message(type_url = "/neutron.interchainqueries.QueryParamsRequest")]
#[proto_query(
    path = "/neutron.interchainqueries.Query/Params",
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
#[proto_message(type_url = "/neutron.interchainqueries.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// Stores all parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// Request type for the Query/RegisteredQueries RPC method.
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
#[proto_message(type_url = "/neutron.interchainqueries.QueryRegisteredQueriesRequest")]
#[proto_query(
    path = "/neutron.interchainqueries.Query/RegisteredQueries",
    response_type = QueryRegisteredQueriesResponse
)]
pub struct QueryRegisteredQueriesRequest {
    /// A list of owners of Interchain Queries. Query response will contain only Interchain Queries
    /// that are owned by one of the owners in the list. If none, Interchain Queries are not filtered
    /// out by the owner field.
    #[prost(string, repeated, tag = "1")]
    pub owners: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// IBC connection ID. Query response will contain only Interchain Queries that have the same IBC
    /// connection ID parameter. If none, Interchain Queries are not filtered out by the connection ID
    /// field.
    #[prost(string, tag = "2")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    /// Pagination parameters for the request. Use values from previous response in the next request
    /// in consecutive requests with paginated responses.
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// Response type for the Query/RegisteredQueries RPC method.
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
#[proto_message(type_url = "/neutron.interchainqueries.QueryRegisteredQueriesResponse")]
pub struct QueryRegisteredQueriesResponse {
    /// A list of registered Interchain Queries.
    #[prost(message, repeated, tag = "1")]
    pub registered_queries: ::prost::alloc::vec::Vec<RegisteredQuery>,
    /// Current page information. Use values from previous response in the next request in consecutive
    /// requests with paginated responses.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// Request type for the Query/RegisteredQuery RPC method.
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
#[proto_message(type_url = "/neutron.interchainqueries.QueryRegisteredQueryRequest")]
#[proto_query(
    path = "/neutron.interchainqueries.Query/RegisteredQuery",
    response_type = QueryRegisteredQueryResponse
)]
pub struct QueryRegisteredQueryRequest {
    /// ID of an Interchain Query.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "queryID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub query_id: u64,
}
/// Response type for the Query/RegisteredQuery RPC method.
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
#[proto_message(type_url = "/neutron.interchainqueries.QueryRegisteredQueryResponse")]
pub struct QueryRegisteredQueryResponse {
    /// A registered Interchain Query.
    #[prost(message, optional, tag = "1")]
    pub registered_query: ::core::option::Option<RegisteredQuery>,
}
/// Request type for the Query/QueryResult RPC method.
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
#[proto_message(type_url = "/neutron.interchainqueries.QueryRegisteredQueryResultRequest")]
#[proto_query(
    path = "/neutron.interchainqueries.Query/QueryResult",
    response_type = QueryRegisteredQueryResultResponse
)]
pub struct QueryRegisteredQueryResultRequest {
    /// ID of an Interchain Query.
    #[prost(uint64, tag = "1")]
    #[serde(alias = "queryID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub query_id: u64,
}
/// Response type for the Query/QueryResult RPC method.
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
#[proto_message(type_url = "/neutron.interchainqueries.QueryRegisteredQueryResultResponse")]
pub struct QueryRegisteredQueryResultResponse {
    /// The last successfully submitted result of an Interchain Query.
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<QueryResult>,
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
#[proto_message(type_url = "/neutron.interchainqueries.Transaction")]
pub struct Transaction {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64_encoded_string::serialize",
        deserialize_with = "crate::serde::as_base64_encoded_string::deserialize"
    )]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Request type for the Query/LastRemoteHeight RPC method.
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
#[proto_message(type_url = "/neutron.interchainqueries.QueryLastRemoteHeight")]
#[proto_query(
    path = "/neutron.interchainqueries.Query/LastRemoteHeight",
    response_type = QueryLastRemoteHeightResponse
)]
pub struct QueryLastRemoteHeight {
    /// Connection ID of an IBC connection to a remote chain. Determines the IBC client used in query
    /// handling.
    #[prost(string, tag = "1")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
}
/// Response type for the Query/LastRemoteHeight RPC method.
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
#[proto_message(type_url = "/neutron.interchainqueries.QueryLastRemoteHeightResponse")]
pub struct QueryLastRemoteHeightResponse {
    /// The height of the chain that the IBC client is currently on.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: u64,
    /// The revision of the chain that the IBC client is currently on.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub revision: u64,
}
pub struct InterchainqueriesQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> InterchainqueriesQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn registered_queries(
        &self,
        owners: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        connection_id: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryRegisteredQueriesResponse, cosmwasm_std::StdError> {
        QueryRegisteredQueriesRequest {
            owners,
            connection_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn registered_query(
        &self,
        query_id: u64,
    ) -> Result<QueryRegisteredQueryResponse, cosmwasm_std::StdError> {
        QueryRegisteredQueryRequest { query_id }.query(self.querier)
    }
    pub fn query_result(
        &self,
        query_id: u64,
    ) -> Result<QueryRegisteredQueryResultResponse, cosmwasm_std::StdError> {
        QueryRegisteredQueryResultRequest { query_id }.query(self.querier)
    }
    pub fn last_remote_height(
        &self,
        connection_id: ::prost::alloc::string::String,
    ) -> Result<QueryLastRemoteHeightResponse, cosmwasm_std::StdError> {
        QueryLastRemoteHeight { connection_id }.query(self.querier)
    }
}
