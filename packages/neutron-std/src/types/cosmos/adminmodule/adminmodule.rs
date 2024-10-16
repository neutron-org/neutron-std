use neutron_std_derive::CosmwasmExt;
/// GenesisState defines the adminmodule module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.adminmodule.adminmodule.GenesisState")]
pub struct GenesisState {
    #[prost(string, repeated, tag = "1")]
    pub admins: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[proto_message(type_url = "/cosmos.adminmodule.adminmodule.QueryAdminsRequest")]
#[proto_query(
    path = "/cosmos.adminmodule.adminmodule.Query/Admins",
    response_type = QueryAdminsResponse
)]
pub struct QueryAdminsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.adminmodule.adminmodule.QueryAdminsResponse")]
pub struct QueryAdminsResponse {
    #[prost(string, repeated, tag = "1")]
    pub admins: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[proto_message(type_url = "/cosmos.adminmodule.adminmodule.QueryArchivedProposalsRequest")]
#[proto_query(
    path = "/cosmos.adminmodule.adminmodule.Query/ArchivedProposals",
    response_type = QueryArchivedProposalsResponse
)]
pub struct QueryArchivedProposalsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.adminmodule.adminmodule.QueryArchivedProposalsLegacyRequest")]
#[proto_query(
    path = "/cosmos.adminmodule.adminmodule.Query/ArchivedProposalsLegacy",
    response_type = QueryArchivedProposalsLegacyResponse
)]
pub struct QueryArchivedProposalsLegacyRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.adminmodule.adminmodule.QueryProposalsResponse")]
pub struct QueryProposalsResponse {
    #[prost(message, repeated, tag = "1")]
    pub proposals: ::prost::alloc::vec::Vec<super::super::gov::v1::Proposal>,
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
#[proto_message(type_url = "/cosmos.adminmodule.adminmodule.QueryArchivedProposalsResponse")]
pub struct QueryArchivedProposalsResponse {
    #[prost(message, repeated, tag = "1")]
    pub proposals: ::prost::alloc::vec::Vec<super::super::gov::v1::Proposal>,
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
#[proto_message(type_url = "/cosmos.adminmodule.adminmodule.QueryArchivedProposalsLegacyResponse")]
pub struct QueryArchivedProposalsLegacyResponse {
    #[prost(message, repeated, tag = "1")]
    pub proposals_legacy: ::prost::alloc::vec::Vec<super::super::gov::v1beta1::Proposal>,
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
#[proto_message(type_url = "/cosmos.adminmodule.adminmodule.MsgDeleteAdmin")]
pub struct MsgDeleteAdmin {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/cosmos.adminmodule.adminmodule.MsgDeleteAdminResponse")]
pub struct MsgDeleteAdminResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.adminmodule.adminmodule.MsgAddAdmin")]
pub struct MsgAddAdmin {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/cosmos.adminmodule.adminmodule.MsgAddAdminResponse")]
pub struct MsgAddAdminResponse {}
/// MsgSubmitProposalLegacy defines an sdk.Msg type that supports submitting arbitrary
/// proposal Content.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.adminmodule.adminmodule.MsgSubmitProposalLegacy")]
pub struct MsgSubmitProposalLegacy {
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<crate::shim::Any>,
    #[prost(string, tag = "2")]
    pub proposer: ::prost::alloc::string::String,
}
/// MsgSubmitProposalLegacyResponse defines the Msg/SubmitProposalLegacy response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.adminmodule.adminmodule.MsgSubmitProposalLegacyResponse")]
pub struct MsgSubmitProposalLegacyResponse {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
}
/// MsgSubmitProposal defines an sdk.Msg type that supports submitting arbitrary
/// proposal Content.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.adminmodule.adminmodule.MsgSubmitProposal")]
pub struct MsgSubmitProposal {
    /// messages are the arbitrary messages to be executed if proposal passes.
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<crate::shim::Any>,
    #[prost(string, tag = "2")]
    pub proposer: ::prost::alloc::string::String,
}
/// MsgSubmitProposalResponse defines the Msg/SubmitProposal response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.adminmodule.adminmodule.MsgSubmitProposalResponse")]
pub struct MsgSubmitProposalResponse {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "proposalID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
}
pub struct AdminmoduleQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> AdminmoduleQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn admins(&self) -> Result<QueryAdminsResponse, cosmwasm_std::StdError> {
        QueryAdminsRequest {}.query(self.querier)
    }
    pub fn archived_proposals(
        &self,
    ) -> Result<QueryArchivedProposalsResponse, cosmwasm_std::StdError> {
        QueryArchivedProposalsRequest {}.query(self.querier)
    }
    pub fn archived_proposals_legacy(
        &self,
    ) -> Result<QueryArchivedProposalsLegacyResponse, cosmwasm_std::StdError> {
        QueryArchivedProposalsLegacyRequest {}.query(self.querier)
    }
}
