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
    let config = gen::AcceptedFormat {
        apivolve_version: gen::Version::new(0, 1, 0),
        encoding: gen::GenerateInputFormat::Json,
    };
    //TODO @mark
    gen::run_generator(config, |prefs| Ok(DebugDumpGenerator::new(prefs)));
}

#[derive(Debug)]
struct DebugDumpGenerator {
    config: gen::UserPreferences,
    version_count: u32,
}

impl DebugDumpGenerator {
    pub fn new(config: gen::UserPreferences) -> Self {
        DebugDumpGenerator {
            config,
            version_count: 0,
        }
    }
}

#[async_trait]
impl gen::Generator for DebugDumpGenerator {
    async fn generate_pending(&mut self, evolution: VersionEvolution) -> GenResult {
        let json = serde_json::to_string_pretty(&evolution).unwrap();
        println!("{}", json);
        self.version_count += 1;
        GenResult::Ok
    }

    async fn generate_version(&mut self, version: Version, evolution: VersionEvolution) -> GenResult {
        println!("// version = {}", version);
        let json = serde_json::to_string_pretty(&evolution).unwrap();
        println!("{}", json);
        self.version_count += 1;
        GenResult::Ok
    }

    async fn finalize(self) -> Result<(), gen::ErrMsg> {
        if self.version_count > 0 {
            println!("// printend {} versions", self.version_count);
            Ok(())
        } else {
            Err("no versions found".to_owned())
        }
    }
}

// #[cfg(test)]
// testsuite_full!();
