use ::apivolve_generator_api::gen1 as gen;
use ::apivolve_generator_api::gen1::GenResult;
use ::apivolve_generator_api::gen1::Version;
use ::apivolve_generator_api::gen1::VersionEvolution;
use ::std::env::args;

fn main() {
    //TODO @mark: make features configurable
    env_logger::init();
    assert!(args().skip(1).next().is_none(), "no arguments expected");
    let config = gen::AcceptedFormat {
        apivolve_version: gen::Version::new(0, 1, 0),
        encoding: gen::GenerateInputFormat::Json,
    };
    let api = DebugDumpApi::new();
    gen::run_generator();
}

#[derive(Debug)]
struct DebugDumpApi {}

impl gen::GeneratorApi<DebugDumpGenerator, (), ()> for DebugDumpApi {
    fn accepts(&mut self) -> Result<(AcceptedFormat, ()), String> {}

    fn features(&mut self, user_pref: UserPreferences, data: ()) -> Result<(FunctionalityRequest, ()), String> {}

    fn make_generator(self, data: ()) -> Result<G, String> {}
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

impl gen::Generator for DebugDumpGenerator {
    fn generate_pending(&mut self, evolution: VersionEvolution) -> GenResult {
        let json = serde_json::to_string_pretty(&evolution).unwrap();
        println!("{}", json);
        self.version_count += 1;
        GenResult::Ok
    }

    fn generate_version(&mut self, version: Version, evolution: VersionEvolution) -> GenResult {
        println!("// version = {}", version);
        let json = serde_json::to_string_pretty(&evolution).unwrap();
        println!("{}", json);
        self.version_count += 1;
        GenResult::Ok
    }

    fn finalize(self) -> Result<(), gen::ErrMsg> {
        if self.version_count > 0 {
            println!("// printed {} versions", self.version_count);
            Ok(())
        } else {
            Err("no versions found".to_owned())
        }
    }
}

// #[cfg(test)]
// testsuite_full!();
