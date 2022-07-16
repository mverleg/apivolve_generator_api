use ::std::net::TcpListener;
use ::std::thread;

use ::log::debug;
use ::log::error;
use ::semver::Version;
use ::smallvec::smallvec;

use crate::gen1::{AcceptedFormat, ErrMsg, FunctionalityRequest, GeneratorProtocol, UserPreferences};
use crate::gen1::run::gen_trait::{Generator, GenResult};

/// Run the generator, handling the communication with Apivolve.
pub fn run_generator<T, U, G: Generator>(
    mut protocol: impl GeneratorProtocol<G, T, U>,
) -> Result<(), ErrMsg> {
    accept_single_connection();
    //TODO @mark: timeout
    let (accepts, transfer1) = protocol.accepts()
        .map_err(|err| format!("generator failed to specify the accepted format: {}", &err))?;
    let accepts_json = serde_json::to_string(&accepts)
        .expect("could not convert AcceptedFormat to json");

    let (features, transfer2) = match protocol.features(user_prefs, transfer1) {
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
    unimplemented!()  //TODO @mark: TEMPORARY! REMOVE THIS!
}

fn accept_single_connection() -> Result<(), ErrMsg> {
    let address = "127.0.0.1:47400";
    let listener = TcpListener::bind(address)
        .map_err(|err| format!("failed to listen for tcp connection on {}, err {}", address, err))?;
    let mut worker;
    if let Some(connection) = listener.incoming().next() {

    }
    thread::spawn(|| reject_extra_connections(listener));
    for connection in listener.incoming() {
        let stream = connection.map_err(|err| format!("failed to accept the forst connection on {}, err {}", address, err))?;
        worker = thread::spawn(move || stream);
    }
    worker.join().expect("generator thread failed to join");
    Ok(())
}

fn reject_extra_connections(listener: TcpListener) {
    for extra_connection in listener.incoming() {
        debug!("unexpected connection: {:?}", extra_connection);
        error!("received more than one connection! this should not happen, it will be ignored");
    }
}

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

fn read_gen_preferences() -> UserPreferences {
    UserPreferences {
        apivolve_version: Version::new(0, 0, 1),
        output_dir: Default::default(),
        extra_args: vec!["--features=documentation,examples,parser,validator".to_owned()],
        requested_parties: smallvec![],
    }
}
