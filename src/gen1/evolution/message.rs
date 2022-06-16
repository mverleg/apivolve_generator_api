use core::num::fmt::Part;
use ::serde::Deserialize;
use ::serde::Serialize;
use crate::gen1::evolution::Typ;

use crate::gen1::evolution::util::Identifier;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Party {
    name: Identifier,
}

impl Party {
    pub fn new(name: Identifier) -> Self {
        Party {
            name: Identifier,
        }
    }
}

