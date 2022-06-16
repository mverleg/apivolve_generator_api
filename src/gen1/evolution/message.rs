use ::serde::Deserialize;
use ::serde::Serialize;
use smallvec::SmallVec;
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
            name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
//TODO @mark: name
pub struct ResponseChoice {
    choices: SmallVec<[Message; 1]>,
    //TODO @mark: make sure min <= max
    min_count: u32,
    max_count: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Message {
    name: Identifier,
    sender: Party,
    //TODO @mark: e.g. specific client, set of clients or all clients?
    recipient: Party,
    content: Typ,
    /// One of these messages can be the response; empty means no response,
    response: ResponseChoice,
}

impl Message {
    pub fn new(name: Identifier, sender: Party, recipient: Party, content: Typ, response: ResponseChoice) -> Self {
        Message {
            name,
            sender,
            recipient,
            content,
            response,
        }
    }
}
