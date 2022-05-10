pub use self::connect::accepts::AcceptsConfig;
pub use self::connect::format::GenerateInputFormat;
pub use self::connect::layout::GenerateInputLayout;
pub use ::semver::Version;

mod connect;

pub trait Generator {

}

/// Run the generator, handling the communication with Apivolve.
pub fn run_generator(accepts_config: AcceptsConfig) {
    //TODO @mark: auth

    // eprintln!("start debug_dump code"); //TODO @mark: TEMPORARY! REMOVE THIS!
    // let step_json = serde_json::to_string_pretty(&steps)
    //     .expect("failed to convert apivolve evolution steps to json");
    // eprintln!("generated debug_dump"); //TODO @mark: TEMPORARY! REMOVE THIS!
    // println!("{}", step_json);
    // eprintln!("wrote debug_dump output"); //TODO @mark: TEMPORARY! REMOVE THIS!
}
