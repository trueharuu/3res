use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    hash::Hash,
    io::Write,
};

use smallvec::{SmallVec, smallvec};

use crate::{board::Board, environment::Environment, input::Pair, piece::Queue};

#[derive(Clone, Eq, Debug)]
pub struct History(pub SmallVec<[Pair; 8]>);

impl History {
    #[must_use] 
    pub fn queue(&self) -> Queue {
        self.0.iter().map(|x| x.0).collect()
    }

    #[must_use] 
    pub fn queue_str(&self) -> String {
        self.queue().as_str()
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
        for i in self.queue() {
            i.hash(state);
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

pub type Set<T> = BTreeSet<T>;
pub type Map<K, V> = BTreeMap<K, V>;
/// Generates all possible queues of length less than or equal to `n` such that
/// from an empty board, queue can be placed such that we result in a perfect clear
/// for each queue, return the list of inputs used to get there
///
/// sample output:
/// I = I ()
/// JJ = J () J (f r)
/// TTSZ = [longer finesse...]
pub fn generate_all_pc_queues(buf: &mut impl Write, n: usize, env: &Environment) {
    // forwards_saved_transitions = {}  # (board_hash, piece) -> next_board_list
    let mut forwards_saved_transitions = HashMap::new();
    // forwards_queue = deque()
    let mut forwards_queue: VecDeque<(Board, History)> = VecDeque::new();

    // forwards_reachable_states = defaultdict(set)  # board_hash -> queue_set

    let mut visited = HashSet::new();
    let mut vq = HashSet::new();

    forwards_queue.push_back((Board::empty(), History(smallvec![])));

    // let mut i: usize = 0;
    // let mut max = 7usize.pow(n as u32);

    while let Some(current) = forwards_queue.pop_front() {
        let key = (current.0, current.1.queue_str());
        if visited.contains(&key) {
            continue;
        }

        visited.insert(key.clone());
        if vq.contains(&current.1.queue()) {
            continue;
        }

        // i += 1;
        let (board, history) = current;
        
        // check each possible next piece
        for piece in env.state.bag.pieces() {
            forwards_saved_transitions.entry((board, piece)).or_insert_with(|| board.get_next_boards(piece, env));

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
                    } else if next_board.is_empty() {
                        if !vq.insert(new_history.queue()) {
                            continue;
                        }

                        writeln!(
                            buf,
                            "{} = {}",
                            new_history.queue_str(),
                            new_history
                                .0
                                .iter()
                                .map(ToString::to_string)
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
#[must_use] 
pub fn get_queue_orders(queue: Queue) -> Vec<Queue> {
    let mut results = vec![];

    if queue.len() == 1 {
        results.push(Queue::from_iter([queue.get(0)]));
        return results;
    }

    for qo in get_queue_orders(queue.slice(1..)) {
        let mut new_q = Queue::new();
        new_q.push(queue.get(0));
        for piece in qo {
            new_q.push(piece);
        }
        results.push(new_q);
    }

    if queue.len() >= 2 {
        let mut tail = Queue::new();
        tail.push(queue.get(0)); // first piece
        for piece in queue.slice(2..) {
            tail.push(piece);
        }

        for qo in get_queue_orders(tail) {
            let mut new_q = Queue::new();
            new_q.push(queue.get(1)); // second piece is prepended
            for piece in qo {
                new_q.push(piece);
            }
            results.push(new_q);
        }
    }

    results
}

#[must_use] 
pub fn max_pcs_in_queue(
    queue: Queue,
    env: &Environment,
    pcs: &Map<Queue, History>,
) -> (usize, Vec<History>) {
    let maxn = pcs.iter().map(|x| x.0.len()).max().unwrap_or_default();

    let mut dp: HashMap<(usize, u8), (usize, Option<(usize, u8)>, Option<Queue>)> = HashMap::new();

    dp.insert((1, queue.get(0)), (0, None, None));

    for i in 1..queue.len() {
        let mut reachable_holds: Vec<u8> = env.state.bag.pieces().collect::<Vec<_>>();
        reachable_holds.push(255);

        for &hold in &reachable_holds {
            let current_state = (i, hold);
            if let Some(cdp) = dp.get(&current_state).copied() {
                for pieces_used in 1..=std::cmp::min(queue.len() - i, maxn) {
                    let mut pcq = Queue::new();
                    pcq.push(hold);
                    pcq.extend(queue.slice(i..i + pieces_used));

                    let saves = get_pc_saves(pcq, pcs);

                    for (save, v) in saves {
                        let next_state = (i + pieces_used, save);
                        let new_score = cdp.0 + 1;

                        if !dp.contains_key(&next_state)
                            || new_score > dp.get(&next_state).unwrap().0
                        {
                            dp.insert(next_state, (new_score, Some(current_state), Some(v)));
                        }
                    }
                }
            }
        }
    }

    let mut max_score = 0;
    let mut best_state = None;
    for (&state, &(score, ..)) in &dp {
        if score > max_score {
            max_score = score;
            best_state = Some(state);
        }
    }

    if max_score == 0 || best_state.is_none() {
        return (0, vec![]);
    }

    let mut rev = Vec::new();
    let mut current_state = best_state;

    while let Some(c) = current_state {
        if let Some(&(_, prev, Some(ref t))) = dp.get(&c) {
            rev.push(*t);
            current_state = prev;
        } else {
            break;
        }
    }

    let history: Vec<History> = rev
        .iter()
        .filter_map(|x| pcs.get(x))
        .cloned()
        .rev()
        .collect();

    (max_score, history)
}

#[must_use] 
pub fn get_pc_saves(queue: Queue, pcs: &Map<Queue, History>) -> Map<u8, Queue> {
    let mut saves = Map::new();

    for qo in get_queue_orders(queue) {
        let len = qo.len();

        if len == 0 {
            continue;
        }

        let prefix = qo.slice(0..len - 1);

        if pcs.contains_key(&prefix) {
            saves.insert(qo.get(len - 1), prefix);
        }

        if pcs.contains_key(&qo) {
            saves.insert(255, qo);
        }
    }

    saves
}
