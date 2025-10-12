use smallvec::SmallVec;

use crate::{
    input::{Key, Pair},
    pc::{History, Set, generate_all_pc_queues},
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
    pub state: &'a State,
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

    pub fn pcs(&self, n: usize) -> Set<History> {
        // if it exists as a file, load from file
        let path = format!("data/{}_{n}.pc", self.state.fingerprint.0);

        if let Ok(s) = std::fs::read_to_string(&path) {
            return Self::parse_pcs(&s);
        }

        // otherwise, generate and save to file
        let pcs = generate_all_pc_queues(n, self);
        let mut f = std::fs::File::create(&path).unwrap();

        writeln!(
            f,
            "#n={n};kicktable={};total={}",
            self.state.fingerprint.0,
            pcs.len()
        )
        .unwrap();
        for i in &pcs {
            if i.queue().count() != n {
                continue;
            }
            writeln!(
                f,
                "{} = {}",
                i.queue().collect::<String>(),
                i.0.iter()
                    .map(|x| format!("({}:{})", x.0, x.1.short()))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
            .unwrap();
        }

        pcs
    }

    pub fn parse_pcs(s: &str) -> Set<History> {
        let mut pcs = Set::new();

        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let mut parts = line.split('=');
            let _ = parts.next().unwrap().trim();
            let finesse = parts.next().unwrap().trim();

            // convert finesse into Vec<Pair>
            // Pair is (char, Finesse)

            let f = finesse
                .split_ascii_whitespace()
                .map(|x| x.parse())
                .collect::<Result<SmallVec<[Pair; 16]>, _>>()
                .unwrap();

            pcs.insert(History(f));

            // println!("{queue} | {f:?}");
        }

        pcs
    }
}
