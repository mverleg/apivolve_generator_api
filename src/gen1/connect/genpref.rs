use ::std::path::PathBuf;

pub use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GenerationPreferences {
    pub apivolve_version: Version,
    pub output_dir: PathBuf,
    pub extra_args: Vec<String>,
}

#[test]
fn serialize() {
    let json = serde_json::to_string(&GenerationPreferences {
        apivolve_version: Version::new(1, 2, 4),
        output_dir: PathBuf::from("/tmp"),
        extra_args: vec!["--strict".to_string()],
    })
    .unwrap();
    assert_eq!(
        json,
        "{\"apivolve_version\":\"1.2.4\",\"output_dir\":\"/tmp\",\"extra_args\":[\"--strict\"]}"
    );
}

#[test]
fn deserialize() {
    let config: GenerationPreferences = serde_json::from_str(
        "{\"apivolve_version\":\"1.2.4\",\"output_dir\":\"/tmp\",\"extra_args\":[\"--strict\"]}",
    )
    .unwrap();
    assert_eq!(
        config,
        GenerationPreferences {
            apivolve_version: Version::new(1, 2, 4),
            output_dir: PathBuf::from("/tmp"),
            extra_args: vec!["--strict".to_string()],
        }
    )
}
