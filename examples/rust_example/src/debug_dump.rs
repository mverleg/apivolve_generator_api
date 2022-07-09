use ::apivolve_generator_api::gen1 as gen;
use ::std::env::args;

fn main() {
    //TODO @mark: make features configurable
    env_logger::init();
    assert!(args().skip(1).next().is_none(), "no arguments expected");
    let api = DebugDumpProtocol::new();
    gen::run_generator(api);
}

/// This is the starting point for making a generator. It handles the exchanging of
/// preferences and eventually builds the Generator. It must implement `GeneratorApi`.
#[derive(Debug)]
struct DebugDumpProtocol {}

impl DebugDumpProtocol {
    pub fn new() -> Self {
        DebugDumpProtocol {}
    }
}

impl gen::GeneratorProtocol<DebugDumpGenerator, (), gen::UserPreferences> for DebugDumpProtocol {
    fn accepts(&mut self) -> Result<(gen::AcceptedFormat, ()), String> {
        Ok(gen::AcceptedFormat {
            apivolve_version: gen::Version::new(0, 1, 0),
            encoding: gen::GenerateInputFormat::Json,
        })
    }

    fn features(&mut self, user_pref: gen::UserPreferences, data: ()) -> Result<(gen::FunctionalityRequest, gen::UserPreferences), String> {

    }

    fn make_generator(self, data: gen::UserPreferences) -> Result<DebugDumpGenerator, String> {

    }
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
    fn generate_pending(&mut self, evolution: gen::VersionEvolution) -> gen::GenResult {
        let json = serde_json::to_string_pretty(&evolution).unwrap();
        println!("{}", json);
        self.version_count += 1;
        gen::GenResult::Ok
    }

    fn generate_version(&mut self, version: gen::Version, evolution: gen::VersionEvolution) -> gen::GenResult {
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
