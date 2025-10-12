use criterion::{Criterion, criterion_group, criterion_main};
use engine::{
    environment::{DropType, Environment},
    file::{corners::Corners, kicks::Kicks, piece::Bag},
    repl::State,
};

pub fn get_next_bm(c: &mut Criterion) {
    // c.bench_function("get_pos", |b| {
    //     let mut r = Rng::new(0);
    //     b.iter_batched(
    //         || {
    //             let lo: u128 =
    //                 unsafe { std::mem::transmute([r.next(), r.next(), r.next(), r.next()]) };
    //             let hi: u128 =
    //                 unsafe { std::mem::transmute([r.next(), r.next(), r.next(), r.next()]) };

    //             let x = r.next();
    //             let y = r.next();
    //             (Board { lo, hi }, ((x % 4) as usize, (y % 64) as usize))
    //         },
    //         |(b, (x, y))| black_box(b.get(x % 4, y % 64)),
    //         criterion::BatchSize::SmallInput,
    //     );
    // });
    c.bench_function("pc_gen", |_b| {
        let k_file = std::fs::read_to_string("data/srsx.kick").unwrap();
        let b_file = std::fs::read_to_string("data/tetromino.piece").unwrap();
        let c_file = std::fs::read_to_string("data/handheld.corners").unwrap();

        let kicks: Kicks = k_file.parse().unwrap();
        let bag: Bag = b_file.parse().unwrap();
        let corners: Corners = c_file.parse().unwrap();
        let env: Environment = Environment {
            droptype: DropType::Sonic,
            vision: 7,
            foresight: 1,
            can_180: true,
            can_das: true,
            can_tap: true,
            can_hold: true,
            upstack: true,
            state: &State {
                kicks,
                bag,
                corners,
                fingerprint: ("srsx".into(), "tetromino".into(), "handheld".into()),
            },
        };

        let _ = env;

        // b.iter(|_| generate_all_pc_queues(4, &env));
    });
    // c.bench_function("get_next", |b| {
    //     let mut r = Rng::new(0);
    //     let k_file = std::fs::read_to_string("data/srsx.kick").unwrap();
    //     let b_file = std::fs::read_to_string("data/tetromino.piece").unwrap();
    //     let c_file = std::fs::read_to_string("data/handheld.corners").unwrap();

    //     let kicks: Kicks = k_file.parse().unwrap();
    //     let bag: Bag = b_file.parse().unwrap();
    //     let corners: Corners = c_file.parse().unwrap();
    //     let env: Environment = Environment {
    //         bag,
    //         kicks,
    //         corners,
    //         droptype: DropType::Sonic,
    //         vision: 7,
    //         foresight: 1,
    //         can_180: true,
    //         can_das: true,
    //         can_tap: true,
    //         can_hold: true,
    //         upstack: true,
    //     };

    //     b.iter_batched(
    //         move || {
    //             (
    //                 *r.pick(&['T', 'I', 'L', 'J', 'O', 'S', 'Z']),
    //                 Board::from_str("XX_X").unwrap(),
    //             )
    //         },
    //         |i| black_box(i.1.get_next_boards(black_box(i.0), black_box(&env))),
    //         criterion::BatchSize::SmallInput,
    //     );
    // });
}

criterion_group!(benches, get_next_bm);
criterion_main!(benches);
