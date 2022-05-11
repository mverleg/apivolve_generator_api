use ::async_trait::async_trait;
use ::semver::Version;

use crate::gen1::Evolution;

pub type ErrMsg = String;
pub type GenResult = Result<(), ErrMsg>;

#[async_trait]
pub trait Generator {
    /// Will be called once at the start, only if there are any pending changes.
    async fn generate_pending(&mut self, evolution: Evolution) -> GenResult;

    /// Will be called for each version, from newest to oldest, after `generate_pending`.
    async fn generate_version(&mut self, version: Version, evolution: Evolution) -> GenResult;

    /// Will be called exactly once at the end if all prior steps were successful.
    async fn finalize(self) -> GenResult;
}
