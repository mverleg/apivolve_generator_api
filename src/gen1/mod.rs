pub use ::semver::Version;

pub use run::test_util::testsuite_basic;
pub use run::test_util::testsuite_full;

pub use self::connect::accepts::AcceptsConfig;
pub use self::connect::format::GenerateInputFormat;
pub use self::connect::genpref::GenerationPreferences;
pub use self::connect::layout::GenFeature;
pub use self::evolution::Evolution;
pub use self::run::run_generator;
pub use self::run::ErrMsg;
pub use self::run::GenResult;
pub use self::run::Generator;

mod connect;
mod evolution;
mod run;

