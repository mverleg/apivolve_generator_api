use crate::gen1::data::{
    BoolType, BytesType, EmptyType, HeterogeneousCollectionType, HomogeneousCollectionType,
    IntType, RealType, RecordType, TextType, UnionType,
};
use ::serde::Deserialize;
use ::serde::Serialize;

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
    HomogeneousCollection(Box<HomogeneousCollectionType>),
    HeterogeneousCollection(HeterogeneousCollectionType),
    Ref(DeclarationRef),
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DeclarationRef {
    id: TypId,
}
