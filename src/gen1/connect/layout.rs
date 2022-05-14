use ::std::fmt;

pub use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;

use ::smallvec::smallvec;
use ::smallvec::SmallVec;

type FeatureCollection = SmallVec<[GenerateInputFeature; 8]>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GenerateInputFeatures {
    features: FeatureCollection,
}

impl GenerateInputFeatures {
    pub fn new(features: impl Into<FeatureCollection>) -> Self {
        //TODO @mark: make features unique
        let mut features = features.into();
        features.sort_unstable();
        features.dedup();
        GenerateInputFeatures {
            features,
        }
    }
}

impl From<Vec<GenerateInputFeature>> for GenerateInputFeatures {
    fn from(features: Vec<GenerateInputFeature>) -> Self {
        GenerateInputFeatures::new(features)
    }
}

impl Default for GenerateInputFeatures {
    fn default() -> Self {
        GenerateInputFeatures {
            features: smallvec![
                GenerateInputFeature::Documentation,
                GenerateInputFeature::Parser,
                GenerateInputFeature::Validator,
                GenerateInputFeature::Examples,
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum GenerateInputFeature {
    /// Evolution documentation.
    Documentation,
    /// The steps a parser would take to generate the input per version.
    Parser,
    /// The steps a validation wouild take to raise the necessary validation errors.
    Validator,
    /// Any example data that the evolution authors provided.
    Examples,
}

impl Default for GenerateInputFeature {
    fn default() -> Self {
        GenerateInputFeature::Steps
    }
}

impl fmt::Display for GenerateInputFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerateInputFeature::Steps => write!(f, "steps"),
        }
    }
}
