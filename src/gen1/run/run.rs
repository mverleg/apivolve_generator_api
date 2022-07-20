//TODO @mark: put on hold in the middle of dev; does not work at all

use ::log::error;
use ::semver::Version;
use ::smallvec::smallvec;

use crate::gen1::{ErrMsg, GenResult, Generator, GeneratorProtocol, UserPreferences};

/// Run the generator, handling the communication with Apivolve.
pub fn run_generator<T, U, G: Generator>(
    mut protocol: impl GeneratorProtocol<G, T, U>,
) -> Result<(), ErrMsg> {
    //accept_single_connection().unwrap();
    //TODO @mark: timeout
    let (accepts, transfer1) = protocol
        .accepts()
        .map_err(|err| format!("generator failed to specify the accepted format: {}", &err))?;
    let _accepts_json =
        serde_json::to_string(&accepts).expect("could not convert AcceptedFormat to json");

    let user_prefs = UserPreferences {
        //TODO @mark:
        apivolve_version: Version::new(1, 0, 0),
        output_dir: Default::default(),
        extra_args: vec![],
        requested_parties: Default::default(),
    };
    let (_features, _transfer2) = match protocol.features(user_prefs, transfer1) {
        Ok(res) => res,
        Err(err) => {
            error!("generator failed to send requested features: {}", &err);
            return Err(err);
        }
    };
    // gen_api.make_generator();
    // //TODO @mark: send `accepts_config`
    // let generator_preferences: UserPreferences = read_gen_preferences();
    // let generator: G = make_generator(generator_preferences)?;
    // //TODO @mark: use a better async runtime
    // generate_until_first_err(generator)
    unimplemented!() //TODO @mark: TEMPORARY! REMOVE THIS!
}

#[allow(unused)] //TODO @mark:
fn generate_until_first_err(mut generator: impl Generator) -> Result<(), ErrMsg> {
    let mut finished = false;
    if let Some(evolution) = None {
        match generator.generate_pending(evolution) {
            GenResult::Ok => {}
            GenResult::FinishEarly => finished = true,
            GenResult::Error(err) => return Err(err),
        }
    }
    if !finished {
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

#[allow(unused)] //TODO @mark:
fn read_gen_preferences() -> UserPreferences {
    UserPreferences {
        apivolve_version: Version::new(0, 0, 1),
        output_dir: Default::default(),
        extra_args: vec!["--features=documentation,examples,parser,validator".to_owned()],
        requested_parties: smallvec![],
    }
}
