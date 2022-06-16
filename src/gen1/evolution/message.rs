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
#[serde(tag = "type", rename_all = "snake_case")]
//TODO @mark: name
pub enum ResponseChoice {
    Optional(SmallVec<[Message; 1]>),
    Required(SmallVec<[Message; 1]>),
    //TODO @mark: multiple?
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
    pub fn new(sender: Party, recipient: Party, content: Typ, response: impl Into<SmallVec<[Message; 1]>>) -> Self {
        Message {
            sender,
            recipient,
            content,
            response: response.into(),
        }
    }
}
