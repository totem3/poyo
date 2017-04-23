extern crate ncurses;
extern crate rand;

mod cli;
mod color;
mod direction;
mod event;
mod field;
mod position;
mod poyo;
mod poyopoyo;
mod size;
mod input;

use input::Input;
use event::Event;
use std::sync::{Arc, Mutex};
use std::thread;
use field::Field;
use direction::Direction;
use size::Size;
use poyopoyo::PoyoPoyo;
use std::time::SystemTime;
use std::sync::mpsc::{channel, Sender, Receiver};

use std::io::{stderr, Write};

struct Main {
    done: bool,
    field: Field,
    view: cli::CliView,
    rx: Receiver<Event>,
}

fn main() {
    let (tx, rx) = channel();
    let mut field = Field::new(tx.clone(), Size::new(6, 12));
    let pp = PoyoPoyo::rand();
    field.set_current(pp);
    let cv = cli::CliView::new(Size::new(field.width() + 2, field.height() + 2));
    let mut input = Input::new(tx.clone());
    let mut m = Main {
        done: false,
        view: cv,
        field: field,
        rx: rx,
    };
    m.on_init();

    {
        let mut tx = tx.clone();
        let mut time = SystemTime::now();
        let mut frame = 0;
        let mut tick = 0;
        thread::spawn(move || loop {
                          let _time = SystemTime::now();
                          match _time.duration_since(time) {
                              Ok(diff) => {
                frame += diff.subsec_nanos();
                tick += diff.subsec_nanos();
                time = _time
            }
                              Err(_) => {}
                          };
                          if frame > 150 * 1000000 {
                              tx.send(Event::FrameUpdate);
                              frame = 0;
                          }
                          if tick > 1000 * 1000000 {
                              tx.send(Event::Tick);
                              tick = 0;
                          }
                      });
    }
    input.run();
    m.main();
    m.on_exit();
}

impl Main {
    fn on_init(&self) {
        self.view.init();
        self.field.on_init();
    }

    fn on_frame(&mut self) {
        // self.view.update();
        self.view.draw();
    }

    fn on_exit(&self) {
        self.view.exit();
    }

    fn tick(&mut self) {
        if self.field.current_can_move(&Direction::Down) {
            self.field.move_current(Direction::Down);
        } else {
            self.field.fix_current();
        }
    }

    fn main(&mut self) {
        while !self.done {
            match self.rx.try_recv() {
                Ok(Event::MovePoyo(poyos)) => self.view.update(poyos),
                Ok(Event::Tick) => self.tick(),
                Ok(Event::FrameUpdate) => self.on_frame(),
                Ok(Event::Exit) => self.done = true,
                Ok(Event::Input(i)) => {
                    writeln!(stderr(), "input {}", i);
                    match i {
                        ncurses::KEY_LEFT => self.field.move_current(Direction::Left),
                        ncurses::KEY_RIGHT => self.field.move_current(Direction::Right),
                        ncurses::KEY_DOWN => self.field.move_current(Direction::Down),
                        0x20 => self.field.rotate_current(),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}
