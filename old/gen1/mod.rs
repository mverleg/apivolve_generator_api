//! The generating executable should emit [GenerateConfig] as json on a single line of stdout.
//! Then Apivolve CLI will send [GenerateSteps] in the desired format on its stdin.

pub use self::config::GenerateConfig;
pub use self::config::GenerateInputFormat;
pub use self::config::GenerateInputLayout;
pub use self::config::Version;
pub use self::run::run;
pub use self::run::run_with_steps;
pub use self::run::GenErr;
pub use self::steps::GenerateSteps;

mod config;
mod run;
mod steps;
