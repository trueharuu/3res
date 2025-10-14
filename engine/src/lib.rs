// #![warn(clippy::pedantic)]
#![warn(clippy::perf, clippy::correctness, clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::struct_excessive_bools,
    clippy::type_complexity,
    clippy::missing_panics_doc
)]
#![deny(unused_qualifications)]

pub mod board;
pub mod common;
pub mod environment;
pub mod file;
pub mod input;
pub mod pc;
pub mod piece;
pub mod randomizer;
pub mod ren;
pub mod repl;

