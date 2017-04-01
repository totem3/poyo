use ncurses::*;
use view::{View,Renderable};
use chr::Color;

#[derive(Debug)]
pub struct Board {
    width: i32,
    height: i32,
    rows: Vec<Vec<Color>>,
}

impl Board {
    pub fn new(width:i32,height:i32,rows:Vec<Vec<Color>>) -> Self {
        Board {
            width,
            height,
            rows
        }
    }
    pub fn rectangle(&self) -> (i32, i32, i32, i32) {
        (self.topmost(), self.rightmost(), self.bottommost(), self.leftmost())
    }
    pub fn leftmost(&self) -> i32 {
        0
    }

    pub fn rightmost(&self) -> i32 {
        self.width - 1
    }

    pub fn topmost(&self) -> i32 {
        9
    }

    pub fn bottommost(&self) -> i32 {
        self.height - 1
    }

    fn create(&self, view: &View) {
        let win = newwin(self.height, self.width, view.y, view.x);
        box_(win, 0, 0);
        wrefresh(win);
    }

    fn destroy(&self, view: &View) {
        let ch = ' ' as chtype;
        let win = newwin(self.height - 2, self.width - 2, view.y + 1, view.x + 1);
        wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
        wrefresh(win);
        delwin(win);
    }
}

impl Renderable for Board {
    fn render(&self, view:&View) {
        self.destroy(view);
        self.create(view);
    }
}
