pub use ::semver::Version;

pub use self::connect::accepts::AcceptsConfig;
pub use self::connect::format::GenerateInputFormat;
pub use self::connect::genpref::GenerationPreferences;
pub use self::connect::layout::GenerateInputLayout;
pub use self::evolution::Evolution;

mod connect;
mod evolution;

pub type ErrMsg = String;
pub type GenResult = Result<(), ErrMsg>;

pub trait Generator {
    async fn generate_version(&mut self, version: Version, evolution: Evolution) -> GenResult;

    async fn generate_pending(&mut self, evolution: Evolution) -> GenResult;

    async fn finalize(&mut self) -> GenResult;
}

/// Run the generator, handling the communication with Apivolve.
pub fn run_generator<G: Generator>(
        accepts_config: AcceptsConfig,
        make_generator: impl FnOnce(GenerationPreferences) -> G) {
    //TODO @mark: auth
    //TODO @mark: send `accepts_config`
    let generator_preferences: GenerationPreferences = read_gen_preferences();
    let generator: G = make_generator(generator_preferences);
    if let Err(err) = generate_until_first_err(generator).await {
        panic!("{}", err);  //TODO @mark:
    }
}

fn generate_until_first_err(generator: G) -> GenResult {
    while let Some(evolution) = unimplemented!() {
        let res = generator.generate_version(evolution).await?;
    }
    let res = generator.generate_pending(evolution).await;
    let res = generator.finalize(evolution).await;
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
