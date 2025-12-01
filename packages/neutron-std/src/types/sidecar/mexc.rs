use neutron_std_derive::CosmwasmExt;
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
#[proto_message(type_url = "/sidecar.mexc.PublicMiniTickerV3Api")]
pub struct PublicMiniTickerV3Api {
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub rate: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub zoned_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub high: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub low: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub volume: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub last_close_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub last_close_zoned_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub last_close_high: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub last_close_low: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/sidecar.mexc.PushDataV3ApiWrapper")]
pub struct PushDataV3ApiWrapper {
    #[prost(string, tag = "1")]
    pub channel: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    #[serde(alias = "symbolID")]
    pub symbol_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::option_as_str::serialize",
        deserialize_with = "crate::serde::option_as_str::deserialize"
    )]
    pub create_time: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::option_as_str::serialize",
        deserialize_with = "crate::serde::option_as_str::deserialize"
    )]
    pub send_time: ::core::option::Option<i64>,
    #[prost(oneof = "push_data_v3_api_wrapper::Body", tags = "309")]
    pub body: ::core::option::Option<push_data_v3_api_wrapper::Body>,
}
/// Nested message and enum types in `PushDataV3ApiWrapper`.
pub mod push_data_v3_api_wrapper {
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
    pub enum Body {
        #[prost(message, tag = "309")]
        PublicMiniTicker(super::PublicMiniTickerV3Api),
    }
}
