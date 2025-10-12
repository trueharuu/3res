use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

use crate::common::traits::Saturating;

// a single coordinate
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinate<T> {
    pub x: T,
    pub y: T,
}

impl<T> Display for Coordinate<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:+},{:+})", self.x, self.y)
    }
}

impl<T> Coordinate<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoordinateParseErr<T>
where
    T: FromStr,
{
    // something in one of the axes failed to parse
    Contents(T::Err),
    MissingParens,
    MismatchedParens,
    MissingComma,
    MisplacedComma,
}

impl<T> FromStr for Coordinate<T>
where
    T: FromStr,
{
    type Err = CoordinateParseErr<T>;
    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        s = s.trim();

        let lp = s.find('(');
        let rp = s.find(')');

        if let Some(l) = lp
            && let Some(r) = rp
        {
            if l >= r {
                return Err(CoordinateParseErr::MismatchedParens);
            }

            let cm = s.find(',');
            if let Some(c) = cm {
                if c < l || c > r {
                    return Err(CoordinateParseErr::MisplacedComma);
                }

                let lhs = &s[l + 1..c];
                let rhs = &s[c + 1..r];
                Ok(Self {
                    x: lhs.trim().parse().map_err(CoordinateParseErr::Contents)?,
                    y: rhs.trim().parse().map_err(CoordinateParseErr::Contents)?,
                })
            } else {
                Err(CoordinateParseErr::MissingComma)
            }
        } else {
            Err(CoordinateParseErr::MissingParens)
        }
    }
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinates<T> {
    pub values: Vec<Coordinate<T>>,
}

impl<'a, T> IntoIterator for &'a Coordinates<T> {
    type Item = &'a Coordinate<T>;
    type IntoIter = std::slice::Iter<'a, Coordinate<T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl<T> Coordinates<T> {
    pub fn iter(&self) -> std::slice::Iter<'_, Coordinate<T>> {
        self.values.iter()
    }
}

impl<T> Display for Coordinates<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.values
                .iter()
                .map(ToString::to_string)
                .collect::<String>()
        )
    }
}

impl<T> FromStr for Coordinates<T>
where
    T: FromStr,
{
    type Err = CoordinateParseErr<T>;
    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut values = Vec::new();

        while !s.is_empty() {
            let start = s
                .find('(')
                .ok_or_else(|| CoordinateParseErr::MissingParens)?;
            let end = s[start..]
                .find(')')
                .ok_or_else(|| CoordinateParseErr::MissingParens)?
                + start;

            let coord_str = &s[start..=end];
            let coord = coord_str.parse::<Coordinate<T>>()?;
            values.push(coord);

            s = &s[end + 1..];
        }

        Ok(Coordinates { values })
    }
}

// saturating addition
impl<T> Add for Coordinate<T>
where
    T: Saturating,
{
    type Output = Coordinate<T>;
    fn add(self, rhs: Coordinate<T>) -> Self::Output {
        Self {
            x: self.x.saturating_add(rhs.x),
            y: self.y.saturating_add(rhs.y),
        }
    }
}

impl<T> Sub for Coordinate<T>
where
    T: Saturating,
{
    type Output = Coordinate<T>;
    fn sub(self, rhs: Coordinate<T>) -> Self::Output {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

impl<T> AddAssign for Coordinate<T>
where
    T: Saturating + Clone,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x.clone().saturating_add(rhs.x);
        self.y = self.y.clone().saturating_add(rhs.y);
    }
}

impl<T> SubAssign for Coordinate<T>
where
    T: Saturating + Clone,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x.clone().saturating_sub(rhs.x);
        self.y = self.y.clone().saturating_sub(rhs.y);
    }
}
