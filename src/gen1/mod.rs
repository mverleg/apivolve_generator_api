pub use ::semver::Version;

pub use self::test_util::testsuite_basic;
pub use self::test_util::testsuite_full;

pub use self::connect::accepts::AcceptsConfig;
pub use self::connect::format::GenerateInputFormat;
pub use self::connect::genpref::GenerationPreferences;
pub use self::connect::layout::GenFeature;
pub use self::evolution::Evolution;
pub use self::run::ErrMsg;
pub use self::run::Generator;
pub use self::run::GenResult;
pub use self::run::run_generator;

mod connect;
mod evolution;
mod run;

mod test_util;
