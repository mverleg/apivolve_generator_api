use ::semver::Version;

use crate::gen1::VersionEvolution;

pub type ErrMsg = String;

#[derive(Debug, Clone)]
pub enum GenResult {
    Ok,
    FinishEarly,
    Error(ErrMsg),
}

pub trait Generator {
    /// Will be called once at the start, only if there are any pending changes.
    fn generate_pending(&mut self, evolution: VersionEvolution) -> GenResult;

    /// Will be called for each version, from newest to oldest, after `generate_pending`.
    fn generate_version(&mut self, version: Version, evolution: VersionEvolution) -> GenResult;

    /// Will be called exactly once at the end if all prior steps were successful.
    fn finalize(self) -> Result<(), ErrMsg>;
}
