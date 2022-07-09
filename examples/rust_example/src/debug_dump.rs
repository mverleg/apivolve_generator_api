use ::apivolve_generator_api::gen1 as gen;
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

/// This is the starting point for making a generator. It handles the exchanging of
/// preferences and eventually builds the Generator. It must implement `GeneratorApi`.
#[derive(Debug)]
struct DebugDumpApi {}

impl gen::GeneratorApi<DebugDumpGenerator, (), ()> for DebugDumpApi {
    fn accepts(&mut self) -> Result<(gen::AcceptedFormat, ()), String> {}

    fn features(&mut self, user_pref: gen::UserPreferences, data: ()) -> Result<(gen::FunctionalityRequest, ()), String> {}

    fn make_generator(self, data: ()) -> Result<DebugDumpGenerator, String> {}
}

/// This does the actual code generation based on Apivolve evolutions. The object is
/// created by `DebugDumpApi` and must implement `Generator`.
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
    fn generate_pending(&mut self, evolution: VersionEvolution) -> gen::GenResult {
        let json = serde_json::to_string_pretty(&evolution).unwrap();
        println!("{}", json);
        self.version_count += 1;
        gen::GenResult::Ok
    }

    fn generate_version(&mut self, version: Version, evolution: VersionEvolution) -> gen::GenResult {
        println!("// version = {}", version);
        let json = serde_json::to_string_pretty(&evolution).unwrap();
        println!("{}", json);
        self.version_count += 1;
        gen::GenResult::Ok
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
