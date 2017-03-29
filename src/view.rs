use std::sync::{Arc, Mutex};
use ncurses::*;

pub trait Renderable {
    fn render(&self);
}

pub struct View {
    objects: Vec<Arc<Mutex<Renderable>>>,
}

impl View {
    pub fn new(objects: Vec<Arc<Mutex<Renderable>>>) -> Self {
        View { objects }
    }

    pub fn init_view() {
        initscr();
        raw();
        keypad(stdscr(), true);
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        refresh();
    }
}

impl Renderable for View {
    fn render(&self) {
        for o in &self.objects {
            let o = o.lock().unwrap();
            o.render();
        }
    }
}
