use std::collections::{HashSet, VecDeque};

use crate::{
    board::Board,
    common::{coordinate::Coordinate, rotation::Rotation},
    environment::Environment,
    input::{Finesse, Input},
};

pub type PieceTy = char;
pub type PieceRef = char;
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Piece {
    pub location: Coordinate<usize>,
    pub rotation: Rotation,
    pub name: PieceRef,
}

impl Piece {
    #[allow(clippy::missing_panics_doc)]
    pub fn cells(&self, env: &Environment) -> impl Iterator<Item = Option<Coordinate<usize>>> {
        let shape = env
            .state.bag
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
