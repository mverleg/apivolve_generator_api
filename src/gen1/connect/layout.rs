use std::fmt::Debug;
use std::hash::{Hash, Hasher};
pub use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;

use ::smallvec::smallvec;
use ::smallvec::SmallVec;

type GenFeatureSet = SmallVec<[GenFeature; 8]>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GenFeatures {
    features: GenFeatureSet,
}

impl GenFeatures {
    pub fn new(features: impl Into<GenFeatureSet>) -> Self {
        //TODO @mark: make features unique
        let mut features = features.into();
        features.sort_unstable();
        features.dedup();
        GenFeatures { features }
    }

    pub fn all() -> Self {
        Self::new(smallvec![
            //TODO @mark: implement all these features
            GenFeature::Documentation,
            GenFeature::Examples,
            GenFeature::Parser,
            GenFeature::Validator,
        ])
    }
}

impl From<Vec<GenFeature>> for GenFeatures {
    fn from(features: Vec<GenFeature>) -> Self {
        GenFeatures::new(features)
    }
}

impl Default for GenFeatures {
    fn default() -> Self {
        GenFeatures::all()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum GenFeature {
    /// Evolution documentation.
    Documentation,
    /// Any example data that the evolution authors provided.
    Examples,
    /// The steps a parser would take to generate the input per version.
    Parser,
    /// The steps a validation wouild take to raise the necessary validation errors.
    Validator,
}

#[derive(Debug, Clone)]
pub enum Gate<T: Debug + Clone> {
    Enabled(T),
    Disabled,
}

impl <T> PartialEq for Gate<T>
        where T: PartialEq + Debug + Clone {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Gate::Enabled(a), Gate::Enabled(b)) => a == b,
            (Gate::Disabled, Gate::Disabled) => true,
            _ => false,
        }
    }
}

impl <T> Eq for Gate<T>
        where T: Eq + Debug + Clone, Gate<T>: PartialEq {}

impl <T> Hash for Gate<T>
        where T: Hash + Debug + Clone {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u8(37);
        if let Gate::Enabled(value) = self {
            value.hash(state);
        }
    }
}
