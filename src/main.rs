extern crate ncurses;
extern crate rand;

mod cli;
mod color;
mod direction;
mod event;
mod field;
mod observer;
mod observable;
mod position;
mod poyo;
mod poyopoyo;
mod size;
mod input;

use input::Input;
use event::Event;
use observable::MutObservable;
use observer::MutObserver;
use std::sync::{Arc, Mutex};
use std::thread;
use field::Field;
use direction::Direction;
use size::Size;
use poyopoyo::PoyoPoyo;
use std::time::SystemTime;

struct Main {
    done: bool,
    field: Field,
    view: Arc<Mutex<cli::CliView>>,
}

fn main() {
    let mut input = Input::new();
    let mut field = Field::default();
    let pp = PoyoPoyo::rand();
    field.set_current(pp);
    let cv = cli::CliView::new(Size::new(field.width() + 2, field.height() + 2));
    let cv = Arc::new(Mutex::new(cv));
    let weak_ref_cv = Arc::downgrade(&cv);
    field.register_mut(weak_ref_cv);
    let m = Main {
        done: false,
        view: cv,
        field: field,
    };
    let acm = Arc::new(Mutex::new(m));
    let _acm = acm.clone();
    let wm = Arc::downgrade(&_acm);
    input.register_mut(wm);
    {
        let m = acm.lock().unwrap();
        m.on_init();
    }
    {
        // main
        let m = acm.clone();
        let mut time = SystemTime::now();
        let mut frame = 0;
        let mut tick = 0;
        thread::spawn(move || loop {
                          match m.lock() {
                              Ok(mut m) => {
                if m.done {
                    break;
                }
                m.on_frame();
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
                    m.on_frame();
                    frame = 0;
                }
                if tick > 1000 * 1000000 {
                    m.tick();
                    tick = 0;
                }
            }
                              Err(_) => {}
                          }
                      });
    }
    input.run();
    {
        let m = acm.lock().unwrap();
        m.on_exit();
    }
}

impl Main {
    fn on_init(&self) {
        match self.view.try_lock() {
            Ok(v) => v.init(),
            Err(_) => {
                panic!("failed to get lock");
            }
        }
        self.field.on_init();
    }

    fn on_frame(&self) {
        match self.view.try_lock() {
            Ok(v) => v.draw(),
            Err(_) => {}
        }
    }

    fn on_exit(&self) {
        match self.view.try_lock() {
            Ok(v) => v.exit(),
            Err(_) => {
                panic!("failed to get lock");
            }
        }
    }

    fn tick(&mut self) {
        if self.field.current_can_move(&Direction::Down) {
            self.field.move_current(Direction::Down);
        } else {
            self.field.fix_current();
        }
    }
}


impl MutObserver for Main {
    fn notify_mut(&mut self, event: &Event) {
        match event {
            &Event::Exit => self.done = true,
            &Event::Input(i) => {
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
