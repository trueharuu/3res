// #![warn(clippy::pedantic)]
#![allow(clippy::perf, clippy::correctness)]
#![allow(
    clippy::missing_errors_doc,
    clippy::struct_excessive_bools,
    clippy::type_complexity
)]
#![deny(unused_qualifications)]

use std::io;

use crate::{
    file::{corners::Corners, kicks::Kicks, piece::Bag},
    repl::{Repl, State},
};

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

fn main() {
    let k_file = std::fs::read_to_string("data/srsx.kick").unwrap();
    let b_file = std::fs::read_to_string("data/tetromino.piece").unwrap();
    let c_file = std::fs::read_to_string("data/handheld.corners").unwrap();

    let kicks: Kicks = k_file.parse().unwrap();
    let bag: Bag = b_file.parse().unwrap();
    let corners: Corners = c_file.parse().unwrap();

    let s = State {
        kicks,
        bag,
        corners,
    };

    let repl = Repl::new(io::stdin(), io::stdout(), s);
    let handle = repl.spawn();
    handle.handle.join().unwrap();
}
