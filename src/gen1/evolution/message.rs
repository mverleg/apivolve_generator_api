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
            name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Message {
    sender: Party,
    //TODO @mark: e.g. specific client, set of clients or all clients?
    recipient: Party,
    content: Typ,
}

impl Message {
    pub fn new(sender: Party, recipient: Party, content: Typ) -> Self {
        Message {
            sender,
            recipient,
            content,
        }
    }
}
