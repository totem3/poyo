use ncurses::*;
use view::Renderable;
use board::Board;

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
}

fn color_view(c: Color) -> &'static str {
    match c {
        Color::Red => "*",
        Color::Green => "@",
        Color::Yellow => "%",
        Color::Blue => "+",
    }
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
