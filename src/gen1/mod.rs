pub use ::semver::Version;

pub use self::connect::AcceptedFormat;
pub use self::connect::FunctionalityRequest;
pub use self::connect::GenFeature;
pub use self::connect::GenFeatures;
pub use self::connect::GenerateInputFormat;
pub use self::connect::UserPreferences;
pub use self::evolution::VersionEvolution;
pub use self::run::run_generator;
pub use self::run::ErrMsg;
pub use self::run::GenResult;
pub use self::run::Generator;
pub use self::run::GeneratorProtocol;

mod connect;
mod data;
mod evolution;
mod run;
//TODO @mark:
// mod tests;
