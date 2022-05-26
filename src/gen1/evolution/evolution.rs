use ::serde::Deserialize;
use ::serde::Serialize;
use ::smallvec::SmallVec;

use crate::gen1::evolution::typ::RecordType;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Evolution {
    //TODO @mark: postfix
    pub messages: SmallVec<[Message; 2]>,
}

impl Evolution {
    pub fn new(messages: impl Into<SmallVec<[Message; 2]>>) -> Self {
        Evolution {
            messages: messages.into()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Message {
    //TODO @mark: differentiate whether it's bidirectional (default) or always sent from older to newer, or from newer to older (this may happen in a client-server situation where servers upgrade before clients)
    data: RecordType,
}

impl Message {
    pub fn new(data: RecordType) -> Self {
        Message {
            data
        }
    }
}
