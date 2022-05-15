use ::tempdir::TempDir;

pub use testsuite_basic;
pub use testsuite_full;

use crate::gen1::GenResult;

use super::generate::generate_no_versions;
use super::generate::make_gen_test;

#[allow(dead_code)]  // used in macro
fn noop_generator() {}

#[allow(dead_code)]  // used in macro
fn accept_all(_output_dir: TempDir) -> GenResult {
    Ok(())
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

#[cfg(test)]
mod tests {
    use ::async_trait::async_trait;
    use ::semver::Version;

    use crate::gen1::{AcceptsConfig, Evolution, GenerateInputFormat, GenerationPreferences, Generator};
    use crate::gen1::connect::layout::GenFeatures;

    use super::*;

    struct NoopGenerator();

    #[async_trait]
    impl Generator for NoopGenerator {
        async fn generate_pending(&mut self, _evolution: Evolution) -> GenResult {
            Ok(())
        }

        async fn generate_version(&mut self, _version: Version, _evolution: Evolution) -> GenResult {
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
