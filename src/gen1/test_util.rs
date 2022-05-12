//! Provide tests to see if a generator handles some pre-defined cases.
//!
//! Since Apivolve can make almost no assumptions about the what generators do,
//! this only provides the data and checks for [Result] and [panic!].
//! It is up to the generator's author to test that the output makes sense.

use std::path::PathBuf;

fn noop_generator() {}

fn accept_all(_output_dir: &PathBuf) -> Result<(), String> {
    Ok(())
}

// macro_rules! testsuite_full {
//     ($generator_func:ident, $test_func:ident) => {
//         #[test]
//         fn test_no_versions() {
//             if let Err(err) = test_no_versions(generator_func) {
//                 panic!("apivolve generator failed: {}", err);
//             }
//         }
//     };
//     ($generator_func:ident) => {
//         testsuite_full!($generator_func, accept_all);
//     };
// }

pub use testsuite_full;
use crate::gen1::{AcceptsConfig, GenerationPreferences, Generator};

macro_rules! make_gen_test {
    ($generator_expr:expr, $test_func:ident) => {
        #[test]
        fn test_no_versions() {
            if let Err(err) = test_no_versions(generator_func) {
                panic!("apivolve generator failed: {}", err);
            }
        }
    };
}

#[test]
fn test_generate_no_versions(test_func: impl FnOnce(PathBuf)) {
    match generate_no_versions(accepts_config, make_generator) {
        Ok(path) => test_func(path),
        Err(err) => panic!("apivolve generator failed: {}", err),
    }
}

type GenFn<G: Generator> = impl FnOnce(GenerationPreferences) -> G;

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
testsuite_full!(noop_generator);
