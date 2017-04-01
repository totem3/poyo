use ncurses::*;
use view::{View, Renderable};
use board::Board;
use rand::random;
use std::cmp::{min,max};

const KEY_SPACE: i32 = 0x20;

#[derive(Debug,Clone)]
pub enum Orient {
    V,
    H,
    RV,
    RH,
}
#[derive(Clone)]
pub struct Chr {
    pub colors: (Color, Color),
    position: (i32, i32),
    orient: Orient,
}

impl Chr {
    pub fn new(colors: (Color, Color), position: (i32, i32), orient: Orient) -> Self {
        Chr {
            colors,
            position,
            orient
        }
    }

    pub fn rand() -> Self {
        let colors = (Color::rand(), Color::rand());
        let position = (2, 1);
        let orient = Orient::V;
        Chr::new(colors, position, orient)
    }
    pub fn x(&self) -> i32 {
        self.position.0
    }
    pub fn y(&self) -> i32 {
        self.position.1
    }
    pub fn x2(&self) -> i32 {
        match self.orient {
            Orient::V | Orient::RV => self.x(),
            Orient::H => self.x()+1,
            Orient::RH => self.x()-1,
        }
    }
    pub fn y2(&self) -> i32 {
        match self.orient {
            Orient::H | Orient::RH => self.y(),
            Orient::V => self.y()+1,
            Orient::RV => self.y()-1,
        }
    }
    pub fn left(&mut self) {
        self.position.0 -= 1;
    }
    pub fn right(&mut self) {
        self.position.0 += 1;
    }
    pub fn up(&mut self) {
        self.position.1 -= 1;
    }
    pub fn down(&mut self) {
        self.position.1 += 1;
    }
    pub fn leftmost(&self) -> i32 {
        min(self.x(),self.x2())
    }
    pub fn rightmost(&self) -> i32 {
        max(self.x(),self.x2())
    }
    pub fn topmost(&self) -> i32 {
        min(self.y(),self.y2())
    }
    pub fn bottommost(&self) -> i32 {
        max(self.y(),self.y2())
    }
    pub fn rotate(&mut self, board: &Board) {
        let (top, right, bottom, left) = board.rectangle();
        self.orient = match self.orient {
            Orient::V => Orient::H,
            Orient::H => Orient::RV,
            Orient::RV => Orient::RH,
            Orient::RH => Orient::V,
        };
        match self.orient {
            Orient::V => {
                if self.y() == bottom - 1 {
                    self.up();
                }
            }
            Orient::H => {
                if self.x() == right - 1 {
                    self.left();
                }
            }
            Orient::RV => {
                if self.y() == top + 1 {
                    self.down();
                }
            }
            Orient::RH => {
                if self.x() == left + 1 {
                    self.right();
                }
            }
        };
    }

    pub fn can_move_left(&self, board: &Board) -> bool {
        let x = self.leftmost() - 1;
        let y = self.bottommost();
        if x <= board.leftmost() || board.is_filled(x, y) {
            false
        } else {
            true
        }
    }
    pub fn can_move_right(&self, board: &Board) -> bool {
        let x = self.rightmost() + 1;
        let y = self.bottommost();
        if x >= board.rightmost() || board.is_filled(x, y){
            false
        } else {
            true
        }
    }
    pub fn can_move_down(&self, board: &Board) -> bool {
        let x = self.x();
        let y = self.bottommost() + 1;
        if y >= board.bottommost() || board.is_filled(x, y) {
            false
        } else {
            true
        }
    }

    pub fn is_bottom(&self, board: &Board) -> bool {
        !self.can_move_down(board)
    }

    pub fn moves(&mut self, input: i32, board: &Board) {
        match input {
            KEY_LEFT => {
                if self.can_move_left(board) {
                    self.left();
                }
            }
            KEY_RIGHT => {
                if self.can_move_right(board) {
                    self.right();
                }
            }
            KEY_DOWN => {
                if self.can_move_down(board) {
                    self.down();
                }
            }
            KEY_SPACE => {
                self.rotate(board);
            }
            _ => {},
        }
    }

    pub fn replace(&mut self, other: Self) {
        self.colors = other.colors;
        self.position = other.position;
        self.orient = other.orient;
    }
}

pub fn init_colors() {
    start_color();
    init_pair(Color::Red as i16, 0, 1);
    init_pair(Color::Green as i16, 0, 2);
    init_pair(Color::Yellow as i16, 0, 3);
    init_pair(Color::Blue as i16, 0, 4);

}

macro_rules! colored {
   ($c:expr => { $x:stmt }) => ( attron(COLOR_PAIR($c as i16)); $x; attroff(COLOR_PAIR($c as i16)););
}

#[derive(Debug,Clone,Copy)]
pub enum Color {
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
}

impl ::std::convert::From<u8> for Color {
    fn from(v:u8)->Self {
        match v {
            1 => Color::Red,
            2 => Color::Green,
            3 => Color::Yellow,
            4 => Color::Blue,
            _ => Color::Red
        }
    }
}

impl Color {
    pub fn rand() -> Self {
        let v:u8 = random();
        Color::from(v % 4 + 1)
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            &Color::Red => "*",
            &Color::Green => "@",
            &Color::Yellow => ".",
            &Color::Blue => "+",
        }
    }

}


impl Renderable for Chr {
    fn render(&self, view: &View) {
        let p = self.colors.0.symbol();
        let x = self.x() + view.x;
        let y = self.y() + view.y;
        colored!(self.colors.0 => {
            mvprintw(y, x, p)
        });
        let s = self.colors.1.symbol();
        colored!(self.colors.1 => {
            mvprintw(self.y2() + view.y, self.x2()+view.x, s)
        });
    }
}
