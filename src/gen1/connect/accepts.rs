pub use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;

use crate::gen1::connect::format::GenerateInputFormat;

/// Specifies which inputs the generator expects from Apivolve.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AcceptsConfig {
    pub apivolve_version: Version,
    pub encoding: GenerateInputFormat,
}

#[cfg(test)]
mod tests {
    pub use ::semver::Version;
    use ::smallvec::smallvec;

    use crate::gen1::connect::evpref::EvolutionPreferences;

    use crate::gen1::connect::format::GenerateInputFormat;
    use crate::gen1::connect::layout::GenFeature;
    use crate::gen1::connect::layout::GenFeatures;

    use super::*;

    #[test]
    fn serialize() {
        let _json = serde_json::to_string(&AcceptsConfig {
            apivolve_version: Version::new(1, 2, 4),
            encoding: GenerateInputFormat::Json,
        })
        .unwrap();
        let features = GenFeatures::new(smallvec![
            GenFeature::Parser,
            GenFeature::Parser,
            GenFeature::Validator,
            GenFeature::Documentation,
        ]);
        let json = serde_json::to_string(&EvolutionPreferences {
            features,
            generate_parties: smallvec![], //TODO @mark: all
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
                encoding: GenerateInputFormat::Json,
            }
        )
    }
}
