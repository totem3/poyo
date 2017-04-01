use std::sync::{Arc, Mutex};
use ncurses::*;

static BOARD_HEIGHT: i32 = 12 + 2;
static BOARD_WIDTH: i32 = 6 + 2;


pub trait Renderable {
    fn render(&self, view: &View);
}

pub struct View {
    pub x: i32,
    pub y: i32,
    objects: Vec<Arc<Mutex<Renderable+Send>>>,
}

impl View {
    pub fn new(x:i32, y:i32, objects: Vec<Arc<Mutex<Renderable+Send>>>) -> Self {
        View { x, y, objects }
    }

    pub fn init_view() -> (i32, i32) {
        initscr();
        raw();
        keypad(stdscr(), true);
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        refresh();

        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);

        let start_x = (max_x - BOARD_WIDTH) / 2;
        let start_y = (max_y - BOARD_HEIGHT) / 2;
        (start_x, start_y)
    }
}

impl Renderable for View {
    fn render(&self, view: &View) {
        for o in &self.objects {
            let o = o.lock().unwrap();
            o.render(view);
            refresh();
        }
    }
}
