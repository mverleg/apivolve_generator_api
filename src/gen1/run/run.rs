use ::semver::Version;
use ::smallvec::smallvec;

use crate::gen1::{ErrMsg, GeneratorProtocol, UserPreferences};
use crate::gen1::run::gen_trait::{Generator, GenResult};

/// Run the generator, handling the communication with Apivolve.
pub fn run_generator<T, U, G: Generator>(
    gen_api: impl GeneratorProtocol<T, U, G>,
) -> Result<(), ErrMsg> {
    //TODO @mark: auth
    //TODO @mark: send `accepts_config`
    let generator_preferences: UserPreferences = read_gen_preferences();
    let generator: G = make_generator(generator_preferences)?;
    //TODO @mark: use a better async runtime
    generate_until_first_err(generator)
}

fn generate_until_first_err(mut generator: impl Generator) -> Result<(), ErrMsg> {
    let mut finished = false;
    if let Some(evolution) = None {
        match generator.generate_pending(evolution) {
            GenResult::Ok => {},
            GenResult::FinishEarly => finished = true,
            GenResult::Error(err) => return Err(err),
        }
    }
    if ! finished {
        while let Some((version, evolution)) = None {
            match generator.generate_version(version, evolution) {
                GenResult::Ok => {}
                GenResult::FinishEarly => break,
                GenResult::Error(err) => return Err(err),
            }
        }
    }
    generator.finalize()?;
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
