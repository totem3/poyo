use ncurses::*;
use std::thread;
use observer::MutObserver;
use observable::{MutObservable, MutObserverRef};
use event::Event;

pub struct Input {
    observers: Vec<MutObserverRef>,
}

impl Input {
    pub fn new() -> Self {
        Input { observers: vec![] }
    }

    pub fn run(self) {
        let t = thread::spawn(move || {
                                  let mut ch = getch();
                                  while ch != 0x71 {
                                      self.notify(&Event::Input(ch));
                                      ch = getch();
                                  }
                                  self.notify(&Event::Exit);
                              });
        t.join();
    }

    fn notify(&self, event: &Event) {
        for observer in &self.observers {
            if let Some(observer) = observer.upgrade() {
                let mut o = observer.lock().unwrap();
                o.notify_mut(event);
            }
        }
    }
}

impl MutObservable for Input {
    fn register_mut(&mut self, observer: MutObserverRef) {
        self.observers.push(observer);
    }
}
