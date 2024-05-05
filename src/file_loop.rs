use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::{
    self, Receiver, Sender,
    TryRecvError::{Disconnected, Empty},
};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};
use std::time::Instant;

use lazy_static::lazy_static;

static mut ATOM: AtomicU64 = AtomicU64::new(0);

lazy_static! {
    static ref TIME: Instant = Instant::now();
}

type Job = Box<dyn FnOnce() + 'static + Send>;

enum Jobber {
    Finish,
    NewJob(Job),
}

struct Worker {
    t: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(rx: Arc<Mutex<Receiver<Jobber>>>) -> Self {
        Self {
            t: Some(thread::spawn(move || loop {
                match rx.lock().unwrap().recv().unwrap() {
                    Jobber::Finish => break,
                    Jobber::NewJob(job) => job(),
                }
            })),
        }
    }
}

struct Pool {
    lengths: usize,
    workers: Vec<Worker>,
    senders: Sender<Jobber>,
}

impl Pool {
    fn new(size: usize) -> Self {
        let (tx, rx): (Sender<Jobber>, Receiver<Jobber>) = mpsc::channel();

        let recv: Arc<Mutex<Receiver<Jobber>>> = Arc::new(Mutex::new(rx));

        let mut workers: Vec<Worker> = Vec::with_capacity(size);

        (0..size).for_each(|_| workers.push(Worker::new(Arc::clone(&recv))));

        Self {
            lengths: size,
            workers: workers,
            senders: tx,
        }
    }

    fn execute<F>(&self, f: F)
        where
            F: FnOnce() + 'static + Send,
    {
        let job: Jobber = Jobber::NewJob(Box::new(f));
        self.senders.send(job).unwrap();
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        (0..self.lengths).for_each(|_| {
            self.senders.send(Jobber::Finish).unwrap();
        });
        self.workers.iter_mut().for_each(|w| {
            if let Some(t) = w.t.take() {
                t.join().unwrap();
            }
        });
        println!("pool droped after {}", TIME.elapsed().as_millis());
    }
}

fn walker(p: PathBuf, tx: Sender<FP>) {
    if let Ok(entries) = fs::read_dir(p) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    tx.send(FP::Path(path)).unwrap();
                } else {
                    let path_str = path.to_str().unwrap();
                    if path_str.ends_with(".vue") {
                        tx.send(FP::File(PathBuf::from(path_str))).unwrap();
                    }
                }
            }
        }
    }
    unsafe {
        ATOM.fetch_sub(1, Ordering::Relaxed);
    }
}

fn file_reader(p: PathBuf) {
    fs::read_to_string(p).unwrap();
    unsafe {
        ATOM.fetch_sub(1, Ordering::Relaxed);
    }
}

enum FP {
    File(PathBuf),
    Path(PathBuf),
}

fn main() {
    loop {
        if !TIME.elapsed().is_zero() {
            break;
        }
    }

    unsafe {
        ATOM.fetch_add(1, Ordering::Relaxed);
    }

    let pool: Pool = Pool::new(4);

    let (tx, rx): (Sender<FP>, Receiver<FP>) = mpsc::channel();

    pool.execute({
        let tx: Sender<FP> = tx.clone();
        move || walker(PathBuf::from("dzhr"), tx)
    });

    loop {
        match rx.try_recv() {
            Ok(fp) => match fp {
                FP::File(file) => {
                    unsafe {
                        ATOM.fetch_add(1, Ordering::Relaxed);
                    }
                    pool.execute(|| file_reader(file));
                }
                FP::Path(path) => {
                    unsafe {
                        ATOM.fetch_add(1, Ordering::Relaxed);
                    }
                    pool.execute({
                        let tx = tx.clone();
                        move || walker(path, tx)
                    });
                }
            },
            Err(error) => match error {
                Empty => unsafe {
                    if ATOM.load(Ordering::Relaxed) == 0 {
                        break;
                    }
                },

                Disconnected => {
                    break;
                }
            },
        }
    }
}
