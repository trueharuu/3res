use std::{fmt::Display, str::FromStr};

use rustc_hash::FxHashMap;

use crate::common::{
    color::Color,
    coordinate::{CoordinateParseErr, Coordinates},
    rotation::Rotation,
};

// format for .piece files
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bag {
    pub shapes: FxHashMap<(u8, Rotation), Shape>,
}

impl Bag {
    #[must_use]
    pub fn get(&self, name: u8, rotation: Rotation) -> Option<&Shape> {
        self.shapes.get(&(name, rotation))
    }

    pub fn pieces(&self) -> impl Iterator<Item = u8> {
        self.shapes.keys().map(|x| x.0)
    }

    #[must_use]
    pub fn width(&self, name: u8, rotation: Rotation) -> Option<usize> {
        let c = self.get(name, rotation)?;
        Some(
            (c.cells.values.iter().map(|x| x.x).max()?
                - c.cells.values.iter().map(|x| x.x).min()?)
            .unsigned_abs() as usize,
        )
    }
}

impl Display for Bag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.shapes
                .values()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl FromStr for Bag {
    type Err = ShapeParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            shapes: s
                .lines()
                .map(str::parse::<Shape>)
                .map(|x| x.map(|s| ((s.name, s.rotation), s)))
                .collect::<Result<FxHashMap<_, _>, _>>()?,
        })
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Shape {
    pub name: u8,
    pub color: Color,
    pub rotation: Rotation,
    pub cells: Coordinates<i32>,
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}={}@{}",
            self.name, self.rotation, self.cells, self.color
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShapeParseErr {
    Malformed,
    UnknownRotation(<Rotation as FromStr>::Err),
    CoordinatesErr(CoordinateParseErr<i32>),
    UnknownColor(<Color as FromStr>::Err),
}

impl FromStr for Shape {
    type Err = ShapeParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let period = s.find('.');
        let eq = s.find('=');
        let at = s.find('@');
        if let Some(p) = period
            && let Some(e) = eq
            && let Some(a) = at
        {
            let name = s[..p].parse::<char>().map(|x| x as u8).map_err(|_| ShapeParseErr::Malformed)?;
            let rotation = s[p + 1..e]
                .parse()
                .map_err(ShapeParseErr::UnknownRotation)?;
            let cells = s[e + 1..a].parse().map_err(ShapeParseErr::CoordinatesErr)?;
            let color = s[a + 1..].parse().map_err(ShapeParseErr::UnknownColor)?;

            Ok(Self {
                name,
                color,
                rotation,
                cells,
            })
        } else {
            Err(ShapeParseErr::Malformed)
        }
    }
}
