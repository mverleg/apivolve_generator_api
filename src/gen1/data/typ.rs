
use ::serde::Deserialize;
use ::serde::Serialize;
use crate::gen1::data::{BoolType, BytesType, EmptyType, HeterogeneousCollectionType, HomogeneousCollectionType, IntType, RealType, RecordType, TextType, UnionType};

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
