use neutron_std_derive::CosmwasmExt;
/// Module is the config object of the builder module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/feemarket.feemarket.module.v1.Module")]
pub struct Module {
    /// Authority defines the custom module authority. If not set, defaults to the
    /// governance module.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// FeeRecipientModule defines the custom module account that the fee will be sent to.
    #[prost(string, tag = "2")]
    pub fee_recipient_module: ::prost::alloc::string::String,
}
