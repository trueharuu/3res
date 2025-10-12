use crate::piece::PieceTy;

pub struct Rng {
    pub seed: i32,
    last_generated: Option<usize>,
    bagid: usize,
    extrabag: Vec<char>,
}

impl Rng {
    #[must_use]
    pub fn new(mut seed: i32) -> Self {
        if seed <= 0 {
            seed += 2_147_483_646;
        }
        Self {
            seed,
            last_generated: None,
            bagid: 0,
            extrabag: vec![],
        }
    }

    pub const BAG: [char; 7] = ['Z', 'L', 'O', 'S', 'I', 'J', 'T'];

    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> i32 {
        self.seed = 16_807 * self.seed % 2_147_483_647;
        self.seed
    }

    pub fn next_float(&mut self) -> f64 {
        f64::from(self.next() - 1) / 2_147_483_646f64
    }

    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    pub fn shuffle_array<'a, T>(&mut self, array: &'a mut [T]) -> &'a mut [T] {
        if array.is_empty() {
            return array;
        }

        for i in array.len()..0 {
            let r = self.next_float() * (i + 1) as f64;
            array.swap(i, r as usize);
        }

        array
    }

    #[allow(const_item_mutation)]
    pub fn next_item(&mut self, randomizer: Randomizer) -> Vec<PieceTy> {
        match randomizer {
            Randomizer::Bag7 => self.shuffle_array(&mut Self::BAG).to_vec(),
            Randomizer::Bag14 => self
                .shuffle_array(&mut [
                    'Z', 'L', 'O', 'S', 'I', 'J', 'T', 'Z', 'L', 'O', 'S', 'I', 'J', 'T',
                ])
                .to_vec(),
            Randomizer::Classic => {
                let tet = Self::BAG;

                let mut idx = (self.next_float() * ((tet.len() + 1) as f64)).floor() as usize;
                if Some(idx) == self.last_generated || idx >= tet.len() {
                    idx = (self.next_float() * (tet.len() as f64)).floor() as usize;
                }

                self.last_generated = Some(idx);
                vec![tet[idx]]
            }
            Randomizer::Pairs => {
                let mut z = ['Z', 'L', 'O', 'S', 'I', 'J', 'T'];
                let s = self.shuffle_array(&mut z);
                let mut pairs = [s[0], s[0], s[0], s[1], s[1], s[1]];
                self.shuffle_array(&mut pairs).to_vec()
            }
            Randomizer::Bag7P1 => {
                let t: &[char] = &Self::BAG;
                let extra = t[(self.next_float() * 7.0).floor() as usize];
                let mut t2 = [t, &[extra]].concat();
                let bag = self.shuffle_array(&mut t2);
                bag.to_vec()
            }
            Randomizer::Bag7P2 => {
                let t: &[char] = &Self::BAG;
                let extr1 = t[(self.next_float() * 7.0).floor() as usize];
                let extr2 = t[(self.next_float() * 7.0).floor() as usize];

                let mut t2: Vec<char> = [t, &[extr1, extr2]].concat();
                let bag = self.shuffle_array(&mut t2);
                bag.to_vec()
            }
            Randomizer::Bag7PX => {
                let extra_piece_count = [3, 2, 1, 1];
                let extra = extra_piece_count
                    .get(self.bagid)
                    .copied()
                    .unwrap_or_default();
                self.bagid += 1;
                if self.extrabag.len() < extra {
                    self.extrabag = self.shuffle_array(&mut Self::BAG).to_vec();
                }

                self.shuffle_array(&mut [&Self::BAG, &self.extrabag[0..extra]].concat())
                    .to_vec()
            }
            Randomizer::TotalMayhem => {
                vec![Self::BAG[(self.next_float() * Self::BAG.len() as f64).floor() as usize]]
            }
        }
    }

    // given a `hint` and `randomizer`,
    // determine all possible sequences that could come *after*
    // the `hint` while placying on this `randomizer`
    // for example if hint is `ZSSIJLO`, we know either `TZ` or `ZT` must come after it
    // on Bag7 randomizer.
    pub fn guess(hint: &[char], len: usize, randomizer: Randomizer) -> Vec<Vec<char>> {
        let _ = (hint, len, randomizer);
        todo!();
    }
}

pub enum Randomizer {
    Bag7,
    Bag14,
    Bag7P1,
    Bag7P2,
    Bag7PX,
    Classic,
    Pairs,
    TotalMayhem,
}
