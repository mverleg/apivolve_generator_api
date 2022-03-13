use std::io::{stdout, Write};

use ::apivolve_generator_api::gen::gen1::GenerateConfig;
use ::apivolve_generator_api::gen::gen1::GenerateInputFormat;
use ::apivolve_generator_api::gen::gen1::GenerateInputLayout;
use ::apivolve_generator_api::gen::gen1::Version;
use ::log::debug;

fn main() {
    let conf = GenerateConfig::new(Version::new(0, 1, 0), GenerateInputLayout::Steps, GenerateInputFormat::Json);
    let json = serde_json::to_string(&conf).unwrap();
    debug!("config: {}", json);
    stdout().lock().write_all(json.as_bytes()).unwrap();
}
