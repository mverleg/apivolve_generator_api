use ::std::fmt;

pub use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GenerateInputFormat {
    Json,
}

impl Default for GenerateInputFormat {
    fn default() -> Self {
        GenerateInputFormat::Json
    }
}

impl fmt::Display for GenerateInputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerateInputFormat::Json => write!(f, "json"),
        }
    }
}
