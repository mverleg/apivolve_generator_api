use ::std::io::{stdout, Write};

use ::apivolve_generator_api::gen1::run_with_steps;

fn main() {
    run_with_steps(|steps| {
        let step_json = serde_json::to_string_pretty(&steps)
            .expect("failed to convert apivolve evolution steps to json");
        println!("{}", step_json);
    }).unwrap();
}
