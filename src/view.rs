use std::sync::{Arc, Mutex};
use ncurses::*;

pub trait Renderable {
    fn render(&self, view: &View);
}

pub struct View {
    pub x: i32,
    pub y: i32,
    objects: Vec<Arc<Mutex<Renderable>>>,
}

impl View {
    pub fn new(x:i32, y:i32, objects: Vec<Arc<Mutex<Renderable>>>) -> Self {
        View { x, y, objects }
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
    fn render(&self, view: &View) {
        for o in &self.objects {
            let o = o.lock().unwrap();
            o.render(view);
        }
    }
}
