use ::futures::executor::block_on;
use ::tempdir::TempDir;

use crate::gen1::{AcceptsConfig, GenerationPreferences, Generator, GenResult};

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
