//! Provide tests to see if a generator handles some pre-defined cases.
//!
//! Since Apivolve can make almost no assumptions about the what generators do,
//! this only provides the data and checks for [Result] and [panic!].
//! It is up to the generator's author to test that the output makes sense.

use std::path::PathBuf;
use semver::Version;

fn noop_generator() {}

fn accept_all(_output_dir: &PathBuf) -> Result<(), String> {
    Ok(())
}

macro_rules! testsuite_basic {
    ($accepts_config_expr: expr, $make_generator_expr: expr, $verify_func_iden: iden) => {
        make_gen_test!(generate_no_versions, $accepts_config_expr, $make_generator_expr, $verify_func_iden);
        make_gen_test!(generate_core_features, $accepts_config_expr, $make_generator_expr, $verify_func_iden);
        make_gen_test!(generate_with_pending, $accepts_config_expr, $make_generator_expr, $verify_func_iden);
        //TODO @mark: more tests
    };
    ($accepts_config_expr: expr, $make_generator_expr: expr) => {
        testsuite_full!($accepts_config_expr: expr, $make_generator_expr: expr, accept_all);
    };
}

macro_rules! testsuite_full {
    ($accepts_config_expr: expr, $make_generator_expr: expr, $verify_func_iden: iden) => {
        testsuite_basic!($accepts_config_expr, $make_generator_expr, $verify_func_iden);
        //TODO @mark: more tests
    };
    ($accepts_config_expr: expr, $make_generator_expr: expr) => {
        testsuite_basic!($accepts_config_expr: expr, $make_generator_expr: expr, accept_all);
    };
}

pub use testsuite_full;
use crate::gen1::{AcceptsConfig, Evolution, GenerationPreferences, Generator, GenResult};

type GenFn<G: Generator> = impl FnOnce(GenerationPreferences) -> G;

macro_rules! make_gen_test {
    ($test_iden: iden, $accepts_config_expr: expr, $make_generator_expr: expr, $verify_func_iden: iden) => {
        #[test]
        fn test_$test_iden(test_func: impl FnOnce(PathBuf)) {
            let accepts_config: AcceptsConfig = $accepts_config_expr;
            let make_generator: GenFn = $make_generator_expr;
            let verify_func: impl FnOnce(PathBuf) = $verify_func_iden;
            match $test_iden(accepts_config, make_generator) {
                Ok(path) => $verify_func_iden(path),
                Err(err) => panic!("apivolve generator failed: {}", err),
            }
        }
    }
}

pub fn generate_no_versions<G: Generator>(
    accepts_config: AcceptsConfig,
    make_generator: GenFn<G>,
) -> Result<PathBuf, String> {
    unimplemented!()
}

pub fn generate_core_features<G: Generator>(
    accepts_config: AcceptsConfig,
    make_generator: GenFn<G>,
) -> Result<PathBuf, String> {
    unimplemented!()
}

pub fn generate_with_pending<G: Generator>(
    accepts_config: AcceptsConfig,
    make_generator: GenFn<G>,
) -> Result<PathBuf, String> {
    unimplemented!()
}

#[cfg(test)]
struct NoopGenerator();

#[cfg(test)]
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
testsuite_full!(AcceptsConfig{}, noop_generator_factory);
