use ::apivolve_generator_api::gen1::run_with_steps;
use ::apivolve_generator_api::gen1::GenerateSteps;

fn main() {
    env_logger::init();
    run_with_steps(debug_dump).unwrap();
}

fn debug_dump(steps: &GenerateSteps) {
    eprintln!("start debug_dump code"); //TODO @mark: TEMPORARY! REMOVE THIS!
    let step_json = serde_json::to_string_pretty(&steps)
        .expect("failed to convert apivolve evolution steps to json");
    eprintln!("generated debug_dump"); //TODO @mark: TEMPORARY! REMOVE THIS!
    println!("{}", step_json);
    eprintln!("wrote debug_dump output"); //TODO @mark: TEMPORARY! REMOVE THIS!
}
