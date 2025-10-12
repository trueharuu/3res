// #![warn(clippy::pedantic)]
#![allow(clippy::perf, clippy::correctness)]
#![allow(
    clippy::missing_errors_doc,
    clippy::struct_excessive_bools,
    clippy::type_complexity
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

