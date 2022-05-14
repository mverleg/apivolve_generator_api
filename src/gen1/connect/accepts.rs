pub use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;
use ::smallvec::smallvec;

use crate::gen1::connect::format::GenerateInputFormat;
use crate::gen1::connect::layout::GenFeatures;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AcceptsConfig {
    pub apivolve_version: Version,
    pub features: GenFeatures,
    pub encoding: GenerateInputFormat,
}

#[test]
fn serialize() {
    let features = smallvec![
        GenerateInputFeature::Parser,
        GenerateInputFeature::Parser,
        GenerateInputFeature::Validator,
        GenerateInputFeature::Documentation,
    ];
    let json = serde_json::to_string(&AcceptsConfig {
        apivolve_version: Version::new(1, 2, 4),
        features: GenFeatures::new(features),
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
    let config: AcceptsConfig = serde_json::from_str(
        "{\"apivolve_version\":\"1.2.4\",\
            \"data_structure\":\"steps\",\"encoding\":\"json\"}",
    )
        .unwrap();
    assert_eq!(
        config,
        AcceptsConfig {
            apivolve_version: Version::new(1, 2, 4),
            features: GenFeatures::default(),
            encoding: GenerateInputFormat::Json,
        }
    )
}
