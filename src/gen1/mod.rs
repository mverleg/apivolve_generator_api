pub use ::semver::Version;

pub use self::connect::accepts::AcceptsConfig;
pub use self::connect::format::GenerateInputFormat;
pub use self::connect::genpref::GenerationPreferences;
pub use self::connect::layout::GenerateInputLayout;

mod connect;

pub trait Generator {

}

/// Run the generator, handling the communication with Apivolve.
pub fn run_generator<G: Generator>(
        accepts_config: AcceptsConfig,
        make_generator: impl FnOnce(GenerationPreferences) -> G) {
    //TODO @mark: auth
    //TODO @mark: send `accepts_config`
    let generator_preferences: GenerationPreferences = unimplemented!();
    let generator: G = make_generator(generator_preferences);


    // eprintln!("start debug_dump code"); //TODO @mark: TEMPORARY! REMOVE THIS!
    // let step_json = serde_json::to_string_pretty(&steps)
    //     .expect("failed to convert apivolve evolution steps to json");
    // eprintln!("generated debug_dump"); //TODO @mark: TEMPORARY! REMOVE THIS!
    // println!("{}", step_json);
    // eprintln!("wrote debug_dump output"); //TODO @mark: TEMPORARY! REMOVE THIS!
}
