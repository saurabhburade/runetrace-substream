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
pub struct RunestoneBuf {
    #[prost(message, repeated, tag="1")]
    pub edicts: ::prost::alloc::vec::Vec<Edict>,
    #[prost(message, optional, tag="2")]
    pub etching: ::core::option::Option<Etching>,
    #[prost(string, tag="3")]
    pub mint: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub pointer: ::prost::alloc::string::String,
}
/// The following fields are marked as optional in Rust
/// They are represented as a oneof in protobuf
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Etching {
    #[prost(int32, optional, tag="1")]
    pub divisibility: ::core::option::Option<i32>,
    #[prost(string, optional, tag="2")]
    pub premine: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub rune: ::core::option::Option<RuneBuf>,
    #[prost(string, optional, tag="4")]
    pub spacers: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="6")]
    pub terms: ::core::option::Option<Terms>,
    #[prost(bool, tag="7")]
    pub turbo: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Terms {
    /// Define the fields of the Terms struct here
    /// Example:
    #[prost(string, tag="1")]
    pub cap: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuneId {
    #[prost(string, tag="1")]
    pub block: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tx: ::prost::alloc::string::String,
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
pub struct Cenotaph {
    #[prost(message, optional, tag="1")]
    pub etching: ::core::option::Option<Rune>,
    #[prost(enumeration="Flaw", tag="2")]
    pub flaw: i32,
    #[prost(message, optional, tag="3")]
    pub mint: ::core::option::Option<RuneId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuneBuf {
    #[prost(uint64, tag="1")]
    pub value: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Flaw {
    EdictOutput = 0,
    EdictRuneId = 1,
    InvalidScript = 2,
    Opcode = 3,
    SupplyOverflow = 4,
    TrailingIntegers = 5,
    TruncatedField = 6,
    UnrecognizedEvenTag = 7,
    UnrecognizedFlag = 8,
    Varint = 9,
}
impl Flaw {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Flaw::EdictOutput => "EDICT_OUTPUT",
            Flaw::EdictRuneId => "EDICT_RUNE_ID",
            Flaw::InvalidScript => "INVALID_SCRIPT",
            Flaw::Opcode => "OPCODE",
            Flaw::SupplyOverflow => "SUPPLY_OVERFLOW",
            Flaw::TrailingIntegers => "TRAILING_INTEGERS",
            Flaw::TruncatedField => "TRUNCATED_FIELD",
            Flaw::UnrecognizedEvenTag => "UNRECOGNIZED_EVEN_TAG",
            Flaw::UnrecognizedFlag => "UNRECOGNIZED_FLAG",
            Flaw::Varint => "VARINT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EDICT_OUTPUT" => Some(Self::EdictOutput),
            "EDICT_RUNE_ID" => Some(Self::EdictRuneId),
            "INVALID_SCRIPT" => Some(Self::InvalidScript),
            "OPCODE" => Some(Self::Opcode),
            "SUPPLY_OVERFLOW" => Some(Self::SupplyOverflow),
            "TRAILING_INTEGERS" => Some(Self::TrailingIntegers),
            "TRUNCATED_FIELD" => Some(Self::TruncatedField),
            "UNRECOGNIZED_EVEN_TAG" => Some(Self::UnrecognizedEvenTag),
            "UNRECOGNIZED_FLAG" => Some(Self::UnrecognizedFlag),
            "VARINT" => Some(Self::Varint),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
