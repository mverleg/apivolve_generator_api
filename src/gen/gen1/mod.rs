//! The generating executable should emit [GenerateConfig] as json on a single line of stdout.
//! Then Apivolve CLI will send [GenerateChangesInput] in the desired format on its stdin.

mod config;
pub use config::GenerateConfig;
