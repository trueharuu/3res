use std::{
    io::{BufRead, BufReader, Read, Write},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
};

use crate::{
    environment::Environment,
    file::{corners::Corners, kicks::Kicks, piece::Bag},
    pc::max_pcs_in_queue,
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
            "pcr" => {
                let e = Environment::new(
                    &s,
                    argv.next().unwrap(),
                    argv.next().unwrap().parse().unwrap(),
                    0,
                );

                let queue = argv.next().unwrap();
                let pcs = e.pcs(argv.next().unwrap().parse().unwrap());

                let chosen = max_pcs_in_queue(&queue.chars().collect::<Vec<_>>(), &e, pcs);

                chosen
                    .first()
                    .unwrap()
                    .clone()
                    .0
                    .iter()
                    .map(|x| {
                        format!(
                            "{}:{}",
                            x.0,
                            x.1.into_iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<_>>()
                                .join(",")
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            }
            "pcp" => {
                let e = Environment::new(
                    &s,
                    argv.next().unwrap(),
                    argv.next().unwrap().parse().unwrap(),
                    0,
                );

                e.pcs(argv.next().unwrap().parse().unwrap());
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
