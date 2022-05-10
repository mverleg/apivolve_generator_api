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
    /// Will be called for each version, in ascending order, before other methods are called.
    async fn generate_version(&mut self, version: Version, evolution: Evolution) -> GenResult;

    /// Will be called once after all the versions only if there are any pending changes.
    async fn generate_pending(&mut self, evolution: Evolution) -> GenResult;

    /// Will be called exactly once at the end, if no prior steps have failed.
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
    //TODO @mark: use a better async runtime
    if let Err(err) = block_on(generate_until_first_err(generator)) {
        panic!("{}", err);  //TODO @mark:
    }
}

async fn generate_until_first_err(generator: G) -> GenResult {
    while let Some(evolution) = () {
        generator.generate_version(evolution).await?;
    };
    if let Some(evolution) = () {
        generator.generate_pending(evolution).await?;
    }
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
