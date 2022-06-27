use ::std::path::PathBuf;

pub use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;
use ::smallvec::SmallVec;

use crate::gen1::data::Party;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UserPreferences {
    pub apivolve_version: Version,
    pub output_dir: PathBuf,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extra_args: Vec<String>,
    pub requested_parties: SmallVec<[Party; 3]>,
}

#[cfg(test)]
mod tests {
    use ::smallvec::smallvec;

    use super::*;

    #[test]
    fn serialize() {
        let json = serde_json::to_string(&UserPreferences {
            apivolve_version: Version::new(1, 2, 4),
            output_dir: PathBuf::from("/tmp"),
            extra_args: vec!["--strict".to_string()],
            requested_parties: smallvec![],
        })
        .unwrap();
        assert_eq!(
            json,
            "{\"apivolve_version\":\"1.2.4\",\"output_dir\":\"/tmp\",\"extra_args\":[\"--strict\"]}"
        );
    }

    #[test]
    fn deserialize() {
        let config: UserPreferences = serde_json::from_str(
            "{\"apivolve_version\":\"1.2.4\",\"output_dir\":\"/tmp\",\"extra_args\":[\"--strict\"]}",
        )
            .unwrap();
        assert_eq!(
            config,
            UserPreferences {
                apivolve_version: Version::new(1, 2, 4),
                output_dir: PathBuf::from("/tmp"),
                extra_args: vec!["--strict".to_string()],
                requested_parties: smallvec![],
            }
        )
    }
}
