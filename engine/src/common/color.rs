use std::{fmt::Display, str::FromStr};

use crate::common::UnknownVariant;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Color {
    I,
    J,
    O,
    L,
    Z,
    S,
    T,
    G,
    E,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::I => "I",
                Self::J => "J",
                Self::O => "O",
                Self::L => "L",
                Self::Z => "Z",
                Self::S => "S",
                Self::T => "T",
                Self::G => "G",
                Self::E => "E",
            }
        )
    }
}

impl FromStr for Color {
    type Err = UnknownVariant;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "i" => Ok(Self::I),
            "j" => Ok(Self::J),
            "o" => Ok(Self::O),
            "l" => Ok(Self::L),
            "z" => Ok(Self::Z),
            "s" => Ok(Self::S),
            "t" => Ok(Self::T),
            "g" => Ok(Self::G),
            "e" => Ok(Self::E),
            c => Err(UnknownVariant(c.to_owned())),
        }
    }
}
