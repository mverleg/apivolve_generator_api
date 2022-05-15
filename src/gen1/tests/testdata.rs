use ::tempdir::TempDir;
use ::semver::Version;

use crate::gen1::{AcceptsConfig, Evolution, GenerationPreferences, Generator};
use crate::gen1::tests::generate::test_with_data;

pub fn generate_no_versions<G, GenFn>(
    accepts_config: AcceptsConfig,
    make_generator: GenFn,
) -> Result<TempDir , String>
        where G: Generator, GenFn: FnOnce(GenerationPreferences) -> Result<G, String> {
    test_with_data(accepts_config, make_generator, None, vec![])
}

pub fn generate_core_features<G: Generator, GenFn: FnOnce(GenerationPreferences) -> Result<G, String>> (
    accepts_config: AcceptsConfig,
    make_generator: GenFn,
) -> Result<TempDir, String> {
    test_with_data(accepts_config, make_generator, None, vec![
        (Version::new(0, 2, 0), Evolution {}),
        (Version::new(0, 1, 0), Evolution {}),
    ])
}

pub fn generate_with_pending<G: Generator, GenFn: FnOnce(GenerationPreferences) -> Result<G, String>> (
    accepts_config: AcceptsConfig,
    make_generator: GenFn,
) -> Result<TempDir, String> {
    test_with_data(accepts_config, make_generator, None, vec![])?;
    unimplemented!()
}
