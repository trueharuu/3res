use std::{
    collections::{HashSet, VecDeque},
    convert::Infallible,
    fmt::Debug,
    ops::{Add, RangeBounds},
    str::FromStr,
};

use crate::{
    board::Board,
    common::{coordinate::Coordinate, rotation::Rotation},
    environment::Environment,
    input::{Finesse, Input},
};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Piece {
    pub location: Coordinate<usize>,
    pub rotation: Rotation,
    pub name: u8,
}

impl Piece {
    #[allow(clippy::missing_panics_doc)]
    pub fn cells(&self, env: &Environment) -> impl Iterator<Item = Option<Coordinate<usize>>> {
        let shape = env
            .state
            .bag
            .get(self.name, self.rotation)
            .expect("piece should be defined in the given `.piece` file");
        shape.cells.values.iter().map(|c| {
            Some(Coordinate::new(
                self.location.x.checked_add_signed(c.x as isize)?,
                self.location.y.checked_add_signed(c.y as isize)?,
            ))
        })
    }

    /// Returns the smallest possible sequence of inputs that can take this piece from its spawn location to its current location.
    #[must_use]
    pub fn find(&self, board: Board, env: &Environment) -> Option<Finesse> {
        let i = Input::new(board, self.name, env);

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_front(Finesse::new());

        while let Some(f) = queue.pop_back() {
            if visited.contains(&i.place(false)) {
                continue;
            }
            visited.insert(i.place(false));

            if i.piece == *self {
                return Some(f);
            }

            for extra_key in env.keyboard() {
                let mut i = i;
                i.apply(f);
                i.apply(Finesse::with(&[extra_key]));
                if i.is_valid() {
                    let mut f = f;
                    f.push(extra_key);
                    queue.push_front(f);
                }
            }
        }

        todo!()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Queue {
    packed: [u128; Self::N],
    len: u8,
}

impl Queue {
    pub const N: usize = 4;
    #[must_use]
    pub fn new() -> Self {
        Self {
            packed: [0; Self::N],
            len: 0,
        }
    }

    #[must_use]
    pub fn get(&self, index: usize) -> u8 {
        let bi = index * 8;
        let ai = bi / 128;
        let o = bi % 128;

        let mut v = ((self.packed[ai] >> o) & 0xff) as u8;
        if o + 8 > 128 {
            let r = o + 8 - 128;
            v |= (((self.packed[ai + 1] & ((1 << r) - 1)) << (8 - r)) & 0xff) as u8;
        }

        v
    }

    #[must_use]
    pub fn slice(&self, range: impl RangeBounds<usize>) -> Queue {
        let mut q = Queue::new();
        for i in self
            .into_iter()
            .enumerate()
            .filter(|(i, _)| range.contains(i))
        {
            q.push(i.1);
        }

        q
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn push(&mut self, piece: u8) {
        assert!(self.len < ((Self::N * 16) & 0xff) as u8);
        let idx = (self.len / 16) as usize;
        let shift = (self.len % 16) * 8;
        self.packed[idx] |= u128::from(piece) << shift;
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let idx = (self.len / 16) as usize;
        let shift = (self.len % 16) * 8;
        let piece = ((self.packed[idx] >> shift) & 0xFF) as u8;
        self.packed[idx] &= !(0xFF << shift);
        Some(piece)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[must_use]
    pub fn as_str(&self) -> String {
        let mut s = String::new();
        for piece in *self {
            s.push(piece as char);
        }
        s
    }
}

impl Debug for Queue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}

pub struct QueueIter {
    queue: Queue,
    index: usize,
}

impl Iterator for QueueIter {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.index >= self.queue.len() {
            return None;
        }
        let idx = self.index / 16;
        let shift = (self.index % 16) * 8;
        let piece = ((self.queue.packed[idx] >> shift) & 0xFF) as u8;
        self.index += 1;
        Some(piece)
    }
}

impl IntoIterator for Queue {
    type Item = u8;
    type IntoIter = QueueIter; // need lifetime workaround if you want owned iteration

    fn into_iter(self) -> Self::IntoIter {
        QueueIter {
            queue: self,
            index: 0,
        }
    }
}

impl FromIterator<u8> for Queue {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut queue = Queue::new();
        for piece in iter {
            queue.push(piece);
        }
        queue
    }
}

impl Add<Queue> for u8 {
    type Output = Queue;
    fn add(self, rhs: Queue) -> Self::Output {
        let mut m = Queue::new();
        m.push(self);
        for i in rhs {
            m.push(i);
        }

        m
    }
}

impl Add<u8> for Queue {
    type Output = Queue;
    fn add(self, rhs: u8) -> Self::Output {
        let mut m = Queue::new();
        for i in self {
            m.push(i);
        }
        m.push(rhs);

        m
    }
}

impl Add<Queue> for Queue {
    type Output = Queue;
    fn add(self, rhs: Queue) -> Self::Output {
        let mut m = Queue::new();

        for i in self.into_iter().chain(rhs) {
            m.push(i);
        }

        m
    }
}

impl Extend<u8> for Queue {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        for i in iter {
            self.push(i);
        }
    }
}

impl FromStr for Queue {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.bytes().collect())
    }
}
