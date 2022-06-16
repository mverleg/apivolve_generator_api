use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;

use crate::gen1::evolution::message::{Message, Party};
use crate::gen1::evolution::typ::RecordType;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Evolution {
    pub declaration: Vec<DeclarationStatus>,
    pub parties: Vec<Party>,
    pub messages: Vec<MessageOperation>,
}

//TODO @mark: I should switch from messages to protocols (interactions of messages)

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageOperation {
    /// This message is the same as in the given later version.
    Unchanged {
        same_as: Version,
        typ: Message,
    },
    /// This message is new or changed, but will be converted to a newer
    /// version after parsing, it won't be handled by user code.
    Convert {
        parse_typ: Message,
        target_version: Version,
        target_typ: Message,
    },
    /// This object is new or changed, and will be handled by user code,
    /// either because it is the latest version, or because there was a
    /// backwards-incompatible change.
    Handle {
        typ: Message,
    },
}

impl Evolution {
    pub fn new(operations: impl Into<Vec<MessageOperation>>, messages: impl Into<Vec<Message>>) -> Self {
        Evolution {
            operations: operations.into(),
            messages: messages.into(),
        }
    }
}
