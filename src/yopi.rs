use ncurses::*;
use view::{View, Renderable};
use board::Board;
use rand::random;
use std::cmp::{min, max};
use std::default::Default;

const KEY_SPACE: i32 = 0x20;

macro_rules! colored {
   ($c:expr => { $x:stmt }) => ( attron(COLOR_PAIR($c as i16)); $x; attroff(COLOR_PAIR($c as i16)););
}

#[derive(Debug,Clone)]
pub enum Orient {
    V,
    H,
    RV,
    RH,
}
#[derive(Clone)]
pub struct YopiYopi{
    yopis: (Yopi, Yopi),
    orient: Orient,
}

#[derive(Clone)]
pub struct Yopi {
    color: Color,
    position: (i32, i32),
}

impl Yopi {
    pub fn new(color: Color, position: (i32, i32)) -> Self {
        Yopi {
            color,
            position,
        }
    }

    pub fn x(&self) -> i32 {
        self.position.0
    }

    pub fn y(&self) -> i32 {
        self.position.1
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

    pub fn down_by(&mut self, n: i32) {
        self.position.1 += n;
    }

    pub fn is_bottom(&self, board: &Board) -> bool {
        let x = self.x();
        let y = self.y() + 1;
        if y >= board.bottommost() || board.is_filled(x, y) {
            true
        } else {
            false
        }
    }

    pub fn to_bottom(&mut self, board: &Board) -> i32 {
        let mut diff = 0;
        while !self.is_bottom(board) {
            diff += 1;
            self.down();
        }
        diff
    }

}

impl Renderable for Yopi {
    fn render(&self, view: &View) {
        let p = self.color.symbol();
        let x = self.x() + view.x;
        let y = self.y() + view.y;
        colored!(self.color => {
            mvprintw(y, x, p)
        });
    }
}

impl YopiYopi {
    pub fn new(y1: Yopi, y2: Yopi) -> Self {
        let orient = if y1.x() == y2.x() {
            if y1.y() < y2.y() {
                Orient::V
            } else {
                Orient::RV
            }
        } else {
            if y1.x() < y2.x() {
                Orient::H
            } else {
                Orient::RH
            }
        };
        YopiYopi{yopis: (y1, y2), orient}
    }

    pub fn rand() -> Self {
        let colors = (Color::rand(), Color::rand());
        let p1 = (2, 1);
        let p2 = (2, 2);
        let y1 = Yopi::new(colors.0, p1);
        let y2 = Yopi::new(colors.1, p2);
        YopiYopi::new(y1, y2)
    }
    pub fn x(&self) -> i32 {
        self.yopis.0.x()
    }
    pub fn y(&self) -> i32 {
        self.yopis.0.y()
    }
    pub fn x2(&self) -> i32 {
        self.yopis.1.x()
    }
    pub fn y2(&self) -> i32 {
        self.yopis.1.y()
    }
    pub fn left(&mut self) {
        self.yopis.0.left();
        self.yopis.1.left();
    }
    pub fn right(&mut self) {
        self.yopis.0.right();
        self.yopis.1.right();
    }
    pub fn up(&mut self) {
        self.yopis.0.up();
        self.yopis.1.up();
    }
    pub fn down(&mut self) {
        self.yopis.0.down();
        self.yopis.1.down();
    }
    pub fn leftmost(&self) -> i32 {
        min(self.x(), self.x2())
    }
    pub fn rightmost(&self) -> i32 {
        max(self.x(), self.x2())
    }
    pub fn topmost(&self) -> i32 {
        min(self.y(), self.y2())
    }
    pub fn bottommost(&self) -> i32 {
        max(self.y(), self.y2())
    }
    pub fn rotate(&mut self, board: &Board) {
        let (top, right, bottom, left) = board.rectangle();
        match self.orient {
            Orient::V => {
                self.orient = Orient::H;
                self.yopis.1.right();
                self.yopis.1.up();
            },
            Orient::H => {
                self.orient = Orient::RV;
                self.yopis.1.left();
                self.yopis.1.up();
            },
            Orient::RV => {
                self.orient = Orient::RH;
                self.yopis.1.left();
                self.yopis.1.down();
            },
            Orient::RH => {
                self.orient = Orient::V;
                self.yopis.1.right();
                self.yopis.1.down();
            },
        }
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
        if x >= board.rightmost() || board.is_filled(x, y) {
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
            _ => {}
        }
    }

    pub fn colors(&self) -> (Color, Color) {
        (self.yopis.0.color, self.yopis.1.color)
    }

    pub fn replace(&mut self, other: Self) {
        self.yopis = other.yopis;
        self.orient = other.orient;
    }

    pub fn to_bottom(&mut self, board: &Board) {
        match self.orient {
            Orient::V => {
                let diff = self.yopis.1.to_bottom(board);
                self.yopis.0.down_by(diff);
            },
            Orient::RV => {
                let diff = self.yopis.0.to_bottom(board);
                self.yopis.1.down_by(diff);
            },
            _ => {
                self.yopis.0.to_bottom(board);
                self.yopis.1.to_bottom(board);
            }
        }
    }

}

pub fn init_colors() {
    start_color();
    init_pair(Color::Red as i16, 0, 1);
    init_pair(Color::Green as i16, 0, 2);
    init_pair(Color::Yellow as i16, 0, 3);
    init_pair(Color::Blue as i16, 0, 4);

}

#[derive(Debug,Clone,Copy)]
pub enum Color {
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
}

impl ::std::convert::From<u8> for Color {
    fn from(v: u8) -> Self {
        match v {
            1 => Color::Red,
            2 => Color::Green,
            3 => Color::Yellow,
            4 => Color::Blue,
            _ => Color::Red,
        }
    }
}

impl Color {
    pub fn rand() -> Self {
        let v: u8 = random();
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


impl Renderable for YopiYopi {
    fn render(&self, view: &View) {
        self.yopis.0.render(view);
        self.yopis.1.render(view);
    }
}
