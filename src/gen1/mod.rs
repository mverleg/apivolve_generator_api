pub use ::semver::Version;

pub use self::connect::accepts::AcceptsConfig;
pub use self::connect::format::GenerateInputFormat;
pub use self::connect::genpref::GenerationPreferences;
pub use self::connect::layout::GenFeature;
pub use self::connect::layout::GenFeatures;
pub use self::evolution::VersionEvolution;
pub use self::run::ErrMsg;
pub use self::run::Generator;
pub use self::run::GenResult;
pub use self::run::run_generator;

mod connect;
mod data;
mod evolution;
mod run;
//TODO @mark:
// mod tests;
