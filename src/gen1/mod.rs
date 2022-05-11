pub use ::semver::Version;

pub use self::connect::accepts::AcceptsConfig;
pub use self::connect::format::GenerateInputFormat;
pub use self::connect::genpref::GenerationPreferences;
pub use self::connect::layout::GenerateInputLayout;
pub use self::evolution::Evolution;
pub use self::run::ErrMsg;
pub use self::run::Generator;
pub use self::run::GenResult;
pub use self::run::run_generator;

mod connect;
mod evolution;
mod run;

#[cfg(test)]
mod test_util;

