use ::std::fmt;

pub use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;

type FeatureCollection = SmallVec<[GenerateInputFeatures; 8]>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GenerateInputFeatures {
    features: FeatureCollection,
}

impl GenerateInputFeatures {
    pub fn new(features: impl Info<FeatureCollection>) -> Self {
        //TODO @mark: make features unique
        GenerateInputFeatures {
            features: SmallVec::new(),
        }
    }
}

impl Default for GenerateInputFeatures {
    fn default() -> Self {
        GenerateInputFeatures {
            features: smallvec![GenerateInputFeature::Documentation, GenerateInputFeature::Parser, GenerateInputFeature::Validator],
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum GenerateInputFeature {
    /// Evolution documentation.
    Documentation,
    /// The steps a parser would take to generate the input per version.
    Parser,
    /// The steps a validation wouild take to raise the necessary validation errors.
    Validator,
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
