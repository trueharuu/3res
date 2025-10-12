use crate::{
    common::{coordinate::Coordinate, rotation::Rotation},
    file::piece::Bag,
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
    pub fn cells(&self, bag: &Bag) -> impl Iterator<Item = Option<Coordinate<usize>>> {
        let shape = bag
            .get(self.name, self.rotation)
            .expect("piece should be defined in the given `.piece` file");
        shape.cells.values.iter().map(|c| {
            Some(Coordinate::new(
                self.location.x.checked_add_signed(c.x as isize)?,
                self.location.y.checked_add_signed(c.y as isize)?,
            ))
        })
    }
}
