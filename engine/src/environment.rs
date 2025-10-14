use smallvec::SmallVec;

use crate::{
    input::{Key, Pair},
    pc::{History, Map, generate_all_pc_queues},
    piece::Queue,
    repl::State,
};
use std::io::Write;

#[derive(Debug, PartialEq, Eq)]
pub struct Environment<'a> {
    pub can_tap: bool,
    pub can_das: bool,
    pub can_180: bool,
    pub can_hold: bool,
    pub droptype: DropType,
    pub vision: usize,
    pub foresight: usize,
    pub upstack: bool,
    pub state: &'a mut State,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum DropType {
    Sonic,
    Soft,
    Hard,
    Both,
}

impl<'a> Environment<'a> {
    #[must_use] 
    pub fn new(state: &'a mut State, flags: &str, vision: usize, foresight: usize) -> Self {
        let mut can_180 = false;
        let mut can_tap = false;
        let mut can_das = false;
        let mut can_hold = false;
        let mut upstack = false;

        for c in flags.chars() {
            match c {
                'f' => can_180 = true,
                't' => can_tap = true,
                'd' => can_das = true,
                'h' => can_hold = true,
                'u' => upstack = true,
                _ => {}
            }
        }

        Self {
            can_180,
            can_tap,
            can_das,
            can_hold,
            droptype: DropType::Sonic,
            vision,
            foresight,
            upstack,
            state,
        }
    }
    #[must_use] 
    pub fn flags(&self) -> String {
        format!(
            "{}{}{}{}{}",
            if self.can_180 { 'f' } else { '-' },
            if self.can_tap { 't' } else { '-' },
            if self.can_das { 'd' } else { '-' },
            if self.can_hold { 'h' } else { '-' },
            if self.upstack { 'u' } else { '-' },
        )
    }
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

    #[must_use] 
    pub fn pcs(&self, n: usize, force: bool) -> Map<Queue, History> {
        // if it exists as a file, load from file
        let path = format!("data/{}_{}_{n}.pc", self.state.fingerprint.0, self.flags());

        if !force && let Ok(s) = std::fs::read_to_string(&path) {
            return Self::parse_pcs(&s);
        }

        // otherwise, generate and save to file
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(
            f,
            "#n={n};kicktable={};total={};{}",
            self.state.fingerprint.0,
            0,
            self.flags()
        )
        .unwrap();
        generate_all_pc_queues(&mut f, n, self);

        // dedup the file
        let s = std::fs::read_to_string(&path).unwrap();
        let mut lines: Vec<_> = s.lines().collect();
        lines.sort_unstable();
        lines.dedup_by_key(|x| x.split('=').next().unwrap().trim());
        let s = lines.join("\n");

        std::fs::write(&path, s).unwrap();

        Self::parse_pcs(&std::fs::read_to_string(&path).unwrap())
    }

    #[must_use] 
    pub fn parse_pcs(s: &str) -> Map<Queue, History> {
        let mut pcs = Map::new();

        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let mut parts = line.split('=');
            let queue = parts.next().unwrap().trim().parse().unwrap();
            let finesse = parts.next().unwrap().trim();

            // convert finesse into Vec<Pair>
            // Pair is (char, Finesse)

            let f = finesse
                .split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<SmallVec<[Pair; _]>, _>>()
                .unwrap();

            pcs.insert(queue, History(f));
        }

        pcs
    }
}
