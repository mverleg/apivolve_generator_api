use ::futures::executor::block_on;
use ::tempdir::TempDir;
use semver::Version;

use crate::gen1::{AcceptsConfig, Evolution, GenerationPreferences, Generator, GenResult};

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
    block_on(generator_steps(gen, None, vec![]))?;
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
