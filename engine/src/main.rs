#![feature(gen_blocks, yield_expr)]
use std::{env, io};

use engine::{file::{corners::Corners, kicks::Kicks, piece::Bag}, repl::{Repl, State}};
fn main() {
    let mut args = env::args();
    args.next();
    let kn = args.next().unwrap();
    let bn = args.next().unwrap_or("tetromino".to_string());
    let cn = args.next().unwrap_or("handheld".to_string());
    // println!("{kn} {bn} {cn}");
    let k_file = std::fs::read_to_string(format!("data/{kn}.kick")).unwrap();
    let b_file = std::fs::read_to_string(format!("data/{bn}.piece")).unwrap();
    let c_file = std::fs::read_to_string(format!("data/{cn}.corners")).unwrap();

    // println!("{k_file}");
    let kicks: Kicks = k_file.parse().unwrap();
    let bag: Bag = b_file.parse().unwrap();
    let corners: Corners = c_file.parse().unwrap();

    let s = State {
        kicks,
        bag,
        corners,
        fingerprint: (kn,bn,cn),
    };

    let repl = Repl::new(io::stdin(), io::stdout(), s);
    let handle = repl.spawn();
    handle.handle.join().unwrap();
}
