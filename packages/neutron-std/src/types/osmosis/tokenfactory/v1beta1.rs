use neutron_std_derive::CosmwasmExt;
/// DenomAuthorityMetadata specifies metadata for addresses that have specific
/// capabilities over a token factory denom. Right now there is only one Admin
/// permission, but is planned to be extended to the future.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.DenomAuthorityMetadata")]
pub struct DenomAuthorityMetadata {
    /// Can be empty for no admin, or a valid osmosis address
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
}
/// GenesisState defines the tokenfactory module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.GenesisState")]
pub struct GenesisState {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<super::Params>,
    #[prost(message, repeated, tag = "2")]
    pub factory_denoms: ::prost::alloc::vec::Vec<GenesisDenom>,
}
/// GenesisDenom defines a tokenfactory denom that is defined within genesis
/// state. The structure contains DenomAuthorityMetadata which defines the
/// denom's admin.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.GenesisDenom")]
pub struct GenesisDenom {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub authority_metadata: ::core::option::Option<DenomAuthorityMetadata>,
    #[prost(string, tag = "3")]
    #[prost(optional)]
    pub hook_contract_address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Params defines the parameters for the tokenfactory module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.Params")]
pub struct Params {
    /// DenomCreationFee defines the fee to be charged on the creation of a new
    /// denom. The fee is drawn from the MsgCreateDenom's sender account, and
    /// transferred to the community pool.
    #[prost(message, repeated, tag = "1")]
    pub denom_creation_fee:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// DenomCreationGasConsume defines the gas cost for creating a new denom.
    /// This is intended as a spam deterrence mechanism.
    ///
    /// See: <https://github.com/CosmWasm/token-factory/issues/11>
    #[prost(uint64, tag = "2")]
    #[prost(optional)]
    #[serde(
        serialize_with = "crate::serde::option_as_str::serialize",
        deserialize_with = "crate::serde::option_as_str::deserialize"
    )]
    pub denom_creation_gas_consume: ::core::option::Option<u64>,
    /// FeeCollectorAddress is the address where fees collected from denom creation
    /// are sent to
    #[prost(string, tag = "3")]
    pub fee_collector_address: ::prost::alloc::string::String,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/osmosis.tokenfactory.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<super::Params>,
}
/// QueryDenomAuthorityMetadataRequest defines the request structure for the
/// DenomAuthorityMetadata gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.QueryDenomAuthorityMetadataRequest")]
#[proto_query(
    path = "/osmosis.tokenfactory.v1beta1.Query/DenomAuthorityMetadata",
    response_type = QueryDenomAuthorityMetadataResponse
)]
pub struct QueryDenomAuthorityMetadataRequest {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub subdenom: ::prost::alloc::string::String,
}
/// QueryDenomAuthorityMetadataResponse defines the response structure for the
/// DenomAuthorityMetadata gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.QueryDenomAuthorityMetadataResponse")]
pub struct QueryDenomAuthorityMetadataResponse {
    #[prost(message, optional, tag = "1")]
    pub authority_metadata: ::core::option::Option<DenomAuthorityMetadata>,
}
/// QueryDenomsFromCreatorRequest defines the request structure for the
/// DenomsFromCreator gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.QueryDenomsFromCreatorRequest")]
#[proto_query(
    path = "/osmosis.tokenfactory.v1beta1.Query/DenomsFromCreator",
    response_type = QueryDenomsFromCreatorResponse
)]
pub struct QueryDenomsFromCreatorRequest {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
}
/// QueryDenomsFromCreatorResponse defines the response structure for the
/// DenomsFromCreator gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.QueryDenomsFromCreatorResponse")]
pub struct QueryDenomsFromCreatorResponse {
    #[prost(string, repeated, tag = "1")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryBeforeSendHookAddressRequest defines the request structure for the
/// BeforeSendHookAddress gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.QueryBeforeSendHookAddressRequest")]
#[proto_query(
    path = "/osmosis.tokenfactory.v1beta1.Query/BeforeSendHookAddress",
    response_type = QueryBeforeSendHookAddressResponse
)]
pub struct QueryBeforeSendHookAddressRequest {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub subdenom: ::prost::alloc::string::String,
}
/// QueryBeforeSendHookAddressResponse defines the response structure for the
/// BeforeSendHookAddress gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.QueryBeforeSendHookAddressResponse")]
pub struct QueryBeforeSendHookAddressResponse {
    #[prost(string, tag = "1")]
    pub contract_addr: ::prost::alloc::string::String,
}
/// QueryFullDenomRequest defines the request structure for the
/// FullDenom gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.QueryFullDenomRequest")]
#[proto_query(
    path = "/osmosis.tokenfactory.v1beta1.Query/FullDenom",
    response_type = QueryFullDenomResponse
)]
pub struct QueryFullDenomRequest {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub subdenom: ::prost::alloc::string::String,
}
/// QueryFullDenomResponse defines the response structure for the
/// FullDenom gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.QueryFullDenomResponse")]
pub struct QueryFullDenomResponse {
    #[prost(string, tag = "1")]
    pub full_denom: ::prost::alloc::string::String,
}
/// MsgCreateDenom defines the message structure for the CreateDenom gRPC service
/// method. It allows an account to create a new denom. It requires a sender
/// address and a sub denomination. The (sender_address, sub_denomination) tuple
/// must be unique and cannot be re-used.
///
/// The resulting denom created is defined as
/// <factory/{creatorAddress}/{subdenom}>. The resulting denom's admin is
/// originally set to be the creator, but this can be changed later. The token
/// denom does not indicate the current admin.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgCreateDenom")]
pub struct MsgCreateDenom {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// subdenom can be up to 44 "alphanumeric" characters long.
    #[prost(string, tag = "2")]
    pub subdenom: ::prost::alloc::string::String,
}
/// MsgCreateDenomResponse is the return value of MsgCreateDenom
/// It returns the full string of the newly created denom
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgCreateDenomResponse")]
pub struct MsgCreateDenomResponse {
    #[prost(string, tag = "1")]
    pub new_token_denom: ::prost::alloc::string::String,
}
/// MsgMint is the sdk.Msg type for allowing an admin account to mint
/// more of a token.  For now, we only support minting to the sender account
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgMint")]
pub struct MsgMint {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub mint_to_address: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgMintResponse")]
pub struct MsgMintResponse {}
/// MsgBurn is the sdk.Msg type for allowing an admin account to burn
/// a token.  For now, we only support burning from the sender account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgBurn")]
pub struct MsgBurn {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub burn_from_address: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgBurnResponse")]
pub struct MsgBurnResponse {}
/// MsgChangeAdmin is the sdk.Msg type for allowing an admin account to reassign
/// adminship of a denom to a new account
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgChangeAdmin")]
pub struct MsgChangeAdmin {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub new_admin: ::prost::alloc::string::String,
}
/// MsgChangeAdminResponse defines the response structure for an executed
/// MsgChangeAdmin message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgChangeAdminResponse")]
pub struct MsgChangeAdminResponse {}
/// MsgSetBeforeSendHook is the sdk.Msg type for allowing an admin account to
/// assign a CosmWasm contract to call with a BeforeSend hook
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgSetBeforeSendHook")]
pub struct MsgSetBeforeSendHook {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub contract_addr: ::prost::alloc::string::String,
}
/// MsgSetBeforeSendHookResponse defines the response structure for an executed
/// MsgSetBeforeSendHook message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgSetBeforeSendHookResponse")]
pub struct MsgSetBeforeSendHookResponse {}
/// MsgSetDenomMetadata is the sdk.Msg type for allowing an admin account to set
/// the denom's bank metadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgSetDenomMetadata")]
pub struct MsgSetDenomMetadata {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<super::super::super::cosmos::bank::v1beta1::Metadata>,
}
/// MsgSetDenomMetadataResponse defines the response structure for an executed
/// MsgSetDenomMetadata message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgSetDenomMetadataResponse")]
pub struct MsgSetDenomMetadataResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgForceTransfer")]
pub struct MsgForceTransfer {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub transfer_from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub transfer_to_address: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgForceTransferResponse")]
pub struct MsgForceTransferResponse {}
/// MsgUpdateParams is the MsgUpdateParams request type.
///
/// Since: 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// Authority is the address of the governance account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/tokenfactory parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<super::Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
///
/// Since: 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/osmosis.tokenfactory.v1beta1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct TokenfactoryQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> TokenfactoryQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn denom_authority_metadata(
        &self,
        creator: ::prost::alloc::string::String,
        subdenom: ::prost::alloc::string::String,
    ) -> Result<QueryDenomAuthorityMetadataResponse, cosmwasm_std::StdError> {
        QueryDenomAuthorityMetadataRequest { creator, subdenom }.query(self.querier)
    }
    pub fn denoms_from_creator(
        &self,
        creator: ::prost::alloc::string::String,
    ) -> Result<QueryDenomsFromCreatorResponse, cosmwasm_std::StdError> {
        QueryDenomsFromCreatorRequest { creator }.query(self.querier)
    }
    pub fn before_send_hook_address(
        &self,
        creator: ::prost::alloc::string::String,
        subdenom: ::prost::alloc::string::String,
    ) -> Result<QueryBeforeSendHookAddressResponse, cosmwasm_std::StdError> {
        QueryBeforeSendHookAddressRequest { creator, subdenom }.query(self.querier)
    }
    pub fn full_denom(
        &self,
        creator: ::prost::alloc::string::String,
        subdenom: ::prost::alloc::string::String,
    ) -> Result<QueryFullDenomResponse, cosmwasm_std::StdError> {
        QueryFullDenomRequest { creator, subdenom }.query(self.querier)
    }
}
