use std::env::args;

fn main() {
    env_logger::init();
    assert!(args().skip(1).next().is_none(), "no arguments expected");

}
