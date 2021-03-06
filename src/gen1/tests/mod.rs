//! Provide tests to see if a generator handles some pre-defined cases.
//!
//! Since Apivolve can make almost no assumptions about the what generators do,
//! this only provides the data and checks for [Result] and [panic!].
//! It is up to the generator's author to test that the output makes sense.

mod generate;
mod suite;
mod testdata;

pub use self::suite::testsuite_basic;
pub use self::suite::testsuite_full;
pub use self::testdata::*;
