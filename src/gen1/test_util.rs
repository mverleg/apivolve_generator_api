//! Provide tests to see if a generator handles some pre-defined cases.
//!
//! Since Apivolve can make almost no assumptions about the what generators do,
//! this only provides the data and checks for [Result] and [panic!].
//! It is up to the generator's author to test that the output makes sense.

use ::std::path::PathBuf;

use ::async_trait::async_trait;
use ::semver::Version;

use crate::gen1::{AcceptsConfig, Evolution, GenerationPreferences, Generator, GenResult};

fn noop_generator() {}

fn accept_all(_output_dir: &PathBuf) -> Result<(), String> {
    Ok(())
}

macro_rules! testsuite_basic {
    ($accepts_config_expr: expr, $make_generator_expr: expr, $verify_func_ident: ident) => {
        make_gen_test!(test_no_versions, generate_no_versions, $accepts_config_expr, $make_generator_expr, $verify_func_ident);
        make_gen_test!(test_core_features, generate_core_features, $accepts_config_expr, $make_generator_expr, $verify_func_ident);
        make_gen_test!(test_with_pending, generate_with_pending, $accepts_config_expr, $make_generator_expr, $verify_func_ident);
        //TODO @mark: more tests
    };
    ($accepts_config_expr: expr, $make_generator_expr: expr) => {
        testsuite_full!($accepts_config_expr: expr, $make_generator_expr: expr, accept_all);
    };
}

macro_rules! testsuite_full {
    ($accepts_config_expr: expr, $make_generator_expr: expr, $verify_func_ident: ident) => {
        testsuite_basic!($accepts_config_expr, $make_generator_expr, $verify_func_ident);
        //TODO @mark: more tests
    };
    ($accepts_config_expr: expr, $make_generator_expr: expr) => {
        testsuite_basic!($accepts_config_expr: expr, $make_generator_expr: expr, accept_all);
    };
}

pub use testsuite_basic;
pub use testsuite_full;

macro_rules! make_gen_test {
    ($test_name: ident, $gen_test_ident: ident, $accepts_config_expr: expr, $make_generator_expr: expr, $verify_func_ident: ident) => {
        #[test]
        fn $test_name() {
            let accepts_config: AcceptsConfig = $accepts_config_expr;
            let make_generator: FnOnce(GenerationPreferences) -> G = $make_generator_expr;
            let verify_func: FnOnce(PathBuf) = $verify_func_ident;
            match $gen_test_ident(accepts_config, make_generator) {
                Ok(path) => $verify_func_ident(path),
                Err(err) => panic!("apivolve generator failed: {}", err),
            }
        }
    }
}

pub fn generate_no_versions<G: Generator, GenFn: FnOnce(GenerationPreferences) -> G>(
    accepts_config: AcceptsConfig,
    make_generator: GenFn,
) -> Result<PathBuf, String> {
    unimplemented!()
}

pub fn generate_core_features<G: Generator, GenFn: FnOnce(GenerationPreferences) -> G>(
    accepts_config: AcceptsConfig,
    make_generator: GenFn,
) -> Result<PathBuf, String> {
    unimplemented!()
}

pub fn generate_with_pending<G: Generator, GenFn: FnOnce(GenerationPreferences) -> G>(
    accepts_config: AcceptsConfig,
    make_generator: GenFn,
) -> Result<PathBuf, String> {
    unimplemented!()
}

#[cfg(test)]
struct NoopGenerator();

#[cfg(test)]
#[async_trait]
impl Generator for NoopGenerator {
    async fn generate_pending(&mut self, evolution: Evolution) -> GenResult {
        Ok(())
    }

    async fn generate_version(&mut self, version: Version, evolution: Evolution) -> GenResult {
        Ok(())
    }

    async fn finalize(self) -> GenResult {
        Ok(())
    }
}

#[cfg(test)]
fn noop_generator_factory(_: GenerationPreferences) -> NoopGenerator {
    NoopGenerator()
}

#[cfg(test)]
testsuite_full!(AcceptsConfig{
    apivolve_version: Version::new(1, 0, 0),
    encoding: GenerateInputFormat::Json,
    ..Default::default()
}, noop_generator_factory);
