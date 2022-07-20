#![allow(clippy::module_inception)] //TODO @mark: TEMPORARY! REMOVE THIS!

extern crate core;

pub use self::util::Identifier;
pub use self::util::Range;

mod driver;
pub mod gen1;
mod util;
