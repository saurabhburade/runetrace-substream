// @generated
/// Represents a continuous block of ordinals assigned to a given UTXO
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rune {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub symbol: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunestoneBufs {
    #[prost(message, repeated, tag="1")]
    pub runestone_bufs: ::prost::alloc::vec::Vec<RunestoneBuf>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunestoneBuf {
    #[prost(message, repeated, tag="1")]
    pub edicts: ::prost::alloc::vec::Vec<Edict>,
    #[prost(message, optional, tag="2")]
    pub etching: ::core::option::Option<Etching>,
    #[prost(message, optional, tag="3")]
    pub mint: ::core::option::Option<RuneId>,
    #[prost(string, tag="4")]
    pub pointer: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Etching {
    #[prost(int32, optional, tag="1")]
    pub divisibility: ::core::option::Option<i32>,
    #[prost(string, optional, tag="2")]
    pub premine: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub rune: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub spacers: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="6")]
    pub turbo: ::core::option::Option<bool>,
    #[prost(string, optional, tag="7")]
    pub supply: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="8")]
    pub terms: ::core::option::Option<Terms>,
    #[prost(message, optional, tag="9")]
    pub id: ::core::option::Option<RuneId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Terms {
    #[prost(string, optional, tag="1")]
    pub cap: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub amount: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Edict {
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<RuneId>,
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub output: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuneId {
    #[prost(string, tag="1")]
    pub block: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tx: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
