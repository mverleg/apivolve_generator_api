use ::std::fmt;

pub use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GenerateInputLayout {
    /// The complete data layout per version.
    // Layout,
    /// The steps to be taken to parse and generate input per version.
    Steps,
}

impl Default for GenerateInputLayout {
    fn default() -> Self {
        GenerateInputLayout::Steps
    }
}

impl fmt::Display for GenerateInputLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerateInputLayout::Steps => write!(f, "steps"),
        }
    }
}

