use ::std::env::args;
use ::apivolve_generator_api::gen1;

fn main() {
    env_logger::init();
    assert!(args().skip(1).next().is_none(), "no arguments expected");
    let config = gen1::AcceptsConfig {
        apivolve_version: gen1::Version::new(0, 1, 0),
        data_structure: gen1::GenerateInputLayout::Steps,
        encoding: gen1::GenerateInputFormat::Json,
    };
    gen1::run_generator(config);
}

#[derive(Debug)]
//TODO @mark: unused?
struct DebugDumpGenerator();

impl gen1::Generator for DebugDumpGenerator {}
