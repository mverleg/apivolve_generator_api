use ::serde::Deserialize;
use ::serde::Serialize;
use crate::gen1::evolution::util::Identifier;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Typ {
    Empty,
    Bool,
    Int,
    Real,
    Bytes {
        length: Option<u64>,
    },
    Text {
        length: Option<u64>,
    },
    HomogeneousCollection(CollectionTyp),
    HeterogeneousCollection(HeterogeneousCollectionTyp),
    Union {
        options: Vec<NamedType>,
    },
    Object {
        values: Vec<NamedType>,
    },
}

//TODO @mark: how about dyanmic maps without fixed keys?
//TODO @mark: how about tuples with heterogeneous fields?
//TODO @mark: are maps needed? how to deal with static vs dynamic type of keys and values? 4 combis?

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CollectionOrdering {
    Sorted,
    Ordered,
    Arbitrary,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Unicity {
    Unique,
    /// Only applicable if the element type is a Tuple, in which case the collection is
    /// interpreted as a dict and the first element is the key (must be hashable).
    FirstUnique,
    NonUnique,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct NamedType {
    name: Identifier,
    typ: Typ,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CollectionTyp {
    element_type: Box<Typ>,
    ordering: CollectionOrdering,
    unique: Unicity,
    length: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct HeterogeneousCollectionTyp {
    ordering: CollectionOrdering,
    length: Option<u64>,
}
