use ::std::io::{stdout, Write};

use ::log::debug;
use ::log::error;

use crate::gen1::GenerateConfig;
use crate::gen1::GenerateInputFormat;
use crate::gen1::GenerateInputLayout;
use crate::gen1::GenerateSteps;
use crate::gen1::Version;

pub enum GenErr {
    ConfigWriteErr
}

pub fn run<T>(gen: impl FnOnce(&GenerateSteps) -> T) -> Result<T, GenErr> {
    return run_with_steps(gen);
}

pub fn run_with_steps<T>(gen: impl FnOnce(&GenerateSteps) -> T) -> Result<T, GenErr> {
    let conf = GenerateConfig::new(Version::new(0, 1, 0), GenerateInputLayout::Steps, GenerateInputFormat::Json);
    let conf_json = serde_json::to_string(&conf).unwrap();
    debug!("config: {}", conf_json);
    {
        let mut writer = stdout().lock();
        writer.write_all(conf_json.as_bytes())
            .map_err(|err| {
                error!("Failed to write configuration to stdout, err: {}", err);
                GenErr::ConfigWriteErr
            })?;
        writer.write_all(b"\n")
            .map_err(|err| {
                error!("Failed to end configuration on stdout, err: {}", err);
                GenErr::ConfigWriteErr
            })?;
    }
    let steps = GenerateSteps::new();  //TODO @mark:
    let res = gen(&steps);
    Ok(res)
}
