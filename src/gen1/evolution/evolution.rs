use ::std::fmt;

use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;

use crate::gen1::data::{Message, Party, TypeDeclaration};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VersionEvolution {
    pub version: Version,
    #[serde(flatten)]
    pub checksum: Checksum,
    pub declaration: Vec<(ChangeStatus, TypeDeclaration)>,
    pub parties: Vec<(ChangeStatus, Party)>,
    pub messages: Vec<MessageOperation>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChangeStatus {
    Unchanged { same_as: Version },
    New,
    Changed { previous: Version },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageOperation {
    /// This message is the same as in the given later version.
    Unchanged { same_as: Version, typ: Message },
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
    Implement { typ: Message },
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Checksum {
    value: String,
}

impl fmt::Display for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
