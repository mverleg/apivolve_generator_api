use ::futures::executor::block_on;
use ::semver::Version;
use ::tempdir::TempDir;

use crate::gen1::{AcceptsConfig, Evolution, GenerationPreferences, Generator, GenResult};

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

            let customization: AcceptsCustomizations = $accepts_config_expr;
            let accepts_config = customization.to_accepts(
                Version::new(0, 1, 0),  //TODO @mark
                GenerateInputFormat::Json,);
            let make_generator = make_generator_expr_type_must_satisfy_this_signature($make_generator_expr);
            let verify_func = verify_func_ident_type_must_satisfy_this_signature($verify_func_ident);
            match $gen_test_ident(accepts_config, make_generator) {
                Ok(path) => if let Err(err) = verify_func(path) {
                    panic!("apivolve generator output was not valid: {}", err)
                },
                Err(err) => panic!("apivolve generator failed: {}", err),
            };
        }
    };
}

pub(crate) use make_gen_test;

pub(crate) fn test_with_data<G, GenFn>(
    accepts_config: AcceptsConfig,
    make_generator: GenFn,
    pending: Option<Evolution>,
    versions: Vec<(Version, Evolution)>,
) -> Result<TempDir , String>
    where G: Generator, GenFn: FnOnce(GenerationPreferences) -> Result<G, String> {
    let out_dir = TempDir::new("apivolve").unwrap();
    let mut gen = make_generator(GenerationPreferences {
        apivolve_version: accepts_config.apivolve_version,
        output_dir: out_dir.path().to_path_buf(),
        extra_args: vec![]
    })?;
    block_on(generator_steps(gen, pending, versions))?;
    Ok(out_dir)
}

async fn generator_steps<G: Generator>(mut gen: G, pending: Option<Evolution>, versions: Vec<(Version, Evolution)>) -> GenResult {
    if let Some(pending) = pending {
        gen.generate_pending(pending).await?;
    }
    for (version, evolution) in versions {
        gen.generate_version(version, evolution).await?;
    }
    gen.finalize().await?;
    Ok(())
}
