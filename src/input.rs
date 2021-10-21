use event::Event;
use ncurses::*;
use std::sync::mpsc::Sender;
use std::thread;

pub struct Input {
    tx: Sender<Event>,
}

impl Input {
    pub fn new(tx: Sender<Event>) -> Self {
        Input { tx: tx }
    }

    pub fn run(self) {
        let _ = thread::spawn(move || {
            let mut ch = getch();
            while ch != 0x71 {
                let _ = self.tx.send(Event::Input(ch));
                ch = getch();
            }
            let _ = self.tx.send(Event::Exit);
        });
    }
}
