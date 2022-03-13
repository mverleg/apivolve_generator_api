//! The generating executable should emit [GenerateConfig] as json on a single line of stdout.
//! Then Apivolve CLI will send [GenerateSteps] in the desired format on its stdin.

pub use config::GenerateConfig;
pub use config::GenerateInputFormat;
pub use config::GenerateInputLayout;
pub use config::Version;
pub use run::GenErr;
pub use run::run;
pub use run::run_with_steps;
pub use steps::GenerateSteps;

mod config;
mod steps;
mod run;
