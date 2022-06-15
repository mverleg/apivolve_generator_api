use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;

use crate::gen1::evolution::typ::RecordType;
use crate::gen1::evolution::TypeDeclaration;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Evolution {
    //TODO @mark: postfix
    pub declarations: Vec<(TypeStatus, TypeDeclaration)>,
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TypeStatus {
    /// This object is the same as in a previous version
    Unchanged(Version),
    /// This object is new or changed, but will be converted to a newer
    /// version after parsing, it won't be handled by user code.
    ForParsing(Version),
    /// This object is new or changed, and will be handled by user code,
    /// either because it is the latest version, or because there was a
    /// backwards-incompatible change.
    Handle(),
}

impl Evolution {
    pub fn new(declarations: impl Into<Vec<TypeDeclaration>>, messages: impl Into<Vec<Message>>) -> Self {
        Evolution {
            declarations: declarations.into(),
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
