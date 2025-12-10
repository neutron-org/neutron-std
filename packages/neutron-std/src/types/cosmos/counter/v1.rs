use neutron_std_derive::CosmwasmExt;
/// QueryGetCountRequest defines the request type for querying x/mock count.
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
#[proto_message(type_url = "/cosmos.counter.v1.QueryGetCountRequest")]
#[proto_query(
    path = "/cosmos.counter.v1.Query/GetCount",
    response_type = QueryGetCountResponse
)]
pub struct QueryGetCountRequest {}
/// QueryGetCountResponse defines the response type for querying x/mock count.
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
#[proto_message(type_url = "/cosmos.counter.v1.QueryGetCountResponse")]
pub struct QueryGetCountResponse {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub total_count: i64,
}
/// MsgIncreaseCounter defines a count Msg service counter.
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
#[proto_message(type_url = "/cosmos.counter.v1.MsgIncreaseCounter")]
pub struct MsgIncreaseCounter {
    /// signer is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// count is the number of times to increment the counter.
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub count: i64,
}
/// MsgIncreaseCountResponse is the Msg/Counter response type.
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
#[proto_message(type_url = "/cosmos.counter.v1.MsgIncreaseCountResponse")]
pub struct MsgIncreaseCountResponse {
    /// new_count is the number of times the counter was incremented.
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub new_count: i64,
}
pub struct CounterQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> CounterQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn get_count(&self) -> Result<QueryGetCountResponse, cosmwasm_std::StdError> {
        QueryGetCountRequest {}.query(self.querier)
    }
}
