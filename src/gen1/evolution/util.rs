use ::serde::Deserialize;
use ::serde::Serialize;

pub type ErrMsg = String;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
//TODO @mark: serialize as just a string
pub struct Identifier {
    name: String,
}

impl Identifier {
    pub fn new(name: impl Into<String>) -> Result<Self, ErrMsg> {
        let name = name.into();
        //TODO @mark: validate name
        Ok(Identifier { name })
    }
}
