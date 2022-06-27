pub use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;
use smallvec::SmallVec;

use crate::gen1::connect::layout::GenFeatures;
use crate::gen1::data::Party;

/// Specifies which inputs the generator expects from Apivolve.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FunctionalityRequest {
    pub features: GenFeatures,
    pub generate_parties: SmallVec<[Party; 3]>,
}

#[cfg(test)]
mod tests {
    pub use ::semver::Version;
    use ::smallvec::smallvec;

    use crate::gen1::connect::layout::GenFeature;
    use crate::gen1::connect::layout::GenFeatures;
    use crate::Identifier;

    use super::*;

    #[test]
    fn serialize() {
        let features = smallvec![
            GenFeature::Parser,
            GenFeature::Parser,
            GenFeature::Validator,
            GenFeature::Documentation,
        ];
        let json = serde_json::to_string(&FunctionalityRequest {
            features: GenFeatures::new(features),
            generate_parties: smallvec![Party::new(Identifier::new("server").unwrap())],
        })
        .unwrap();
        assert_eq!(
            json,
            r#"{"apivolve_version":"1.2.4","features":{"features":["documentation","parser","validator"]},"encoding":"json"}"#
        );
    }

    #[test]
    fn deserialize() {
        let config: FunctionalityRequest = serde_json::from_str(
            r#"{"apivolve_version":"1.2.4","features":{"features":[
            "parser","validator"]},"encoding":"json"}"#,
        )
        .unwrap();
        assert_eq!(
            config,
            FunctionalityRequest {
                features: GenFeatures::new(smallvec![GenFeature::Parser, GenFeature::Validator,]),
                generate_parties: smallvec![Party::new(Identifier::new("server").unwrap())]
            }
        )
    }
}
