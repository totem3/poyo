extern crate ncurses;

use ncurses::*;

static BOARD_HEIGHT: i32 = 12 + 2;
static BOARD_WIDTH: i32 = 6 + 2;

#[derive(Debug)]
struct Board {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    rows: Vec<Vec<Color>>,
    win: Option<WINDOW>,
}

impl Board {
    fn rectangle(&self) -> (i32, i32, i32, i32) {
        (self.topmost(), self.rightmost(), self.bottommost(), self.leftmost())
    }
    fn leftmost(&self) -> i32 {
        self.x
    }

    fn rightmost(&self) -> i32 {
        self.x + self.width - 1
    }

    fn topmost(&self) -> i32 {
        self.y
    }

    fn bottommost(&self) -> i32 {
        self.y + self.height - 1
    }

    fn render(&mut self) {
        self.destroy();
        self.create();
    }

    fn create(&mut self) {
        let win = newwin(self.height, self.width, self.y, self.x);
        box_(win, 0, 0);
        wrefresh(win);
        self.win = Some(win);
    }

    fn destroy(&mut self) {
        let ch = ' ' as chtype;
        if let Some(win) = self.win {
            wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
            wrefresh(win);
            delwin(win);
        }
    }
}

macro_rules! colored {
   ($c:expr => { $x:stmt }) => ( attron(COLOR_PAIR($c as i16)); $x; attroff(COLOR_PAIR($c as i16)););
}

#[derive(Debug,Clone,Copy)]
enum Color {
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
}

fn color_view(c: Color) -> &'static str {
    match c {
        Color::Red => "*",
        Color::Green => "@",
        Color::Yellow => "%",
        Color::Blue => "+",
    }
}

enum Orient {
    V,
    H,
    RV,
    RH,
}
struct Chr {
    colors: (Color, Color),
    position: (i32, i32),
    orient: Orient,
}

impl Chr {
    fn x(&self) -> i32 {
        self.position.0
    }
    fn y(&self) -> i32 {
        self.position.1
    }
    fn left(&mut self) {
        self.position.0 -= 1;
    }
    fn right(&mut self) {
        self.position.0 += 1;
    }
    fn up(&mut self) {
        self.position.1 -= 1;
    }
    fn down(&mut self) {
        self.position.1 += 1;
    }
    fn rotate(&mut self, board: &Board) {
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

    fn can_move_left(&self, board: &Board) -> bool {
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
    fn can_move_right(&self, board: &Board) -> bool {
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
    fn can_move_up(&self, board: &Board) -> bool {
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
    fn can_move_down(&self, board: &Board) -> bool {
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


fn main() {
    initscr();
    raw();

    keypad(stdscr(), true);
    noecho();

    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    refresh();

    start_color();
    init_pair(Color::Red as i16, 0, 1);
    init_pair(Color::Blue as i16, 0, 2);
    init_pair(Color::Yellow as i16, 0, 3);
    init_pair(Color::Blue as i16, 0, 4);

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    let start_y = (max_y - BOARD_HEIGHT) / 2;
    let start_x = (max_x - BOARD_WIDTH) / 2;
    let mut board = Board {
        width: BOARD_WIDTH,
        height: BOARD_HEIGHT,
        x: start_x,
        y: start_y,
        rows: Vec::new(),
        win: None,
    };
    board.render();
    let x = start_x + 1;
    let y = start_y + 1;

    let mut s = Chr {
        colors: (Color::Red, Color::Blue),
        position: (x, y),
        orient: Orient::V,
    };
    s.render();
    let mut ch = getch();
    while ch != KEY_F(1) {
        match ch {
            KEY_LEFT => {
                if s.can_move_left(&board) {
                    s.left();
                }
            }
            KEY_RIGHT => {
                if s.can_move_right(&board) {
                    s.right();
                }
            }
            KEY_UP => {
                if s.can_move_up(&board) {
                    s.up();
                }
            }
            KEY_DOWN => {
                if s.can_move_down(&board) {
                    s.down();
                }
            }
            0x20 => {
                s.rotate(&board);
            }
            _ => break,
        }
        board.render();
        s.render();
        ch = getch();
    }

    endwin();
}
