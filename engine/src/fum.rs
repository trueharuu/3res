use fumen::{CellColor, Fumen, Page};
use itertools::Itertools;

use crate::ren::PathItem;

pub fn tofumen(t: Vec<PathItem>) -> Fumen {
    Fumen {
        guideline: true,
        pages: t
            .into_iter()
            .map(|p| Page {
                comment: Some(format!("{} {}", p.1, p.2.short())),
                garbage_row: [CellColor::Empty; 10],
                lock: false,
                piece: None,
                mirror: false,
                rise: false,
                field: (0..23)
                    .map(|y| {
                        (0..10)
                            .map(|x| {
                                if p.0.get(x, y) {
                                    CellColor::Grey
                                } else {
                                    CellColor::Empty
                                }
                            })
                            .collect_array::<10>()
                            .unwrap()
                    })
                    .collect_array::<23>()
                    .unwrap(),
            })
            .collect(),
    }
}
