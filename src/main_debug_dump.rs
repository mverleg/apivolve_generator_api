use ::apivolve_generator_api::gen1::GenerateSteps;
use ::apivolve_generator_api::gen1::run_with_steps;

fn main() {
    env_logger::init();
    run_with_steps(debug_dump).unwrap();
}

fn debug_dump(steps: &GenerateSteps) {
    let step_json = serde_json::to_string_pretty(&steps)
        .expect("failed to convert apivolve evolution steps to json");
    println!("{}", step_json);
}
