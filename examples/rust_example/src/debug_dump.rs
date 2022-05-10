use ::std::env::args;
use ::apivolve_generator_api::gen1 as gen;

fn main() {
    env_logger::init();
    assert!(args().skip(1).next().is_none(), "no arguments expected");
    let config = gen::AcceptsConfig {
        apivolve_version: gen::Version::new(0, 1, 0),
        data_structure: gen::GenerateInputLayout::Steps,
        encoding: gen::GenerateInputFormat::Json,
    };
    gen::run_generator(config, |prefs| DebugDumpGenerator::new(prefs));
}

#[derive(Debug)]
struct DebugDumpGenerator {
    config: gen::GenerationPreferences,
}

impl DebugDumpGenerator {
    pub fn new(config: gen::GenerationPreferences) -> Self {
        DebugDumpGenerator {
            config
        }
    }
}

impl gen::Generator for DebugDumpGenerator {}
