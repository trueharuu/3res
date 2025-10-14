use std::{fmt::Display, str::FromStr};

use crate::{
    common::{
        coordinate::{CoordinateParseErr, Coordinates},
        rotation::Rotation,
    },
    piece::Piece,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Kicks {
    pub entries: Vec<Kick>,
}

impl Display for Kicks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.entries
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl Kicks {
    #[must_use]
    pub fn get(&self, piece: &Piece, source: Rotation, target: Rotation) -> Option<&Kick> {
        self.entries
            .iter()
            .find(|x| x.piece == piece.name && x.source == source && x.target == target)
    }
}

impl FromStr for Kicks {
    type Err = KickParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            entries: s
                .lines()
                .map(str::parse::<Kick>)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Kick {
    pub piece: u8,
    pub source: Rotation,
    pub target: Rotation,
    pub tests: Coordinates<i32>,
}

impl Display for Kick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}{}={}",
            self.piece, self.source, self.target, self.tests
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KickParseErr {
    Malformed,
    UnknownRotation(<Rotation as FromStr>::Err),
    CoordinatesErr(CoordinateParseErr<i32>),
}

impl FromStr for Kick {
    type Err = KickParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let period = s.find('.');
        let eq = s.find('=');

        if let Some(p) = period
            && let Some(e) = eq
        {
            let piece = s[0..p].parse::<char>().map(|x| x as u8).map_err(|_| KickParseErr::Malformed)?;
            let source = s[p + 1..p + 2]
                .parse()
                .map_err(KickParseErr::UnknownRotation)?;
            let target = s[p + 2..e].parse().map_err(KickParseErr::UnknownRotation)?;

            let tests = s[e + 1..].parse().map_err(KickParseErr::CoordinatesErr)?;

            Ok(Self {
                piece,
                source,
                target,
                tests,
            })
        } else {
            Err(KickParseErr::Malformed)
        }
    }
}
