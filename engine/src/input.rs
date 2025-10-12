use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use crate::{
    board::Board,
    common::{coordinate::Coordinate, rotation::Rotation},
    environment::Environment,
    piece::{Piece, PieceRef},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Input<'a> {
    pub board: Board,
    pub piece: Piece,
    pub environment: &'a Environment<'a>,

    last_successful_action: Option<Key>,
}

impl<'a> Input<'a> {
    #[must_use]
    pub fn new(board: Board, p: PieceRef, environment: &'a Environment) -> Self {
        let piece = Piece {
            name: p,
            rotation: Rotation::North,
            // @usMath says its 1
            location: Coordinate::new(1, board.height() + 4),
        };

        Self {
            board,
            piece,
            environment,
            last_successful_action: None,
        }
    }

    #[must_use]
    pub fn fingerprint(&self) -> Piece {
        self.piece
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        let mut c = self.piece.cells(self.environment);
        c.all(|x| matches!(x, Some(v) if v.x < self.board.width() && !self.board.get(v.x,v.y)))
    }

    pub fn move_left(&mut self) {
        let cx = self.piece.location.x.checked_sub(1);
        if cx.is_none() {
            return;
        }

        // lol
        unsafe {
            // dbg!("usafe block");
            self.piece.location.x = cx.unwrap_unchecked();
        }
        if !self.is_valid() {
            self.piece.location.x += 1;
            return;
        }

        self.last_successful_action = Some(Key::MoveLeft);
    }

    pub fn move_right(&mut self) {
        let cx = self.piece.location.x.checked_add(1);
        if cx.is_none() {
            return;
        }

        unsafe {
            // dbg!("usafe block");
            self.piece.location.x = cx.unwrap_unchecked();
        }
        if !self.is_valid() {
            self.piece.location.x -= 1;
            return;
        }

        self.last_successful_action = Some(Key::MoveRight);
    }

    pub fn soft_drop(&mut self) {
        let cx = self.piece.location.y.checked_sub(1);
        if cx.is_none() {
            return;
        }

        unsafe {
            // dbg!("usafe block");
            self.piece.location.y = cx.unwrap_unchecked();
        }
        if !self.is_valid() {
            self.piece.location.y += 1;
            return;
        }

        self.last_successful_action = Some(Key::SoftDrop);
    }

    pub fn das_left(&mut self) {
        let mut moved = false;

        while let Some(cx) = self.piece.location.x.checked_sub(1) {
            self.piece.location.x = cx;
            if !self.is_valid() {
                self.piece.location.x += 1;
                break;
            }
            moved = true;
        }

        if moved {
            self.last_successful_action = Some(Key::DasLeft);
        }
    }

    pub fn das_right(&mut self) {
        let mut moved = false;

        while let Some(cx) = self.piece.location.x.checked_add(1) {
            self.piece.location.x = cx;
            if !self.is_valid() {
                self.piece.location.x -= 1;
                break;
            }
            moved = true;
        }

        if moved {
            self.last_successful_action = Some(Key::DasRight);
        }
    }

    pub fn sonic_drop(&mut self) {
        let mut moved = false;

        while let Some(cy) = self.piece.location.y.checked_sub(1) {
            // println!("dropping");
            self.piece.location.y = cy;
            if !self.is_valid() {
                self.piece.location.y += 1;
                break;
            }
            moved = true;
        }

        if moved {
            self.last_successful_action = Some(Key::SonicDrop);
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn rotate_cw(&mut self) {
        // println!("calling rotate_cw()");
        let ir = self.piece.rotation;
        let fr = self.piece.rotation.rotate_cw();
        let k = self
            .environment
            .state
            .kicks
            .get(&self.piece, ir, fr)
            .expect("piece should have this interaction defined in the .kick file");
        let ipos = self.piece.location;

        for &test in &k.tests {
            // println!("testing {test}");
            let cx = ipos.x.checked_add_signed(test.x as isize);
            let cy = ipos.y.checked_add_signed(test.y as isize);

            if let Some(x) = cx
                && let Some(y) = cy
            {
                self.piece.rotation = fr;
                self.piece.location.x = x;
                self.piece.location.y = y;

                if !self.is_valid() {
                    // println!("test {test} failed");
                    self.piece.rotation = ir;
                    self.piece.location = ipos;
                    continue;
                }

                self.last_successful_action = Some(Key::RotateCW);
                return;
            }
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn rotate_ccw(&mut self) {
        let ir = self.piece.rotation;
        let fr = self.piece.rotation.rotate_ccw();
        let k = self
            .environment
            .state
            .kicks
            .get(&self.piece, ir, fr)
            .expect("piece should have this interaction defined in the .kick file");
        let ipos = self.piece.location;

        for &test in &k.tests {
            let cx = ipos.x.checked_add_signed(test.x as isize);
            let cy = ipos.y.checked_add_signed(test.y as isize);

            if let Some(x) = cx
                && let Some(y) = cy
            {
                self.piece.rotation = fr;
                self.piece.location.x = x;
                self.piece.location.y = y;

                if !self.is_valid() {
                    self.piece.rotation = ir;
                    self.piece.location = ipos;
                    continue;
                }

                self.last_successful_action = Some(Key::RotateCCW);
                return;
            }
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn rotate_180(&mut self) {
        let ir = self.piece.rotation;
        let fr = self.piece.rotation.rotate_180();
        let k = self
            .environment
            .state
            .kicks
            .get(&self.piece, ir, fr)
            .expect("piece should have this interaction defined in the .kick file");
        let ipos = self.piece.location;

        for &test in &k.tests {
            let cx = ipos.x.checked_add_signed(test.x as isize);
            let cy = ipos.y.checked_add_signed(test.y as isize);

            if let Some(x) = cx
                && let Some(y) = cy
            {
                self.piece.rotation = fr;
                self.piece.location.x = x;
                self.piece.location.y = y;

                if !self.is_valid() {
                    self.piece.rotation = ir;
                    self.piece.location = ipos;
                    continue;
                }

                self.last_successful_action = Some(Key::Rotate180);
                return;
            }
        }
    }

    pub fn send(&mut self, key: Key) {
        // self.sonic_drop(); // evil hack LOLZ
        match key {
            Key::MoveLeft => self.move_left(),
            Key::MoveRight => self.move_right(),
            Key::SoftDrop => self.soft_drop(),
            Key::DasLeft => self.das_left(),
            Key::DasRight => self.das_right(),
            Key::SonicDrop => self.sonic_drop(),
            Key::RotateCW => self.rotate_cw(),
            Key::RotateCCW => self.rotate_ccw(),
            Key::Rotate180 => self.rotate_180(),
            Key::Hold => (),
        }
    }

    #[must_use]
    pub fn place(mut self, hd: bool) -> Board {
        if hd {
            self.sonic_drop();
        }

        for c in self.piece.cells(self.environment).flatten() {
            self.board.set(c.x, c.y, true);
        }

        self.board.skim();

        self.board
    }

    #[must_use]
    pub fn is_spin(&self) -> bool {
        // dbg!(self.last_successful_action);
        match self.last_successful_action {
            Some(Key::RotateCW | Key::RotateCCW | Key::Rotate180) => {}
            _ => return false,
        }

        let Some(corner_set) = self
            .environment
            .state
            .corners
            .get(self.piece.name, self.piece.rotation)
        else {
            return false;
        };

        let mut filled = 0;
        #[allow(
            clippy::cast_possible_wrap,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss
        )]
        for corner in &corner_set.corners.values {
            let x = self.piece.location.x as i32 + corner.x;
            let y = self.piece.location.y as i32 + corner.y;

            if x < 0
                || y < 0
                || x >= self.board.width() as i32
                || y >= self.board.height() as i32
                || self.board.get(x as usize, y as usize)
            {
                filled += 1;
            }
        }

        filled >= 3
    }

    pub fn apply(&mut self, f: Finesse) {
        for i in 0..f.len {
            let key = f.get(i).unwrap();

            self.send(key);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    MoveLeft,
    MoveRight,
    SoftDrop,
    DasLeft,
    DasRight,
    SonicDrop,
    RotateCW,
    RotateCCW,
    Rotate180,
    Hold,
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::DasLeft => "moveLeft",
                Self::DasRight => "moveRight",
                Self::MoveLeft => "moveLeft",
                Self::MoveRight => "moveRight",
                Self::Rotate180 => "rotate180",
                Self::RotateCCW => "rotateCCW",
                Self::RotateCW => "rotateCW",
                Self::SoftDrop => "softDrop",
                Self::SonicDrop => "softDrop",
                Self::Hold => "hold",
            }
        )
    }
}

impl Key {
    pub fn short(self) -> &'static str {
        match self {
            Self::DasLeft => "dl",
            Self::DasRight => "dr",
            Self::MoveLeft => "l",
            Self::MoveRight => "r",
            Self::Rotate180 => "f",
            Self::RotateCCW => "ccw",
            Self::RotateCW => "cw",
            Self::SoftDrop => "fd",
            Self::SonicDrop => "sd",
            Self::Hold => "h",
        }
    }
}

impl FromStr for Key {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "dl" => Ok(Self::DasLeft),
            "dr" => Ok(Self::DasRight),
            "l" => Ok(Self::MoveLeft),
            "r" => Ok(Self::MoveRight),
            "f" => Ok(Self::Rotate180),
            "ccw" => Ok(Self::RotateCCW),
            "cw" => Ok(Self::RotateCW),
            "fd" => Ok(Self::SoftDrop),
            "sd" => Ok(Self::SonicDrop),
            "h" => Ok(Self::Hold),
            c => Err(format!("unknown variant {c}")),
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Finesse {
    packed: u128,
    pub len: u8,
    spin: bool,
}

impl Extend<Key> for Finesse {
    fn extend<T: IntoIterator<Item = Key>>(&mut self, iter: T) {
        for i in iter {
            self.push(i);
        }
    }
}

impl IntoIterator for Finesse {
    type Item = Key;
    type IntoIter = FinesseIterator;

    fn into_iter(self) -> Self::IntoIter {
        FinesseIterator { f: self, idx: 0 }
    }
}

pub struct FinesseIterator {
    f: Finesse,
    idx: u8,
}

impl Iterator for FinesseIterator {
    type Item = Key;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.f.len {
            return None;
        }
        let val = (self.f.packed >> (self.idx * 4)) & 0x0F;
        self.idx += 1;
        Some(unsafe {
            std::mem::transmute::<u8, Key>(val as u8)
        })
    }
}

impl Finesse {
    pub fn new() -> Self {
        Self {
            packed: 0,
            len: 0,
            spin: false,
        }
    }
    pub fn push(&mut self, key: Key) {
        self.packed |= (key as u128) << (self.len * 4);
        self.len += 1;
    }

    pub fn get(&self, idx: u8) -> Option<Key> {
        if idx >= self.len {
            return None;
        }
        let val = (self.packed >> (idx * 4)) & 0x0F;
        Some(unsafe {
            // dbg!("usafe block");
            // println!("{self} {idx}");
            std::mem::transmute::<u8, Key>(val as u8)
        })
    }

    pub fn with_spin(mut self, s: bool) -> Self {
        self.spin = s;
        self
    }

    pub fn short(self) -> String {
        let mut v = vec![];
        for i in 0..self.len {
            v.push(self.get(i).unwrap().short());
        }

        v.join(",")
    }

    pub fn with(t: &[Key]) -> Self {
        let mut n = Self::new();
        for i in t {
            n.push(*i);
        }

        n
    }
}

impl Default for Finesse {
    fn default() -> Self {
        Self::new()
    }
}
impl Debug for Finesse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Finesse")
            .field(&self.packed)
            .field(&self.len)
            .field(&(if self.spin { 1 } else { 0 }))
            .finish()
    }
}
impl Display for Finesse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for t in 0..self.len {
            write!(
                f,
                "{}{}",
                self.get(t).unwrap(),
                if t == self.len - 1 { "" } else { " " },
            )?;
        }

        write!(f, "{}", if self.spin { " *" } else { "" })?;

        Ok(())
    }
}

impl FromStr for Finesse {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Ok(Self::new());
        }
        s.split(',')
            .map(|x| x.parse())
            .collect::<Result<Vec<_>, _>>()
            .map(|x| Self::with(&x))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pair(pub char, pub Finesse);

// this is of format `(X:cw,ccw,sd,...)`
impl FromStr for Pair {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if !s.starts_with('(') || !s.ends_with(')') {
            return Err("finesse must start with '(' and end with ')'".into());
        }
        let s = &s[1..s.len() - 1];
        
        let mut parts = s.splitn(2, ':');
        let piece = parts
            .next()
            .ok_or_else(|| "missing piece".to_string())?
            .chars()
            .next()
            .ok_or_else(|| "piece must be a single character".to_string())?;
        let finesse_str = parts
            .next()
            .ok_or_else(|| "missing finesse".to_string())?;
        let finesse: Finesse = finesse_str.parse()?;


        Ok(Self(piece, finesse))
    }
}