use std::env::args;

fn main() {
    assert!(args().skip(1).next().is_none(), "no arguments expected");

}
