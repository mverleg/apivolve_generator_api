use ::std::fmt;

use ::semver::Version;

use crate::gen::gen1::{GenerateInputFormat, GenerateInputLayout};

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateConfig {
    apivolve_version: Version,
    data_structure: GenerateInputLayout,
    encoding: GenerateInputFormat,
}

impl GenerateConfig {
    pub fn new(apivolve_version: Version, data_structure: GenerateInputLayout, encoding: GenerateInputFormat) -> Self {
        GenerateConfig { apivolve_version, data_structure, encoding }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GenerateInputLayout {
    /// The complete data layout per version.
    Layout,
    /// The steps to be taken to parse and generate input per version.
    Steps,
}

impl fmt::Display for GenerateInputLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerateInputLayout::Layout => write!(f, "layout"),
            GenerateInputLayout::Steps => write!(f, "steps"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GenerateInputFormat {
    Json,
}

impl fmt::Display for GenerateInputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerateInputFormat::Json => write!(f, "json"),
        }
    }
}
