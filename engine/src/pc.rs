use std::collections::{HashSet, VecDeque};

use crate::board::Board;

pub fn generate_all_pc_queues(n: usize, h: usize) {
    let h = n.min(h);
    let mut pcs = HashSet::new();

    let n_b = n / 4 + 1;
    let n_f = n - n_b;

    let bq = VecDeque::new();
    bq.push_back(Board::empty());
}
