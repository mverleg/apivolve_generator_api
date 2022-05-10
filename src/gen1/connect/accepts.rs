use crate::gen1::connect::format::GenerateInputFormat;
use crate::gen1::connect::layout::GenerateInputLayout;
pub use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AcceptsConfig {
    pub apivolve_version: Version,
    pub data_structure: GenerateInputLayout,
    pub encoding: GenerateInputFormat,
}

#[test]
fn serialize() {
    let json = serde_json::to_string(&AcceptsConfig {
        apivolve_version: Version::new(1, 2, 4),
        data_structure: GenerateInputLayout::Steps,
        encoding: GenerateInputFormat::Json,
    })
    .unwrap();
    assert_eq!(
        json,
        "{\"apivolve_version\":\"1.2.4\",\"data_structure\":\"Steps\",\"encoding\":\"Json\"}"
    );
}

#[test]
fn deserialize() {
    let config: AcceptsConfig = serde_json::from_str(
        "{\"apivolve_version\":\"1.2.4\",\
            \"data_structure\":\"Steps\",\"encoding\":\"Json\"}",
    )
    .unwrap();
    assert_eq!(
        config,
        AcceptsConfig {
            apivolve_version: Version::new(1, 2, 4),
            data_structure: GenerateInputLayout::Steps,
            encoding: GenerateInputFormat::Json,
        }
    )
}
