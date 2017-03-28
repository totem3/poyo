use ncurses::*;
use view::Renderable;
use chr::Color;

#[derive(Debug)]
pub struct Board {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    rows: Vec<Vec<Color>>,
}

impl Board {
    pub fn new(width:i32,height:i32,x:i32,y:i32,rows:Vec<Vec<Color>>) -> Self {
        Board {
            width,
            height,
            x,
            y,
            rows
        }
    }
    pub fn rectangle(&self) -> (i32, i32, i32, i32) {
        (self.topmost(), self.rightmost(), self.bottommost(), self.leftmost())
    }
    pub fn leftmost(&self) -> i32 {
        self.x
    }

    pub fn rightmost(&self) -> i32 {
        self.x + self.width - 1
    }

    pub fn topmost(&self) -> i32 {
        self.y
    }

    pub fn bottommost(&self) -> i32 {
        self.y + self.height - 1
    }

    fn create(&self) {
        let win = newwin(self.height, self.width, self.y, self.x);
        box_(win, 0, 0);
        wrefresh(win);
    }

    fn destroy(&self) {
        let ch = ' ' as chtype;
        let win = newwin(self.height, self.width, self.y, self.x);
        wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
        wrefresh(win);
        delwin(win);
    }
}

impl Renderable for Board {
    fn render(&self) {
        self.destroy();
        self.create();
    }
}
