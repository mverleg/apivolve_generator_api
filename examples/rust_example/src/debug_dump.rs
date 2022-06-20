use ::apivolve_generator_api::gen1 as gen;
use ::async_trait::async_trait;
use ::std::env::args;

fn main() {
    env_logger::init();
    assert!(args().skip(1).next().is_none(), "no arguments expected");
    let config = gen::AcceptsConfig {
        apivolve_version: gen::Version::new(0, 1, 0),
        features: gen::GenFeatures::new(),
        encoding: gen::GenerateInputFormat::Json,
    };
    gen::run_generator(config, DebugDumpGenerator::new);
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

#[async_trait]
impl gen::Generator for DebugDumpGenerator {}

#[cfg(test)]
testsuite_full!();
