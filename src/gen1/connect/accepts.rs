pub use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;
#[cfg(test)]
use ::smallvec::smallvec;

use crate::gen1::connect::format::GenerateInputFormat;
use crate::gen1::connect::layout::GenFeatures;
use crate::gen1::connect::layout::GenFeature;

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
        GenFeature::Parser,
        GenFeature::Parser,
        GenFeature::Validator,
        GenFeature::Documentation,
    ];
    let json = serde_json::to_string(&AcceptsConfig {
        apivolve_version: Version::new(1, 2, 4),
        features: GenFeatures::new(features),
        encoding: GenerateInputFormat::Json,
    })
        .unwrap();
    assert_eq!(
        json,
        r#"{"apivolve_version":"1.2.4","features":{"features":["documentation","parser","validator"]},"encoding":"json"}"#
    );
}

#[test]
fn deserialize() {
    let config: AcceptsConfig = serde_json::from_str(
        r#"{"apivolve_version":"1.2.4","features":{"features":[
        "parser","validator"]},"encoding":"json"}"#,
    )
        .unwrap();
    assert_eq!(
        config,
        AcceptsConfig {
            apivolve_version: Version::new(1, 2, 4),
            features: GenFeatures::new(smallvec![
                GenFeature::Parser,
                GenFeature::Validator,
            ]),
            encoding: GenerateInputFormat::Json,
        }
    )
}
