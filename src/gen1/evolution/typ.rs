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
        #[serde(default, skip_serializing_if = "is_default")]
        length: Length,
    },
    Text {
        #[serde(default, skip_serializing_if = "is_default")]
        length: Length,
    },
    HomogeneousCollection(HomogeneousCollectionTyp),
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
        Length { min: len, max: Some(len) }
    }

    pub fn at_least(min_len: u64) -> Self {
        Length { min: min_len, max: None }
    }

    pub fn at_most(max_len: u64) -> Self {
        Length { min: 0, max: Some(max_len) }
    }

    pub fn between(min_len: u64, max_len: u64) -> Self {
        Length { min: min_len, max: Some(max_len) }
    }
}

impl Default for Length {
    fn default() -> Self {
        Length::unknown()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct NamedType {
    name: Identifier,
    typ: Typ,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct HomogeneousCollectionTyp {
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
pub struct HeterogeneousCollectionTyp {
    #[serde(default, skip_serializing_if = "is_default")]
    ordering: CollectionOrdering,
    #[serde(default, skip_serializing_if = "is_default")]
    length: Length,
}
