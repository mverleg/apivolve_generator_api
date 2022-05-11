use ::futures::executor::block_on;
use ::semver::Version;

use crate::gen1::{AcceptsConfig, GenerationPreferences};
use crate::gen1::run::gen_trait::{Generator, GenResult};

/// Run the generator, handling the communication with Apivolve.
pub fn run_generator<G: Generator>(
    accepts_config: AcceptsConfig,
    make_generator: impl FnOnce(GenerationPreferences) -> G) {
    //TODO @mark: auth
    //TODO @mark: send `accepts_config`
    let generator_preferences: GenerationPreferences = read_gen_preferences();
    let generator: G = make_generator(generator_preferences);
    //TODO @mark: use a better async runtime
    if let Err(err) = block_on(generate_until_first_err(generator)) {
        panic!("{}", err);  //TODO @mark:
    }
}

async fn generate_until_first_err(mut generator: impl Generator) -> GenResult {
    if let Some(evolution) = None {
        generator.generate_pending(evolution).await?;
    }
    while let Some((version, evolution)) = None {
        generator.generate_version(version, evolution).await?;
    };
    generator.finalize().await?;
    Ok(())
}

fn read_gen_preferences() -> GenerationPreferences {
    //TODO @mark: TEMPORARY! REMOVE THIS!
    if 1==1 { unimplemented!(); }
    GenerationPreferences {
        apivolve_version: Version::new(0, 0, 1),
        output_dir: Default::default(),
        extra_args: vec![]
    }
}
