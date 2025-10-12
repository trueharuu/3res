use std::str::FromStr;

use crate::{common::{
    coordinate::{CoordinateParseErr, Coordinates},
    rotation::Rotation,
}, piece::{PieceRef, PieceTy}};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Corners {
    pub entries: Vec<CornerSet>,
}

impl Corners {
    #[must_use] 
    pub fn get(&self, piece: PieceRef, rotation: Rotation) -> Option<&CornerSet> {
        self.entries.iter().find(|x| x.piece == piece && x.rotation == rotation)
    }
}

impl FromStr for Corners {
    type Err = CornerParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            entries: s
                .lines()
                .map(str::parse::<CornerSet>)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CornerSet {
    pub piece: PieceTy,
    pub rotation: Rotation,
    pub corners: Coordinates<i32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CornerParseErr {
    Malformed,
    UnknownRotation(<Rotation as FromStr>::Err),
    CoordinatesErr(CoordinateParseErr<i32>),
}
impl FromStr for CornerSet {
    type Err = CornerParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let period = s.find('.');
        let eq = s.find('=');

        if let Some(p) = period
            && let Some(e) = eq
        {
            let piece = s[0..p].parse().map_err(|_| CornerParseErr::Malformed)?;
            let rotation = s[p + 1..e]
                .parse()
                .map_err(CornerParseErr::UnknownRotation)?;

            let corners = s[e + 1..].parse().map_err(CornerParseErr::CoordinatesErr)?;

            Ok(Self {
                piece,
                rotation,
                corners,
            })
        } else {
            Err(CornerParseErr::Malformed)
        }
    }
}
