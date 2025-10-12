use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    hash::Hash,
};

use smallvec::{SmallVec, smallvec};

use crate::{board::Board, environment::Environment, input::{Pair}};

#[derive(Clone, Eq, Debug)]
pub struct History(pub SmallVec<[Pair; 16]>);

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
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.queue().collect::<String>().hash(state);
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

pub type Set<T> = BTreeSet<T>;
/// Generates all possible queues of length less than or equal to `n` such that
/// from an empty board, queue can be placed such that we result in a perfect clear
/// for each queue, return the list of inputs used to get there
///
/// sample output:
/// I = I ()
/// JJ = J () J (f r)
/// TTSZ = [longer finesse...]
pub fn generate_all_pc_queues(n: usize, env: &Environment) -> Set<History> {
    // forwards_saved_transitions = {}  # (board_hash, piece) -> next_board_list
    let mut forwards_saved_transitions = HashMap::new();
    // forwards_queue = deque()
    let mut forwards_queue: VecDeque<(Board, History)> = VecDeque::new();

    // forwards_reachable_states = defaultdict(set)  # board_hash -> queue_set
    let mut forwards_reachable_states: HashMap<Board, Set<History>> = HashMap::new();

    let mut visited = Set::new();

    forwards_queue.push_back((Board::empty(), History(smallvec![])));

    // let mut i: usize = 0;
    // let mut max = 7usize.pow(n as u32);

    while let Some(current) = forwards_queue.pop_front() {
        // let key = (current.0, current.1.0);
        if visited.contains(&current) {
            continue;
        }

        visited.insert(current.clone());

        // i += 1;
        let (board, history) = current;

        // check each possible next piece
        for piece in env.state.bag.pieces() {
            if !forwards_saved_transitions.contains_key(&(board, piece)) {
                forwards_saved_transitions
                    .insert((board, piece), board.get_next_boards(piece, env));
            }

            for &(next_board, f) in forwards_saved_transitions.get(&(board, piece)).unwrap() {
                // track reachable board states
                if next_board.height() <= n {
                    let new_history: History = {
                        let mut n = history.0.clone();
                        n.push(Pair(piece, f));
                        History(n)
                    };
                    if new_history.0.len() < n && !next_board.is_empty() {
                        forwards_queue.push_back((next_board, new_history.clone()));
                    }

                    forwards_reachable_states
                        .entry(next_board)
                        .or_default()
                        .insert(new_history.clone());
                    // if next_board.is_empty() {
                    //     println!("{} {i}/{max}", new_history.queue().collect::<String>())
                    // }
                }
            }
        }
    }

    for (board, queues) in forwards_reachable_states {
        if board.is_empty() {
            return queues;
        }
    }

    Set::new()
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
        for order in get_queue_orders(&[queue[0]].iter().chain(&queue[2..]).copied().collect::<Vec<_>>()) {
            results.push(format!("{}{}", queue[1], order));
        }
    }

    results
}

pub fn max_pcs_in_queue(queue: &[char], env: &Environment, pcs: Set<History>) -> Vec<History> {
    
    todo!()
}