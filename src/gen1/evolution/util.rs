use ::std::fmt;

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

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.name)
    }
}

/// Non-empty, positive, inclusive range
//TODO @mark: maybe serialize as tuple?
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Range {
    min: u32,
    max: u32,
}

impl Range {
    pub fn new(min: u32, max: u32) -> Self {
        assert!(min <= max);
        Range {
            min,
            max
        }
    }

    pub fn single_value(value: u32) -> Self {
        Range::new(value, value)
    }

    pub fn min(&self) -> u32 {
        self.min
    }

    pub fn max(&self) -> u32 {
        self.max
    }

    pub fn len(&self) -> u32 {
        self.max - self.min + 1
    }
}