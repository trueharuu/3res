use std::{
    collections::{HashMap, HashSet},
    hash::{DefaultHasher, Hash, Hasher},
    io::Write,
};

use smallvec::{SmallVec, smallvec};

use crate::{
    board::Board,
    environment::Environment,
    input::{Finesse, Key, Pair},
};

#[derive(Clone, Eq, Debug)]
pub struct History(pub SmallVec<[Pair; 4]>);

impl History {
    pub fn queue(&self) -> impl Iterator<Item = char> {
        self.0.iter().map(|x| x.0)
    }
}

impl PartialEq for History {
    fn eq(&self, other: &Self) -> bool {
        let l = self.0.iter().map(|x| x.0);
        let r = other.0.iter().map(|x| x.0);

        l.eq(r)
    }
}

impl Hash for History {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for c in self.queue() {
            c.hash(state);
        }
    }
}

impl PartialOrd for History {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for History {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0.len().cmp(&other.0.len()) {
            std::cmp::Ordering::Equal => {
                self.0.iter().map(|x| x.0).cmp(other.0.iter().map(|x| x.0))
            }

            c => c,
        }
    }
}

fn hash_queue(history: &History) -> u64 {
    let mut hasher = DefaultHasher::new();
    for c in history.queue() {
        c.hash(&mut hasher);
    }
    hasher.finish()
}

pub type Set<T> = HashSet<T>;
/// Generates all possible queues of length less than or equal to `n` such that
/// from an empty board, queue can be placed such that we result in a perfect clear
/// for each queue, return the list of inputs used to get there
///
/// sample output:
/// I = I ()
/// JJ = J () J (f r)
/// TTSZ = [longer finesse...]
pub fn generate_all_pc_queues(buf: &mut impl Write, n: usize, env: &Environment) {
    const MAX_CACHE_SIZE: usize = 10000; // Maximum number of cached transitions
    
    // Use a Vec as a stack for depth-first search (uses less memory than VecDeque)
    let mut stack: Vec<(Board, History)> = Vec::new();
    let mut forwards_saved_transitions = HashMap::with_capacity(MAX_CACHE_SIZE);
    
    // Use a more compact visited state representation
    let mut visited: HashSet<u64> = HashSet::new();
    
    // Track cache stats for periodic cleanup
    let mut cache_access_count = 0;
    
    stack.push((Board::empty(), History(smallvec![])));

    while let Some((board, history)) = stack.pop() {
        // Create a combined hash of board and history
        let mut hasher = DefaultHasher::new();
        board.hash(&mut hasher);
        for c in history.queue() {
            c.hash(&mut hasher);
        }
        let state_hash = hasher.finish();
        
        if !visited.insert(state_hash) {
            continue;
        }
        
        // Periodically clear the transition cache if it gets too large
        cache_access_count += 1;
        if cache_access_count % 1000 == 0 && forwards_saved_transitions.len() > MAX_CACHE_SIZE {
            forwards_saved_transitions.clear();
        }

        // check each possible next piece
        for piece in env.state.bag.pieces() {
            let next_boards = forwards_saved_transitions
                .entry((board, piece))
                .or_insert_with(|| board.get_next_boards(piece, env));

            for &mut (next_board, f) in next_boards {
                // track reachable board states
                if next_board.height() <= n {
                    let new_history: History = {
                        let mut n = history.0.clone();
                        n.push(Pair(piece, f));
                        History(n)
                    };
                    if new_history.0.len() < n && !next_board.is_empty() {
                        stack.push((next_board, new_history));
                    } else if next_board.is_empty() {
                        writeln!(
                            buf,
                            "{} = {}",
                            new_history.queue().collect::<String>(),
                            new_history
                                .0
                                .iter()
                                .map(|x| format!("({}:{})", x.0, x.1.short()))
                                .collect::<Vec<_>>()
                                .join(" ")
                        )
                        .unwrap();
                    }
                }
            }
        }
    }
}

/// Obtains all possible ways to play a queue given one hold
/*
def get_queue_orders(queue):
  if len(queue) == 1:
    yield queue[0]
    return
  for queue_order in get_queue_orders(queue[1:]):
    yield queue[0] + queue_order
  for queue_order in get_queue_orders(queue[0] + queue[2:]):
    yield queue[1] + queue_order
*/
pub fn get_queue_orders(queue: &[char]) -> Vec<String> {
    if queue.len() == 1 {
        return vec![queue[0].to_string()];
    }

    let mut results = vec![];

    for order in get_queue_orders(&queue[1..]) {
        results.push(format!("{}{}", queue[0], order));
    }

    if queue.len() > 2 {
        for order in get_queue_orders(
            &[queue[0]]
                .iter()
                .chain(&queue[2..])
                .copied()
                .collect::<Vec<_>>(),
        ) {
            results.push(format!("{}{}", queue[1], order));
        }
    }

    results
}

pub fn max_pcs_in_queue(queue: &[char], _env: &Environment, _pcs: Set<History>) -> Vec<History> {
    // todo: make this not fake LOL
    vec![History(smallvec![Pair(
        queue[0],
        Finesse::with(&[Key::Rotate180])
    )])]
}
