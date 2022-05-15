use ::futures::executor::block_on;
use ::tempdir::TempDir;

use crate::gen1::{AcceptsConfig, GenerationPreferences, Generator, GenResult};

#[allow(unused_macros)] // it is not unused, don't know why it's not being detected
macro_rules! make_gen_test {
    ($test_name: ident, $gen_test_ident: ident, $accepts_config_expr: expr, $make_generator_expr: expr, $verify_func_ident: ident) => {
        #[test]
        fn $test_name() {
            #[inline]
            fn make_generator_expr_type_must_satisfy_this_signature<G, F>(make_generator: F) -> F
                    where G: Generator, F: FnOnce(GenerationPreferences) -> Result<G, String> {
                make_generator
            }

            #[inline]
            fn verify_func_ident_type_must_satisfy_this_signature<F>(verify_func: F) -> F
                    where F: FnOnce(TempDir) -> GenResult {
                verify_func
            }

            let accepts_config: AcceptsConfig = $accepts_config_expr;
            let make_generator = make_generator_expr_type_must_satisfy_this_signature($make_generator_expr);
            let verify_func = verify_func_ident_type_must_satisfy_this_signature($verify_func_ident);
            match $gen_test_ident(accepts_config, make_generator) {
                Ok(path) => verify_func(path),
                Err(err) => panic!("apivolve generator failed: {}", err),
            };
        }
    };
}

pub(crate) use make_gen_test;

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

async fn generator_steps<G: Generator>(gen: G) -> GenResult {
    gen.finalize().await?;
    Ok(())
}
