use ncurses::*;
use view::{View,Renderable};
use yopi::{YopiYopi,Color};

#[derive(Debug)]
pub struct Board {
    width: i32,
    height: i32,
    pub rows: Vec<Vec<Option<Color>>>,
}

macro_rules! colored {
   ($c:expr => { $x:stmt }) => ( attron(COLOR_PAIR($c as i16)); $x; attroff(COLOR_PAIR($c as i16)););
}

impl Board {
    pub fn new(width:i32,height:i32,rows:Vec<Vec<Option<Color>>>) -> Self {
        let rows = vec![vec![None;6]; 12];
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
        0
    }

    pub fn bottommost(&self) -> i32 {
        self.height - 1
    }

    pub fn add(&mut self, p: &YopiYopi) {
        let mut p = p.clone();
        p.to_bottom(self);
        let (x, y) = (p.x()-1, p.y()-1);
        let (x2, y2) = (p.x2()-1, p.y2()-1);
        self.rows[y as usize][x as usize] = Some(p.colors().0);
        self.rows[y2 as usize][x2 as usize] = Some(p.colors().1);
    }

    pub fn is_filled(&self, x:i32, y:i32) -> bool {
        if x <= 0 || y <= 0 || x > 6 || y > 12 {
            return true;
        }
        self.rows[(y-1) as usize][(x-1) as usize].is_some()
    }

    fn create(&self, view: &View) {
        let win = newwin(self.height, self.width, view.y, view.x);
        box_(win, 0, 0);
        for (y, row) in self.rows.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if let &Some(c) = col {
                    colored!(c => {
                        mvprintw(y as i32 + 1 + view.y, x as i32 + 1 + view.x, c.symbol())
                    });
                }
            }
        }
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
