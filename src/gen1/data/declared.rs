use ::serde::Deserialize;
use ::serde::Serialize;

use crate::gen1::data::{Typ, TypId};
use crate::util::Identifier;

//TODO @mark: are maps needed? how to deal with static vs dynamic type of keys and values? 4 combis?

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
pub struct NamedType {
    name: Identifier,
    typ: Typ,
}

impl NamedType {
    pub fn new(name: Identifier, typ: Typ) -> Self {
        NamedType { name, typ }
    }
}
