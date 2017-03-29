extern crate ncurses;
extern crate rand;

mod board;
mod chr;
mod view;

use ncurses::*;
use std::sync::{Arc, Mutex};

use board::Board;
use chr::Chr;
use chr::Color;
use chr::Orient;
use view::View;
use view::Renderable;

static BOARD_HEIGHT: i32 = 12 + 2;
static BOARD_WIDTH: i32 = 6 + 2;

fn main() {
    View::init_view();
    chr::init_colors();

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    let start_y = (max_y - BOARD_HEIGHT) / 2;
    let start_x = (max_x - BOARD_WIDTH) / 2;
    let board = Arc::new(Mutex::new(Board::new (
        BOARD_WIDTH,
        BOARD_HEIGHT,
        Vec::new(),
    )));
    let x = start_x + 1;
    let y = start_y + 1;

    let s = Arc::new(Mutex::new(Chr::rand()));

    let _b = board.clone();
    let _s = s.clone();
    let view = View::new( start_x, start_y, vec![_b, _s] );
    view.render(&view);
    let mut ch = getch();
    while ch != 0x71 {
        // This block is necessary to avoid deadlock
        {
            let mut s = s.lock().unwrap();
            let b = board.lock().unwrap();
            s.moves(ch, &b);
        }
        view.render(&view);
        ch = getch();
    }

    endwin();
}
