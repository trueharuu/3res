/// 4x64 board.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board {
    lo: u128,
    hi: u128,
}

impl Board {
    #[inline]
    #[must_use]
    pub fn get(&self, x: usize, y: usize) -> bool {
        if x >= 4 || y >= 64 {
            return false;
        }
        if y < 32 {
            // bottom half
            (self.lo >> (y * 4 + x)) & 1 != 0
        } else {
            // top half
            (self.hi >> ((y - 32) * 4 + x)) & 1 != 0
        }
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        if x >= 4 || y >= 64 {
            return;
        }

        let bit_index = (y % 32) * 4 + x;
        if y < 32 {
            if value {
                self.lo |= 1 << bit_index;
            } else {
                self.lo &= !(1 << bit_index);
            }
        } else if value {
            self.hi |= 1 << bit_index;
        } else {
            self.hi &= !(1 << bit_index);
        }
    }

    #[inline]
    pub fn num_minos(&self) -> u16 {
        (self.lo.count_ones() + self.hi.count_ones()) as u16
    }

    pub fn skim(&mut self) {
        let mut rows = [0u8; 64];

        // extract rows: bottom 32 -> lo, top 32 -> hi
        for y in 0..32 {
            rows[y] = ((self.lo >> (y * 4)) & 0b1111) as u8; // bottom half
            rows[32 + y] = ((self.hi >> (y * 4)) & 0b1111) as u8; // top half
        }

        let mut new_rows = [0u8; 64];
        let mut write = 0; // start writing from the bottom
        for item in rows {
            if item != 0b1111 {
                new_rows[write] = item;
                write += 1;
            }
        }

        // rebuild lo and hi from new_rows
        self.lo = 0;
        self.hi = 0;
        for y in 0..32 {
            self.lo |= (new_rows[y] as u128) << (y * 4);
            self.hi |= (new_rows[32 + y] as u128) << (y * 4);
        }
    }
    pub const fn width(&self) -> usize {
        4
    }

    pub fn height(&self) -> usize {
        // scan top half first
        for row in (0..32).rev() {
            if (self.hi >> (row * 4)) & 0b1111 != 0 {
                return row + 1 + 32; // row index + 1, counting rows
            }
        }

        // then bottom half
        for row in (0..32).rev() {
            if (self.lo >> (row * 4)) & 0b1111 != 0 {
                return row + 1; // row index + 1
            }
        }

        0 // completely empty
    }

    pub fn small(&self) -> String {
        use std::fmt::Write;
        let mut seen = false;

        let mut v = vec![];
        let write_half = move |half: u128, seen: &mut bool, v: &mut Vec<String>| {
            for row in (0..32).rev() {
                let empty = (half >> (row * 4)) & 0b1111;
                if empty != 0 || *seen {
                    *seen = true;
                    let mut z = String::new();
                    for col in 0..4 {
                        let bit = (half >> (row * 4 + col)) & 1;
                        write!(z, "{}", if bit == 1 { 'X' } else { '_' }).unwrap();
                    }

                    v.push(z);
                }
            }
        };

        write_half(self.hi, &mut seen, &mut v);
        write_half(self.lo, &mut seen, &mut v);
        v.join("|")
    }

    #[must_use]
    pub fn get_next_boards(
        &self,
        piece: PieceRef,
        environment: &Environment,
    ) -> Vec<(Self, Finesse)> {
        let mut queue = VecDeque::new();
        let mut visited_active = HashMap::new(); // Track active states
        let mut final_placements = HashMap::new(); // Track final placements

        let available_keys = environment.keyboard();

        // Initial active state (not placed)
        let initial_state = {
            let i = Input::new(*self, piece, environment);
            i.fingerprint()
        };

        let initial_finesse = Finesse::new();
        queue.push_back(initial_finesse);
        visited_active.insert(initial_state, initial_finesse);

        while let Some(input_seq) = queue.pop_front() {
            // Check if this sequence leads to a new final placement
            let placed_state = {
                let mut i = Input::new(*self, piece, environment);
                i.apply(input_seq);
                i.place(true)
            };
            final_placements.entry(placed_state).or_insert(input_seq);

            for key in &available_keys {
                let mut new_seq = input_seq;
                new_seq.push(*key);

                // Get the ACTIVE state (not placed)
                let next_active_state = {
                    let mut i = Input::new(*self, piece, environment);
                    i.apply(new_seq);
                    i.fingerprint()
                };

                if visited_active.contains_key(&next_active_state) {
                    continue;
                }

                visited_active.insert(next_active_state, new_seq);
                queue.push_back(new_seq);
            }
        }

        final_placements.into_iter().collect()
    }
}
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.lo == 0 && self.hi == 0 {
            writeln!(f, "<empty>")?;
            return Ok(());
        }
        let mut seen = false;

        let write_half =
            |half: u128, f: &mut std::fmt::Formatter<'_>, seen: &mut bool| -> std::fmt::Result {
                for row in (0..32).rev() {
                    let empty = (half >> (row * 4)) & 0b1111;
                    if empty != 0 || *seen {
                        *seen = true;
                        write!(f, "|")?;
                        for col in 0..4 {
                            let bit = (half >> (row * 4 + col)) & 1;
                            write!(f, "{}", if bit == 1 { 'X' } else { '_' })?;
                        }
                        writeln!(f, "|")?;
                    }
                }
                Ok(())
            };

        write_half(self.hi, f, &mut seen)?;
        write_half(self.lo, f, &mut seen)?;
        Ok(())
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hi: u128 = 0;
        let mut lo: u128 = 0;

        let rows: Vec<&str> = s.split('|').filter(|r| !r.is_empty()).collect();
        if rows.len() > 64 {
            return Err("too many rows".to_string());
        }

        let (hi_rows, lo_rows) = if rows.len() <= 32 {
            (Vec::new(), rows)
        } else {
            let split = rows.len() - 32;
            (rows[..split].to_vec(), rows[split..].to_vec())
        };
        let encode = |rows: &[&str], target: &mut u128| -> Result<(), String> {
            for (i, &line) in rows.iter().rev().enumerate() {
                if line.len() != 4 {
                    return Err(format!("invalid row: {}", line));
                }
                let row = i; // row 0 = bottom of half
                for (col, ch) in line.chars().enumerate() {
                    match ch {
                        'X' => *target |= 1 << (row * 4 + col),
                        '_' => {}
                        _ => return Err(format!("invalid char: {}", ch)),
                    }
                }
            }
            Ok(())
        };
        encode(&hi_rows, &mut hi)?;
        encode(&lo_rows, &mut lo)?;

        Ok(Board { hi, lo })
    }
}
impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{self:#}")
        } else {
            f.debug_tuple("Board")
                .field(&self.lo)
                .field(&self.hi)
                .finish()
        }
    }
}

use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use crate::{
    environment::Environment,
    input::{Finesse, Input},
    piece::PieceRef,
};
