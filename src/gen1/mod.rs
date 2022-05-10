
mod connect;

pub use self::connect::accepts::AcceptsConfig;

pub trait Generator {

}

/// Run the generator, handling the communication with Apivolve.
pub fn run_generator(generator: impl Generator) {
    // eprintln!("start debug_dump code"); //TODO @mark: TEMPORARY! REMOVE THIS!
    // let step_json = serde_json::to_string_pretty(&steps)
    //     .expect("failed to convert apivolve evolution steps to json");
    // eprintln!("generated debug_dump"); //TODO @mark: TEMPORARY! REMOVE THIS!
    // println!("{}", step_json);
    // eprintln!("wrote debug_dump output"); //TODO @mark: TEMPORARY! REMOVE THIS!
}
