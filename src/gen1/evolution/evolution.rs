use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;

use crate::gen1::evolution::typ::RecordType;
use crate::gen1::evolution::TypeDeclaration;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Evolution {
    pub operations: Vec<TypeOperation>,
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TypeOperation {
    /// This object is the same as in the given later version.
    Unchanged {
        same_as: Version,
        typ: TypeDeclaration,
    },
    /// This object is new or changed, but will be converted to a newer
    /// version after parsing, it won't be handled by user code.
    Convert {
        parse_typ: TypeDeclaration,
        target_version: Version,
        target_typ: TypeDeclaration,
    },
    /// This object is new or changed, and will be handled by user code,
    /// either because it is the latest version, or because there was a
    /// backwards-incompatible change.
    Handle {
        typ: TypeDeclaration,
    },
}

impl Evolution {
    pub fn new(operations: impl Into<Vec<TypeOperation>>, messages: impl Into<Vec<Message>>) -> Self {
        Evolution {
            operations: operations.into(),
            messages: messages.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Message {
    //TODO @mark: differentiate whether it's bidirectional (default) or always sent from older to newer, or from newer to older (this may happen in a client-server situation where servers upgrade before clients)
    data: RecordType,  //TODO @mark: change to reference
}

impl Message {
    pub fn new(data: RecordType) -> Self {
        Message { data }
    }
}
