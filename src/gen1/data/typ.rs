
use ::serde::Deserialize;
use ::serde::Serialize;
use crate::util::Identifier;

pub type TypId = u64;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Typ {
    Empty(EmptyType),
    Bool(BoolType),
    Int(IntType),
    Real(RealType),
    Bytes(BytesType),
    Text(TextType),
    HomogeneousCollection(HomogeneousCollectionType),
    HeterogeneousCollection(HeterogeneousCollectionType),
    ObjectRef(TypId), //TODO @mark: or should this be split into record and union?
                      //TODO @mark: generic type uses
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TypeDeclaration {
    //TODO @mark: split into declarations:
    Union(UnionType),
    Record(RecordType),
    //TODO @mark: generic type declarations
}

//TODO @mark: are maps needed? how to deal with static vs dynamic type of keys and values? 4 combis?

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CollectionOrdering {
    Arbitrary,
    Ordered,
    Sorted,
}

impl Default for CollectionOrdering {
    fn default() -> Self {
        CollectionOrdering::Arbitrary
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Unicity {
    NonUnique,
    Unique,
    /// Only applicable if the element type is a Tuple, in which case the collection is
    /// interpreted as a dict and the first element is the key (must be hashable).
    FirstUnique,
}

impl Default for Unicity {
    fn default() -> Self {
        Unicity::NonUnique
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Length {
    #[serde(default, skip_serializing_if = "is_default")]
    min: u64,
    #[serde(default, skip_serializing_if = "is_default")]
    max: Option<u64>,
}

impl Length {
    pub fn unknown() -> Self {
        Length { min: 0, max: None }
    }

    pub fn fixed(len: u64) -> Self {
        Length {
            min: len,
            max: Some(len),
        }
    }

    pub fn at_least(min_len: u64) -> Self {
        Length {
            min: min_len,
            max: None,
        }
    }

    pub fn at_most(max_len: u64) -> Self {
        Length {
            min: 0,
            max: Some(max_len),
        }
    }

    pub fn between(min_len: u64, max_len: u64) -> Self {
        Length {
            min: min_len,
            max: Some(max_len),
        }
    }
}

impl Default for Length {
    fn default() -> Self {
        Length::unknown()
    }
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
#[serde(deny_unknown_fields)]
pub struct NamedType {
    name: Identifier,
    typ: Typ,
}

impl NamedType {
    pub fn new(name: Identifier, typ: Typ) -> Self {
        NamedType { name, typ }
    }
}

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
#[serde(deny_unknown_fields)]
pub struct BytesType {
    #[serde(default, skip_serializing_if = "is_default")]
    length: Length,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TextType {
    #[serde(default, skip_serializing_if = "is_default")]
    length: Length,
}

impl TextType {
    pub fn new(length: Length) -> Self {
        TextType { length }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UnionType {
    id: TypId,
    name: Option<Identifier>,
    options: Vec<NamedType>,
}

impl UnionType {
    pub fn new(name: Option<Identifier>, options: impl Into<Vec<NamedType>>) -> Self {
        UnionType {
            id: 0, //TODO @mark:
            name,
            options: options.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RecordType {
    id: TypId,
    name: Option<Identifier>,
    values: Vec<NamedType>,
}

impl RecordType {
    pub fn new(name: Option<Identifier>, values: impl Into<Vec<NamedType>>) -> Self {
        RecordType {
            id: 0, //TODO @mark:
            name,
            values: values.into(),
        }
    }

    pub fn named(name: Identifier, values: impl Into<Vec<NamedType>>) -> Self {
        RecordType::new(Some(name), values)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct HomogeneousCollectionType {
    element_type: Box<Typ>,
    #[serde(default, skip_serializing_if = "is_default")]
    ordering: CollectionOrdering,
    #[serde(default, skip_serializing_if = "is_default")]
    unique: Unicity,
    #[serde(default, skip_serializing_if = "is_default")]
    length: Length,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct HeterogeneousCollectionType {
    #[serde(default, skip_serializing_if = "is_default")]
    ordering: CollectionOrdering,
    #[serde(default, skip_serializing_if = "is_default")]
    length: Length,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_homogeneous() {
        let inp = Typ::HomogeneousCollection(HomogeneousCollectionType {
            element_type: Box::new(Typ::Text(TextType {
                length: Length::unknown(),
            })),
            ordering: CollectionOrdering::Arbitrary,
            unique: Unicity::NonUnique,
            length: Length::at_least(10),
        });
        let json = serde_json::to_string(&inp).unwrap();
        assert_eq!(
            json,
            r#"{"type":"homogeneous_collection","element_type":{"type":"text"},"length":{"min":10}}"#
        )
    }

    #[test]
    fn serialize_heterogeneous() {
        let inp = Typ::HeterogeneousCollection(HeterogeneousCollectionType {
            ordering: CollectionOrdering::Sorted,
            length: Length::between(1, 100),
        });
        let json = serde_json::to_string(&inp).unwrap();
        assert_eq!(
            json,
            r#"{"type":"heterogeneous_collection","ordering":{"type":"sorted"},"length":{"min":1,"max":100}}"#
        )
    }
}