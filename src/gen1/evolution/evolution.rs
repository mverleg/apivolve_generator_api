use ::serde::Deserialize;
use ::serde::Serialize;
use ::smallvec::SmallVec;

use crate::gen1::evolution::typ::RecordType;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Evolution {
    //TODO @mark: postfix
    pub records: SmallVec<[RecordType; 2]>,
}

