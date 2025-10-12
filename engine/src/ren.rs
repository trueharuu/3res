use std::collections::VecDeque;

use crate::{board::Board, environment::Environment, input::Finesse};

#[derive(Debug, Clone, Eq, Hash)]
pub struct Node<'a> {
    pub board: Board,
    pub hold: Option<char>,
    pub queue: &'a [char],
    pub prev: Option<Box<Self>>,
    pub finesse: Finesse,
    pub used: Option<char>,
    pub ptr: usize,
}

pub fn ren_bfs(state: &Node, env: &Environment) -> Vec<Vec<PathItem>> {
    let mut queue: VecDeque<Node> = VecDeque::new();
    let mut visited: Vec<Node> = Vec::new();

    let ql = env.vision;

    // todo: optimize for n > 0
    // `n` represents the maximum amount of breaks we allow in a single path
    // problem is our sample space increases exponentially as `n` increases
    // this usually isnt an issue given that combo tends to have very few breaks on a sensible `vision`
    // but this does take an abundantly long time for poor `vision` or just actually evil queues
    for n in 0..env.vision {
        queue.push_front(state.clone());
        visited.push(state.clone());

        let mut p = vec![];
        while !queue.is_empty() {
            let current_node = queue.pop_back().unwrap();

            // discard any segment that has more than wanted non-combo placements
            if current_node.breaks() > n {
                continue;
            }

            // only output finished paths
            if current_node.ptr == ql {
                p.push(current_node.path());
            }

            for neighbor in current_node.neighbors(env) {
                visited.push(neighbor.clone());
                queue.push_front(neighbor);
            }
        }

        if !p.is_empty() {
            return p;
        }
    }

    return vec![];
}

impl<'a> PartialEq for Node<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr && self.hold == other.hold && self.board == other.board
    }
}

impl<'a> Node<'a> {
    pub fn neighbors(self, env: &Environment) -> Vec<Self> {
        let mut n = vec![];

        if self.queue.is_empty() {
            if let Some(h) = self.hold {
                for (c, f) in self.board.get_next_boards(h, env) {
                    n.push(Node {
                        board: c,
                        hold: None,
                        queue: &self.queue[0..0],
                        prev: Some(Box::new(self.clone())),
                        finesse: f,
                        used: Some(h),
                        ptr: self.ptr + 1,
                    });
                }

                return n;
            } else {
                return n;
            }
        }

        // consume regularly
        for (c, f) in self.board.get_next_boards(self.queue[0], env) {
            n.push(Node {
                board: c,
                hold: self.hold,
                queue: &self.queue[1..],
                prev: Some(Box::new(self.clone())),
                finesse: f,
                used: Some(self.queue[0]),
                ptr: self.ptr + 1,
            });
        }

        if let Some(h) = self.hold {
            for (c, f) in self.board.get_next_boards(h, env) {
                n.push(Node {
                    board: c,
                    hold: Some(self.queue[0]),
                    queue: &self.queue[1..],
                    prev: Some(Box::new(self.clone())),
                    finesse: f,
                    used: Some(h),
                    ptr: self.ptr + 1,
                });
            }
        }

        if let Some(t) = self.queue.get(1)
            && self.hold.is_none()
        {
            for (c, f) in self.board.get_next_boards(*t, env) {
                n.push(Node {
                    board: c,
                    hold: Some(self.queue[0]),
                    queue: &self.queue[2..],
                    prev: Some(Box::new(self.clone())),
                    finesse: f,
                    used: Some(self.queue[1]),
                    ptr: self.ptr + 1,
                });
            }
        }

        n
    }

    pub fn size(&self) -> usize {
        1 + self.prev.as_ref().map(|x| x.size()).unwrap_or(0)
    }

    pub fn path(&self) -> Vec<PathItem> {
        if self.prev.is_none() {
            return vec![];
        }

        [
            self.prev.clone().unwrap().path(),
            vec![PathItem(self.board, self.used.unwrap(), self.finesse)],
        ]
        .concat()
    }

    pub fn path_small(&self) -> String {
        self.path()
            .into_iter()
            .map(|x| format!("({} {})", x.1, x.2.short()))
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn breaks(&self) -> usize {
        if let Some(p) = &self.prev {
            (if self.board.num_minos() > p.board.num_minos() {
                1
            } else {
                0
            }) + p.breaks()
        } else {
            0
        }
    }

    pub fn non_pcs(&self) -> usize {
        if let Some(p) = &self.prev {
            (if self.board.num_minos() != 0 { 1 } else { 0 }) + p.non_pcs()
        } else {
            0
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct PathItem(pub Board, pub char, pub Finesse);
