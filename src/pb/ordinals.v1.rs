// @generated
// RUNES

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
/// Represents a continuous block of ordinals assigned to a given UTXO
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrdinalBlock {
    #[prost(string, tag="1")]
    pub utxo: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, tag="3")]
    pub start: u64,
    #[prost(uint64, tag="4")]
    pub size: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(string, tag="1")]
    pub txid: ::prost::alloc::string::String,
    /// Output number
    #[prost(uint64, tag="2")]
    pub idx: u64,
    /// Amount transferred in sats
    #[prost(uint64, tag="3")]
    pub amount: u64,
    /// Fee in sats
    /// int64 fee = 4;
    /// Ordinals assignment (only present for coinbase transaction)
    #[prost(message, repeated, tag="4")]
    pub coinbase_ordinals: ::prost::alloc::vec::Vec<OrdinalBlock>,
    /// Input UTXOs
    #[prost(string, repeated, tag="5")]
    pub input_utxos: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Note: The ordinals blocks here are relative and refer to the
    /// ordinals assigned to the input UTXOs
    /// E.g.: The Nth to Mth ordinals of the input utxos should
    /// be assigned to some output utxo
    #[prost(message, repeated, tag="6")]
    pub relative_ordinals: ::prost::alloc::vec::Vec<OrdinalBlock>,
    #[prost(message, repeated, tag="7")]
    pub inscriptions: ::prost::alloc::vec::Vec<Inscription>,
    /// Optional metadata
    #[prost(message, optional, tag="8")]
    pub rune: ::core::option::Option<RunestoneBuf>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    /// Block timestamp
    #[prost(uint64, tag="1")]
    pub timestamp: u64,
    /// Block number
    #[prost(uint64, tag="2")]
    pub number: u64,
    /// Total miner reward (in sats)
    #[prost(uint64, tag="3")]
    pub miner_reward: u64,
    /// Block subsidy (in sats)
    #[prost(uint64, tag="4")]
    pub subsidy: u64,
    /// Miner fees (in sats)
    #[prost(uint64, tag="5")]
    pub fees: u64,
    /// Block transactions
    #[prost(message, repeated, tag="6")]
    pub txs: ::prost::alloc::vec::Vec<Transaction>,
    #[prost(string, tag="7")]
    pub miner_address: ::prost::alloc::string::String,
    #[prost(uint64, tag="8")]
    pub total_runes_tx: u64,
    #[prost(message, repeated, tag="9")]
    pub runestones: ::prost::alloc::vec::Vec<RunestoneBuf>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Inscription {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Optional MIME type of the inscription
    #[prost(string, optional, tag="2")]
    pub content_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional pointer if the inscription is not for the
    /// first ordinal of its inputs
    #[prost(int64, optional, tag="3")]
    pub pointer: ::core::option::Option<i64>,
    #[prost(string, optional, tag="4")]
    pub parent: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub metadata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="6")]
    pub metaprotocol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub content_encoding: ::core::option::Option<::prost::alloc::string::String>,
    /// Content of the inscription
    #[prost(string, tag="8")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub content_length: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Inscriptions {
    #[prost(message, repeated, tag="1")]
    pub inscriptions: ::prost::alloc::vec::Vec<Inscription>,
}
// @@protoc_insertion_point(module)
