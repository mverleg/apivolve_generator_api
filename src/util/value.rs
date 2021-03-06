use ::std::fmt;
use std::str::FromStr;

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

impl FromStr for Identifier {
    type Err = ErrMsg;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Identifier::new(text)
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
    pub fn try_new(min: u32, max: u32) -> Option<Self> {
        if min > max {
            return None;
        }
        Some(Range { min, max })
    }

    pub fn new(min: u32, max: u32) -> Self {
        Range::try_new(min, max)
            .unwrap_or_else(|| panic!("not a valid range: {} to {} inclusive", min, max))
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

    pub fn is_empty(&self) -> bool {
        self.max <= self.min
    }
}
