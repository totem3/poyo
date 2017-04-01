extern crate ncurses;
extern crate rand;

mod board;
mod chr;
mod view;

use ncurses::*;
use std::sync::{Arc, Mutex};

use board::Board;
use chr::Chr;
use view::View;
use view::Renderable;

static BOARD_HEIGHT: i32 = 12 + 2;
static BOARD_WIDTH: i32 = 6 + 2;

fn main() {
    let (start_x, start_y) = View::init_view();
    chr::init_colors();

    let board = Arc::new(Mutex::new(Board::new (
        BOARD_WIDTH,
        BOARD_HEIGHT,
        Vec::new(),
    )));

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
