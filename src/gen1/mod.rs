use ::async_trait::async_trait;
use ::futures::executor::block_on;
pub use ::semver::Version;

pub use self::connect::accepts::AcceptsConfig;
pub use self::connect::format::GenerateInputFormat;
pub use self::connect::genpref::GenerationPreferences;
pub use self::connect::layout::GenerateInputLayout;
pub use self::evolution::Evolution;
pub use self::run::ErrMsg;
pub use self::run::Generator;
pub use self::run::GeneratorConfig;
pub use self::run::GeneratorError;

mod connect;
mod evolution;
mod run;

