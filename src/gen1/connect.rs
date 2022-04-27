use ::std::fmt;

pub use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenerateConfig {
    pub apivolve_version: Version,
    #[serde(default)]
    pub data_structure: GenerateInputLayout,
    #[serde(default)]
    pub encoding: GenerateInputFormat,
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
        })
            .unwrap();
        assert_eq!(
            json,
            "{\"apivolve_version\":\"1.2.4\",\"data_structure\":\"steps\",\"encoding\":\"json\"}"
        );
    }

    #[test]
    fn deserialize() {
        let config: GenerateConfig = serde_json::from_str(
            "{\"apivolve_version\":\"1.2.4\",\
                \"data_structure\":\"steps\",\"encoding\":\"json\"}",
        )
            .unwrap();
        assert_eq!(
            config,
            GenerateConfig {
                apivolve_version: Version::new(1, 2, 4),
                data_structure: GenerateInputLayout::Steps,
                encoding: GenerateInputFormat::Json,
            }
        )
    }
}
