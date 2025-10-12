use std::{fmt::Display, str::FromStr};

use crate::common::UnknownVariant;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Rotation {
    North,
    East,
    South,
    West,
}

impl Rotation {
    #[must_use] 
    pub fn rotate_cw(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    #[must_use] 
    pub fn rotate_ccw(self) -> Self {
        match self {
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
            Self::North => Self::West,
        }
    }

    #[must_use] 
    pub fn rotate_180(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::North => "N",
                Self::East => "E",
                Self::South => "S",
                Self::West => "W",
            }
        )
    }
}

impl FromStr for Rotation {
    type Err = UnknownVariant;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "n" | "north" | "spawn" | "0" => Ok(Self::North),
            "e" | "east" | "right" | "1" => Ok(Self::East),
            "s" | "south" | "reverse" | "2" => Ok(Self::South),
            "w" | "west" | "left" | "3" => Ok(Self::West),
            c => Err(UnknownVariant(c.to_string())),
        }
    }
}
