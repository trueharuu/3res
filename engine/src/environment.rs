use crate::{
    file::{corners::Corners, kicks::Kicks, piece::Bag},
    input::Key,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Environment {
    pub kicks: Kicks,
    pub bag: Bag,
    pub corners: Corners,
    pub can_tap: bool,
    pub can_das: bool,
    pub can_180: bool,
    pub can_hold: bool,
    pub droptype: DropType,
    pub vision: usize,
    pub foresight: usize,
    pub upstack: bool,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum DropType {
    Sonic,
    Soft,
    Hard,
    Both,
}

impl Environment {
    #[must_use] 
    pub fn keyboard(&self) -> Vec<Key> {
        let mut m = vec![];
        if self.can_tap {
            m.push(Key::MoveLeft);
            m.push(Key::MoveRight);
        }

        if self.can_das {
            m.push(Key::DasLeft);
            m.push(Key::DasRight);
        }

        m.push(Key::RotateCW);
        m.push(Key::RotateCCW);
        if self.can_180 {
            m.push(Key::Rotate180);
        }

        match self.droptype {
            DropType::Soft => m.push(Key::SoftDrop),
            DropType::Sonic => m.push(Key::SonicDrop),
            DropType::Both => {
                m.push(Key::SoftDrop);
                m.push(Key::SonicDrop);
            }
            DropType::Hard => {}
        }

        m
    }
}
