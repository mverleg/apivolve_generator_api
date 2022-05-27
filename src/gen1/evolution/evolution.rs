use ::serde::Deserialize;
use ::serde::Serialize;

use crate::gen1::evolution::typ::RecordType;
use crate::gen1::evolution::TypeDeclaration;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Evolution {
    //TODO @mark: postfix
    pub declarations: Vec<TypeDeclaration>,
    pub messages: Vec<Message>,
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
