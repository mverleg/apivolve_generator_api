use ::semver::Version;
use ::serde::Deserialize;
use ::serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenerateSteps {
    pub versions: Vec<(Version, Vec<Step>)>,
    pub pending: Vec<Step>,
}

impl GenerateSteps {
    pub fn new() -> Self {
        GenerateSteps { versions: vec![], pending: vec![] }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Step {
    source: Source,
}

/// The source file and line that created this step.
/// Intended just for logging and reporting; do not touch the files (which may not be there).
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Source {
    /// e.g.: v1.2.4/add_user.apiv:10
    locator: String,
}
