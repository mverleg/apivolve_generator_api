use ::std::fmt;

use ::serde::Deserialize;
use ::serde::Serialize;
use crate::gen1::evolution::TypeDeclaration;

use crate::gen1::evolution::util::{Identifier, Range};

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

impl fmt::Display for Party {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ResponseChoice {
    choices: Vec<[Message; 1]>,
    count: Range,
}

impl ResponseChoice {
    pub fn new(choices: Vec<[Message; 1]>, count: Range) -> Self {
        ResponseChoice {
            choices,
            count,
        }
    }

    pub fn choices(&self) -> &[Message] {
        &self.choices
    }

    pub fn count(&self) -> Range {
        self.count.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Message {
    name: Identifier,
    sender: Party,
    //TODO @mark: e.g. specific client, set of clients or all clients?
    recipient: Party,
    content: TypeDeclaration,
    /// One of these messages can be the response; empty means no response,
    response: ResponseChoice,
}

impl Message {
    pub fn new(name: Identifier, sender: Party, recipient: Party, content: TypeDeclaration, response: ResponseChoice) -> Self {
        Message {
            name,
            sender,
            recipient,
            content,
            response,
        }
    }
}
