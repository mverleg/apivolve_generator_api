
use ::serde::Deserialize;
use ::serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EmptyType {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BoolType {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct IntType {
    width: IntWidth,
    signed: Signed,
}

impl IntType {
    pub fn new(width: IntWidth, signed: Signed) -> Self {
        IntType { width, signed }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RealType {
    format: RealFormat,
}

impl RealType {
    pub fn new(format: RealFormat) -> Self {
        RealType { format }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Signed {
    Signed,
    Unsigned
}

impl Signed {
    pub fn is_signed(&self) -> bool {
        matches!(self, Signed::Signed)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RealFormat {
    IEEE754_32bit,
    IEEE754_64bit,
    //TODO @mark: maybe decimal with fixed number of decimals
    // Decimal { decimals: u32 },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum IntWidth {
    Byte,
    Bit16,
    Bit32,
    Bit64,
    Unlimited,
}
