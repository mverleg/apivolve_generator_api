use ::futures::executor::block_on;
use ::semver::Version;
use ::smallvec::smallvec;

use crate::gen1::{AcceptedFormat, UserPreferences};
use crate::gen1::run::gen_trait::{Generator, GenResult};

/// Run the generator, handling the communication with Apivolve.
pub fn run_generator<G: Generator>(
    _accepts_config: AcceptedFormat,
    make_generator: impl FnOnce(UserPreferences) -> Result<G, String>,
) -> GenResult {
    //TODO @mark: auth
    //TODO @mark: send `accepts_config`
    let generator_preferences: UserPreferences = read_gen_preferences();
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

fn read_gen_preferences() -> UserPreferences {
    UserPreferences {
        apivolve_version: Version::new(0, 0, 1),
        output_dir: Default::default(),
        extra_args: vec!["--features=documentation,examples,parser,validator".to_owned()],
        requested_parties: smallvec![],
    }
}
