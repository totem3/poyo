use ncurses::*;
use view::Renderable;
use board::Board;
use rand::random;

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
    colors: (Color, Color),
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
        let position = (2, 0);
        let orient = Orient::V;
        Chr::new(colors, position, orient)
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
        let after = match self.orient {
            Orient::RH => self.x() - 2,
            _ => self.x() - 1,
        };
        if after <= board.leftmost() {
            false
        } else {
            true
        }
    }
    pub fn can_move_right(&self, board: &Board) -> bool {
        let after = match self.orient {
            Orient::H => self.x() + 2,
            _ => self.x() + 1,
        };
        if after >= board.rightmost() {
            false
        } else {
            true
        }
    }
    pub fn can_move_up(&self, board: &Board) -> bool {
        let after = match self.orient {
            Orient::RV => self.y() - 2,
            _ => self.y() - 1,
        };
        if after <= board.topmost() {
            false
        } else {
            true
        }
    }
    pub fn can_move_down(&self, board: &Board) -> bool {
        let after = match self.orient {
            Orient::V => self.y() + 2,
            _ => self.y() + 1,
        };
        if after >= board.bottommost() {
            false
        } else {
            true
        }
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
}

fn color_view(c: Color) -> &'static str {
    match c {
        Color::Red => "*",
        Color::Green => "@",
        Color::Yellow => ".",
        Color::Blue => "+",
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
        let v:u8 = random();// % 4 + 1;
        Color::from(v % 4 + 1)
    }
}

impl Renderable for Chr {
    fn render(&self) {
        let p = color_view(self.colors.0);
        colored!(self.colors.0 => {
            mvprintw(self.y(), self.x(), p)
        });
        let s = color_view(self.colors.1);
        colored!(self.colors.1 => {
            match self.orient {
                Orient::V => mvprintw(self.y() + 1, self.x(), s),
                Orient::H => mvprintw(self.y(), self.x() + 1, s),
                Orient::RV => mvprintw(self.y() - 1, self.x(), s),
                Orient::RH => mvprintw(self.y(), self.x() - 1, s),
            }
        });
    }
}
