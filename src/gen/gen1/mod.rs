//! The generating executable should emit [GenerateConfig] as json on a single line of stdout.
//! Then Apivolve CLI will send [GenerateChangesInput] in the desired format on its stdin.

pub use config::GenerateConfig;
pub use config::GenerateInputFormat;
pub use config::GenerateInputLayout;
pub use config::Version;

mod config;
