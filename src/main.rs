extern crate ncurses;
extern crate rand;

mod cli;
mod color;
mod direction;
mod event;
mod field;
mod game_state;
mod input;
mod position;
mod poyo;
mod poyopoyo;
mod size;

use direction::Direction;
use event::Event;
use field::Field;
use game_state::GameState;
use input::Input;
use poyopoyo::PoyoPoyo;
use size::Size;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::SystemTime;

struct Main {
    state: GameState,
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
    let input = Input::new(tx.clone());
    let mut m = Main::new(cv, field, rx);
    m.on_init();
    {
        let tx = tx.clone();
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
            // if frame > 150 * 1000000 {
            if frame > 10 * 1000000 {
                let _ = tx.send(Event::FrameUpdate);
                frame = 0;
            }
            if tick > 1000 * 1000000 {
                let _ = tx.send(Event::Tick);
                tick = 0;
            }
        });
    }
    input.run();
    m.main();
    m.on_exit();
}

impl Main {
    fn new(cv: cli::CliView, field: Field, rx: Receiver<Event>) -> Self {
        Main {
            state: GameState::Start,
            done: false,
            view: cv,
            field: field,
            rx: rx,
        }
    }
    fn on_init(&mut self) {
        self.view.init();
        self.field.on_init();
    }

    fn on_frame(&mut self) {
        // self.view.update();
        self.view.draw(&self.state);
    }

    fn on_exit(&self) {
        self.view.exit();
    }

    fn tick(&mut self) {
        if self.field.current_can_move(&Direction::Down) {
            self.field.move_current(Direction::Down);
        } else {
            if let Some((removed_count, (left, top))) = self.field.fix_current() {
                if removed_count == 0 && left == 1 && top == 0 {
                    self.state = GameState::GameOver;
                }
            }
        }
    }

    fn main(&mut self) {
        while !self.done {
            match self.rx.try_recv() {
                Ok(Event::MovePoyo(poyos)) => match self.state {
                    GameState::Start => {}
                    GameState::Playing { poyos: _ } => {
                        self.state = GameState::Playing { poyos };
                    }
                    GameState::GameOver => {}
                },
                Ok(Event::Tick) => self.tick(),
                Ok(Event::FrameUpdate) => self.on_frame(),
                Ok(Event::Exit) => self.done = true,
                Ok(Event::Input(i)) => match i {
                    ncurses::KEY_LEFT => self.field.move_current(Direction::Left),
                    ncurses::KEY_RIGHT => self.field.move_current(Direction::Right),
                    ncurses::KEY_DOWN => self.field.move_current(Direction::Down),
                    0x20 => match self.state {
                        GameState::Start => {
                            self.state = GameState::Playing { poyos: vec![] };
                        }
                        GameState::Playing { poyos: _ } => {
                            self.field.rotate_current();
                        }
                        GameState::GameOver => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
