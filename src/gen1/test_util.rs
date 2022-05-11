//! Provide tests to see if a generator handles some pre-defined cases.
//!
//! Since Apivolve can make almost no assumptions about the what generators do,
//! this only provides the data and checks for [Result] and [panic!].
//! It is up to the generator's author to test that the output makes sense.

use std::path::PathBuf;

fn accept_all(_output_dir: &PathBuf) -> Result<(), String> {
    Ok(())
}

macro_rules! testsuite_full {
    ($generator_func:ident, $test_func:ident) => {
        #[test]
        fn no_versions() {
            if let Err(err) = test_no_versions(generator_func) {
                panic!("apivolve generator failed: {}", err);
            }
        }
    };
    ($generator_func:ident) => {
        testsuite_full!($generator_func, accept_all);
    };
}

pub use testsuite_full;

pub fn no_versions() {

}

pub fn core_features() {

}

pub fn with_pending() {

}
