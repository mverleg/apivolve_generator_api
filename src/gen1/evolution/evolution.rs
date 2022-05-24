use ::serde::Deserialize;
use ::serde::Serialize;
use ::smallvec::SmallVec;

use crate::gen1::evolution::typ::ObjectType;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Evolution {
    //TODO @mark: postfix
    pub objects: SmallVec<[ObjectType; 2]>,
}

