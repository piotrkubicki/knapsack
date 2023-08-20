use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc::{Sender, channel, Receiver};
use std::thread::{self, JoinHandle};

pub struct Writter {
    tx: Sender<Vec<String>>,
}

impl Writter {

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

    pub fn register(&self, filename: &str, headers: Vec<&str>) -> Sender<Vec<String>> {
        let mut f = Writter::open_file(filename);
        let _  = writeln!(&mut f, "{}", headers.join(", "));

        self.tx.clone()
    }

    pub fn run() -> (Self, JoinHandle<()>) {
        let (tx, rx): (Sender<Vec<String>>, Receiver<Vec<String>>) = channel();

        let handle = thread::spawn(move || {
            loop {
                if let Ok(mut msg) = rx.recv() {
                    let filename = msg.remove(0);
                    let mut f = Writter::open_file(&filename);

                    let _ = writeln!(&mut f, "{}", msg.join(", "));
                } else {
                    break;
                }
            }
        });

        (Writter{ tx }, handle)
    }
}
