use std::{
    io::{BufRead, BufReader, Read, Write},
    str::FromStr,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::Instant,
};

use crate::{
    board::Board,
    environment::Environment,
    file::{corners::Corners, kicks::Kicks, piece::Bag},
    input::{Finesse, Input, Key},
    pc::max_pcs_in_queue,
    ren::{Node, PathItem, ren_bfs},
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct State {
    pub kicks: Kicks,
    pub bag: Bag,
    pub corners: Corners,
    pub fingerprint: (String, String, String),
}
pub struct Repl<I, O> {
    pub i: I,
    pub o: O,
    pub state: State,
}

pub struct ReplHandle {
    running: Arc<AtomicBool>,
    pub handle: JoinHandle<()>,
}

impl<I, O> Repl<I, O>
where
    I: Read + Send + 'static,
    O: Write + Send + 'static,
{
    pub fn new(i: I, o: O, state: State) -> Self {
        Self { i, o, state }
    }

    pub fn spawn(self) -> ReplHandle {
        let running = Arc::new(AtomicBool::new(true));
        let run_clone = running.clone();

        let handle = thread::spawn(move || {
            let mut reader = BufReader::new(self.i);
            let mut writer = self.o;
            let mut line = String::new();

            while run_clone.load(Ordering::SeqCst) {
                line.clear();

                let bytes = match reader.read_line(&mut line) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => n,
                };

                // eprintln!("ok {bytes}");

                if bytes > 0 {
                    let trimmed = line.trim_end();
                    let response = Self::respond(self.state.clone(), trimmed);
                    let _ = writeln!(writer, "{response}");
                    let _ = writer.flush();
                }
            }
        });

        ReplHandle { running, handle }
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn respond(s: State, arg: &str) -> String {
        let mut argv = arg.split_ascii_whitespace();
        let Some(ma) = argv.next() else {
            return "?".to_string();
        };
        match ma {
            "pc" => {
                // let b = argv.next().unwrap();
                let queue: Vec<char> = argv.next().unwrap().chars().collect();
                // let hold = argv
                //     .next()
                //     .filter(|x| *x != "_")
                //     .and_then(|x| x.chars().next());
                // let vision: usize = argv.next().unwrap().parse().unwrap();
                // let foresight: usize = argv.next().unwrap().parse().unwrap();

                let flags = argv.next().unwrap();

                let env: Environment = Environment {
                    droptype: crate::environment::DropType::Sonic,
                    vision: 0,
                    foresight: 0,
                    can_180: flags.contains('f'),
                    can_das: flags.contains('d'),
                    can_tap: flags.contains('t'),
                    can_hold: flags.contains('h'),
                    upstack: flags.contains('u'),
                    state: &s,
                };

                let pcs = env.pcs(4);
                let z = max_pcs_in_queue(&queue, &env, pcs);
                z.iter()
                    .map(|x| x.queue().collect::<String>())
                    .collect::<Vec<_>>()
                    .join(", ")
            }
            "pcgen" => {
                let n = argv.next().and_then(|x| x.parse().ok()).unwrap();
                let env: Environment = Environment {
                    droptype: crate::environment::DropType::Sonic,
                    vision: 0,
                    foresight: 0,
                    can_180: true,
                    can_das: true,
                    can_tap: true,
                    can_hold: true,
                    upstack: true,
                    state: &s,
                };

                let i = Instant::now();
                let _ = env.pcs(n);
                let e = i.elapsed();

                // println!("{q:#?}");
                println!("{}ms", e.as_millis());

                String::new()
            }
            "ren" => {
                let b = argv.next().unwrap();
                let queue: Vec<char> = argv.next().unwrap().chars().collect();
                let hold = argv
                    .next()
                    .filter(|x| *x != "_")
                    .and_then(|x| x.chars().next());
                let vision: usize = argv.next().unwrap().parse().unwrap();
                let foresight: usize = argv.next().unwrap().parse().unwrap();

                let flags = argv.next().unwrap();

                let env: Environment = Environment {
                    droptype: crate::environment::DropType::Sonic,
                    vision,
                    foresight,
                    can_180: flags.contains('f'),
                    can_das: flags.contains('d'),
                    can_tap: flags.contains('t'),
                    can_hold: flags.contains('h'),
                    upstack: flags.contains('u'),
                    state: &s,
                };

                let board: Board = b.parse().unwrap();
                let init = Node {
                    board,
                    hold,
                    queue: &queue[0..env.vision.min(queue.len())],
                    prev: None,
                    finesse: Finesse::new(),
                    used: None,
                    ptr: 0,
                };

                let p = ren_bfs(&init, &env);
                println!("{p:?}");
                for i in p {
                    for PathItem(n, c, f) in i {
                        println!("{}({} {})", n, c, f);
                    }
                    println!();
                }

                String::new()
            }

            "branch" => {
                let b = argv.next().unwrap();
                let queue: Vec<char> = argv.next().unwrap().chars().collect();
                let hold = argv
                    .next()
                    .filter(|x| *x != "_")
                    .and_then(|x| x.chars().next());
                let vision: usize = argv.next().unwrap().parse().unwrap();
                let foresight: usize = argv.next().unwrap().parse().unwrap();

                let flags = argv.next().unwrap();

                let env: Environment = Environment {
                    droptype: crate::environment::DropType::Sonic,
                    vision,
                    foresight,
                    can_180: flags.contains('f'),
                    can_das: flags.contains('d'),
                    can_tap: flags.contains('t'),
                    can_hold: flags.contains('h'),
                    upstack: flags.contains('u'),
                    state: &s,
                };

                let board: Board = b.parse().unwrap();
                let init = Node {
                    board,
                    hold,
                    queue: &queue,
                    prev: None,
                    finesse: Finesse::new(),
                    used: None,
                    ptr: 0,
                };

                let n = init.neighbors(&env);
                let _ = n;

                String::new()
            }

            "id" => {
                let mut board: Board = argv.next().unwrap().parse().unwrap();
                board.skim();
                format!("{board} {}", board.height())
            }
            "send" => {
                let board: Board = argv.next().unwrap().parse().unwrap();
                let piece = argv.next().and_then(|x| x.chars().next()).unwrap();
                let flags = argv.next().unwrap();

                let env: Environment = Environment {
                    droptype: crate::environment::DropType::Sonic,
                    vision: 7,
                    foresight: 1,
                    can_180: flags.contains('f'),
                    can_das: flags.contains('d'),
                    can_tap: flags.contains('t'),
                    can_hold: flags.contains('h'),
                    upstack: flags.contains('u'),
                    state: &s,
                };

                let mut i = Input::new(board, piece, &env);

                let keys = argv
                    .next()
                    .and_then(|x| {
                        x.split(",")
                            .map(|x| x.parse())
                            .collect::<Result<Vec<Key>, _>>()
                            .ok()
                    })
                    .unwrap_or_default();
                let z = Finesse::with(&keys);
                i.apply(z);
                println!(
                    "{} {} fills: {}",
                    i.piece.location,
                    i.piece.rotation,
                    i.piece
                        .cells(&env)
                        .map(|x| x.unwrap().to_string())
                        .collect::<Vec<_>>()
                        .join("")
                );

                let r = i.place(false);
                format!("{r}{}", z.short())
            }
            "next" => {
                let board: Board = argv.next().unwrap().parse().unwrap();
                let piece = argv.next().and_then(|x| x.chars().next()).unwrap();
                let flags = argv.next().unwrap();

                let env: Environment = Environment {
                    droptype: crate::environment::DropType::Sonic,
                    vision: 7,
                    foresight: 1,
                    can_180: flags.contains('f'),
                    can_das: flags.contains('d'),
                    can_tap: flags.contains('t'),
                    can_hold: flags.contains('h'),
                    upstack: flags.contains('u'),
                    state: &s,
                };

                println!("keyboard = {:?}", env.keyboard());

                let z = board.get_next_boards(piece, &env);
                for (i, f) in z {
                    println!("{i}[{}]\n", f.short());
                }
                String::new()
            }
            "test" => {
                let b = Board::from_str("XX_X|X___").unwrap();
                println!("{b}");
                String::new()
            }
            // "ex" => std::process::abort(),
            _ => "?".to_string(),
        }
    }
}

impl ReplHandle {
    pub fn kill(self) {
        self.running.store(false, Ordering::SeqCst);
        let _ = self.handle.join();
    }
}
