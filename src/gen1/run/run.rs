use ::futures::executor::block_on;
use ::semver::Version;
use smallvec::smallvec;

use crate::gen1::run::gen_trait::{GenResult, Generator};
use crate::gen1::{AcceptsConfig, GenerationPreferences};

/// Run the generator, handling the communication with Apivolve.
pub fn run_generator<G: Generator>(
    _accepts_config: AcceptsConfig,
    make_generator: impl FnOnce(GenerationPreferences) -> Result<G, String>,
) -> GenResult {
    //TODO @mark: auth
    //TODO @mark: send `accepts_config`
    let generator_preferences: GenerationPreferences = read_gen_preferences();
    let generator: G = make_generator(generator_preferences)?;
    //TODO @mark: use a better async runtime
    block_on(generate_until_first_err(generator))
}

async fn generate_until_first_err(mut generator: impl Generator) -> GenResult {
    if let Some(evolution) = None {
        generator.generate_pending(evolution).await?;
    }
    while let Some((version, evolution)) = None {
        generator.generate_version(version, evolution).await?;
    }
    generator.finalize().await?;
    Ok(())
}

fn read_gen_preferences() -> GenerationPreferences {
    GenerationPreferences {
        apivolve_version: Version::new(0, 0, 1),
        output_dir: Default::default(),
        extra_args: vec!["--features=documentation,examples,parser,validator".to_owned()],
        requested_parties: smallvec![],
    }
}
