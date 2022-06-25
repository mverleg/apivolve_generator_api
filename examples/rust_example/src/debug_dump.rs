use ::apivolve_generator_api::gen1 as gen;
use ::apivolve_generator_api::gen1::GenResult;
use ::apivolve_generator_api::gen1::Version;
use ::apivolve_generator_api::gen1::VersionEvolution;
use ::async_trait::async_trait;
use ::std::env::args;

fn main() {
    //TODO @mark: make features configurable
    env_logger::init();
    assert!(args().skip(1).next().is_none(), "no arguments expected");
    let config = gen::AcceptsConfig {
        apivolve_version: gen::Version::new(0, 1, 0),
        features: gen::GenFeatures::all(),
        encoding: gen::GenerateInputFormat::Json,
    };
    gen::run_generator(config, |prefs| Ok(DebugDumpGenerator::new(prefs)));
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
impl gen::Generator for DebugDumpGenerator {
    async fn generate_pending(&mut self, evolution: VersionEvolution) -> GenResult {
        unimplemented!();  //TODO @mark:
    }

    async fn generate_version(&mut self, version: Version, evolution: VersionEvolution) -> GenResult {
        unimplemented!();  //TODO @mark:
    }

    async fn finalize(self) -> GenResult {
        unimplemented!();  //TODO @mark:
    }
}

#[cfg(test)]
testsuite_full!();
