use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc::{Sender, channel, Receiver};
use std::thread::{self, JoinHandle};
use crate::Writter;

pub struct CsvWritter {
    tx: Sender<Vec<String>>,
}

impl CsvWritter {

    fn open_file(filename: &str) -> File {
        let f: File;
        let path = format!("./logs/{}", filename);
        if Path::new(&path).exists() {
            f = File::options().append(true).open(path).unwrap();
        } else {
            f = File::options().create_new(true).append(true).open(path).unwrap();
        }

        f
    }

    pub fn run() -> (Self, JoinHandle<()>) {
        let (tx, rx): (Sender<Vec<String>>, Receiver<Vec<String>>) = channel();

        let handle = thread::spawn(move || {
            loop {
                if let Ok(mut msg) = rx.recv() {
                    let filename = msg.remove(0);
                    let mut f = Self::open_file(&filename);

                    let _ = writeln!(&mut f, "{}", msg.join(", "));
                } else {
                    break;
                }
            }
        });

        (Self{ tx }, handle)
    }
}

impl Writter for CsvWritter {
    fn write(&self, data: Vec<String>) {
        let _ = self.tx.send(data);
    }
}
