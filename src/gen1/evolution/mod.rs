use ::serde::Deserialize;
use ::serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Evolution {
}
