use ::std::env::args;
use ::apivolve_generator_api::gen1;

fn main() {
    env_logger::init();
    assert!(args().skip(1).next().is_none(), "no arguments expected");
    let config = gen1::AcceptsConfig {};
    gen1::run_generator(DebugDumpGenerator());
}

#[derive(Debug)]
struct DebugDumpGenerator();

impl gen1::Generator for DebugDumpGenerator {}
