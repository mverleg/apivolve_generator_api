use ::std::fmt;

pub use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GenerateInputLayout {
    /// The complete data layout per version.
    // Layout,
    /// The steps to be taken to parse and generate input per version.
    Steps,
}

impl fmt::Display for GenerateInputLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerateInputLayout::Steps => write!(f, "steps"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let json = serde_json::to_string(&GenerateConfig {
            apivolve_version: Version::new(1, 2, 4),
            data_structure: GenerateInputLayout::Steps,
            encoding: GenerateInputFormat::Json,
        }).unwrap();
        assert_eq!(json, "{\"apivolve_version\":\"1.2.4\",\"data_structure\":\"Steps\",\"encoding\":\"Json\"}");
    }

    #[test]
    fn deserialize() {
        let config: GenerateConfig = serde_json::from_str("{\"apivolve_version\":\"1.2.4\",\
                \"data_structure\":\"Steps\",\"encoding\":\"Json\"}").unwrap();
        assert_eq!(config, GenerateConfig {
            apivolve_version: Version::new(1, 2, 4),
            data_structure: GenerateInputLayout::Steps,
            encoding: GenerateInputFormat::Json,
        })
    }
}
