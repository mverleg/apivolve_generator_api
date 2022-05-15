//! Provide tests to see if a generator handles some pre-defined cases.
//!
//! Since Apivolve can make almost no assumptions about the what generators do,
//! this only provides the data and checks for [Result] and [panic!].
//! It is up to the generator's author to test that the output makes sense.

use ::std::path::PathBuf;
use futures::executor::block_on;

use tempdir::TempDir;

use crate::gen1::{AcceptsConfig, GenerationPreferences, Generator};

fn noop_generator() {}

fn accept_all(_output_dir: TempDir) -> Result<(), String> {
    Ok(())
}

#[allow(unused_macros)] // it is not unused, don't know why it's not being detected
macro_rules! make_gen_test {
    ($test_name: ident, $gen_test_ident: ident, $accepts_config_expr: expr, $make_generator_expr: expr, $verify_func_ident: ident) => {
        #[test]
        fn $test_name() {
            let accepts_config: AcceptsConfig = $accepts_config_expr;
            //let make_generator: FnOnce(GenerationPreferences) -> G = $make_generator_expr;
            let make_generator = $make_generator_expr;
            //let verify_func: FnOnce(PathBuf) = $verify_func_ident;
            let verify_func = $verify_func_ident;
            match $gen_test_ident(accepts_config, make_generator) {
                Ok(path) => verify_func(path),
                Err(err) => panic!("apivolve generator failed: {}", err),
            };
        }
    };
}

#[macro_export]
macro_rules! testsuite_basic {
    ($accepts_config_expr: expr, $make_generator_expr: expr, $verify_func_ident: ident) => {
        make_gen_test!(
            test_no_versions,
            generate_no_versions,
            $accepts_config_expr,
            $make_generator_expr,
            $verify_func_ident
        );
        make_gen_test!(
            test_core_features,
            generate_core_features,
            $accepts_config_expr,
            $make_generator_expr,
            $verify_func_ident
        );
        make_gen_test!(
            test_with_pending,
            generate_with_pending,
            $accepts_config_expr,
            $make_generator_expr,
            $verify_func_ident
        );
        //TODO @mark: more tests
    };
    ($accepts_config_expr: expr, $make_generator_expr: expr) => {
        testsuite_full!(
            $accepts_config_expr: expr,
            $make_generator_expr: expr,
            accept_all
        );
    };
}

#[macro_export]
macro_rules! testsuite_full {
    ($accepts_config_expr: expr, $make_generator_expr: expr, $verify_func_ident: ident) => {
        testsuite_basic!(
            $accepts_config_expr,
            $make_generator_expr,
            $verify_func_ident
        );
        //TODO @mark: more tests
    };
    ($accepts_config_expr: expr, $make_generator_expr: expr) => {
        testsuite_basic!($accepts_config_expr, $make_generator_expr, accept_all);
    };
}

#[rustfmt::skip]
pub use testsuite_basic;
#[rustfmt::skip]
pub use testsuite_full;

pub fn generate_no_versions<G, GenFn>(
    accepts_config: AcceptsConfig,
    make_generator: GenFn,
) -> Result<TempDir , String>
        where G: Generator, GenFn: FnOnce(GenerationPreferences) -> Result<G, String> {
    let out_dir = TempDir::new("apivolve").unwrap();
    let mut gen = make_generator(GenerationPreferences {
        apivolve_version: accepts_config.apivolve_version,
        output_dir: out_dir.path().to_path_buf(),
        extra_args: vec![]
    })?;
    block_on(generator_steps(gen))?;
    Ok(out_dir)
}

pub fn generate_core_features<G: Generator, GenFn: FnOnce(GenerationPreferences) -> Result<G, String>> (
    accepts_config: AcceptsConfig,
    make_generator: GenFn,
) -> Result<TempDir, String> {
    unimplemented!()
}

pub fn generate_with_pending<G: Generator, GenFn: FnOnce(GenerationPreferences) -> Result<G, String>> (
    accepts_config: AcceptsConfig,
    make_generator: GenFn,
) -> Result<TempDir, String> {
    unimplemented!()
}

async fn generator_steps<G: Generator>(gen: G) -> Result<(), String> {
    gen.finalize().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use ::async_trait::async_trait;
    use ::semver::Version;

    use crate::gen1::AcceptsConfig;
    use crate::gen1::connect::layout::GenFeatures;
    use crate::gen1::Evolution;
    use crate::gen1::GenerateInputFormat;
    use crate::gen1::GenerationPreferences;
    use crate::gen1::Generator;
    use crate::gen1::GenResult;

    use super::*;

    struct NoopGenerator();

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

    fn noop_generator_factory(_: GenerationPreferences) -> Result<NoopGenerator, String> {
        Ok(NoopGenerator())
    }

    testsuite_full!(
        AcceptsConfig {
            apivolve_version: Version::new(1, 0, 0),
            features: GenFeatures::default(),
            encoding: GenerateInputFormat::Json,
        },
        noop_generator_factory
    );
}
