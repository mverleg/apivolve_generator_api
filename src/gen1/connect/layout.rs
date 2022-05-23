use ::std::fmt::Debug;
use ::std::hash::{Hash, Hasher};

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

macro_rules! make_gate {
    ($GateName: ident, $feature_name: expr) => {
        #[allow(unused)]
        #[derive(Debug, Clone)]
        pub enum $GateName<T: Debug + Clone> {
            Enabled(T),
            Disabled,
        }

        #[allow(unused)]
        impl <T: Debug + Clone> $GateName<T> {
            pub fn of(value: T) -> Self {
                $GateName::Enabled(value)
            }

            pub fn none() -> Self {
                $GateName::Disabled
            }

            pub fn get(&self) -> &T {
                match self {
                    $GateName::Enabled(value) => &value,
                    $GateName::Disabled => panic!("Feature '{}' is disabled", $feature_name),
                }
            }

            pub fn unwrap(self) -> T {
                match self {
                    $GateName::Enabled(value) => value,
                    $GateName::Disabled => panic!("Feature '{}' is disabled", $feature_name),
                }
            }
        }

        impl <T> PartialEq for $GateName<T>
                where T: PartialEq + Debug + Clone {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    ($GateName::Enabled(a), $GateName::Enabled(b)) => a == b,
                    ($GateName::Disabled, $GateName::Disabled) => true,
                    _ => false,
                }
            }
        }

        impl <T> Eq for $GateName<T>
                where T: Eq + Debug + Clone, $GateName<T>: PartialEq {}

        impl <T> Hash for $GateName<T>
                where T: Hash + Debug + Clone {
            fn hash<H: Hasher>(&self, state: &mut H) {
                state.write_u8(37);
                if let $GateName::Enabled(value) = self {
                    value.hash(state);
                }
            }
        }
    }
}

make_gate!(DocumentationGate, "documentation");
make_gate!(ExamplesGate, "examples");
make_gate!(ParserGate, "parser");
make_gate!(ValidatorGate, "validator");

#[cfg(test)]
mod tests {
    use super::*;

    make_gate!(TestGate, "testgate");

    #[test]
    fn gate() {
        let gate = TestGate::of("test");
        assert_eq!(gate.get(), &"test");
    }
}
