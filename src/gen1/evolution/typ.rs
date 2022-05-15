use ::serde::Deserialize;
use ::serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Typ {
    Null,
    Bool,
    Int,
    Real,
    Text,
    Collection {
        element_type: Typ,
        ordering: ArrayOrdering,
        unique: bool,
    },
    Union {
        options: Vec<Typ>,
    },
    Tuple {
        values: Vec<Typ>,
    },
    Object {
        values: Vec<Typ>,
    },
}

//TODO @mark: how about dyanmic maps without fixed keys?
//TODO @mark: how about tuples with heterogeneous fields?

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Collection {
    Sorted,
    Ordered,
    Arbitrary,
}
