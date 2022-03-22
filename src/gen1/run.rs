use ::std::io::{stdout, Write};
use ::std::io::{Read, stdin};
use ::std::time::Instant;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use ::log::debug;
use ::log::error;
use ::log::trace;

use crate::gen1::GenerateConfig;
use crate::gen1::GenerateInputFormat;
use crate::gen1::GenerateInputLayout;
use crate::gen1::GenerateSteps;
use crate::gen1::Version;

#[derive(Debug)]
pub enum GenErr {
    ConfigWriteErr,
    InputReadErr,
    InputParseErr,
}

pub fn run<T>(gen: impl FnOnce(&GenerateSteps) -> T) -> Result<T, GenErr> {
    return run_with_steps(gen);
}

pub fn run_with_steps<T>(gen: impl FnOnce(&GenerateSteps) -> T) -> Result<T, GenErr> {
    let timer = Instant::now();
    let monitor = start_slowness_monitor();
    send_config()?;
    let steps = recv_steps()?;
    monitor.ready();
    debug!("got evolution steps in {} ms", timer.elapsed().as_millis());
    let timer = Instant::now();
    let res = gen(&steps);
    debug!("generation took {} ms (from receiving steps)", timer.elapsed().as_millis());
    Ok(res)
}

#[derive(Debug)]
struct ReadyNotifier {
    is_ready: AtomicBool,
}

impl ReadyNotifier {
    fn new() -> Self {
        ReadyNotifier {
            is_ready: AtomicBool::new(false),
        }
    }

    fn ready(&self) {
        self.is_ready.store(true, Ordering::Release);
    }
}

#[must_use]
fn start_slowness_monitor() -> Arc<ReadyNotifier> {
    let notifier = Arc::new(ReadyNotifier::new());
    let notifier_for_thread = notifier.clone();
    thread::spawn(move || {
        sleep(Duration::from_secs(5));
        if !notifier_for_thread.is_ready.load(Ordering::Acquire) {
            eprintln!("still waiting for apivolve to supply the evolution data...");
        }
    });
    notifier
}

fn recv_steps() -> Result<GenerateSteps, GenErr> {
    debug!("reading evolution steps response");
    let steps = {
        let mut reader = stdin().lock();
        trace!("acquired evolution steps read lock");
        let mut steps_json = String::new();
        let len = reader.read_to_string(&mut steps_json)
            .map_err(|err| {
                error!("Failed to read steps input from stdin, err: {}", err);
                GenErr::InputReadErr
            })?;
        if len == 0 {
            error!("Steps input from stdin was empty; either no input was sent, or read failed");
            return Err(GenErr::InputReadErr);
        }
        debug!("read {} byte string steps", steps_json.len());
        serde_json::from_str::<GenerateSteps>(&steps_json)
            .map_err(|err| {
                error!("Failed to parse steps input from stdin, err: {}", err);
                GenErr::InputParseErr
            })?
    };
    trace!("parsed evolution steps, starting provided generator function");
    Ok(steps)
}

fn send_config() -> Result<(), GenErr> {
    let conf = GenerateConfig::new(Version::new(0, 1, 0), GenerateInputLayout::Steps, GenerateInputFormat::Json);
    let conf_json = serde_json::to_string(&conf).unwrap();
    debug!("config: {}", conf_json);
    let mut writer = stdout().lock();
    trace!("acquired config write lock");
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
    Ok(())
}
