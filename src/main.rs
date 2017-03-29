extern crate ncurses;

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
const KEY_SPACE: i32 = 0x20;

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
        start_x,
        start_y,
        Vec::new(),
    )));
    let x = start_x + 1;
    let y = start_y + 1;

    let s = Arc::new(Mutex::new(Chr::new(
        (Color::Red, Color::Blue),
        (x, y),
        Orient::V
    )));

    let _b = board.clone();
    let _s = s.clone();
    let view = View::new( vec![_b, _s] );
    view.render();
    let mut ch = getch();
    while ch != KEY_F(1) {
        match ch {
            KEY_LEFT => {
                let mut s = s.lock().unwrap();
                let b = board.lock().unwrap();
                if s.can_move_left(&b) {
                    s.left();
                }
            }
            KEY_RIGHT => {
                let mut s = s.lock().unwrap();
                let b = board.lock().unwrap();
                if s.can_move_right(&b) {
                    s.right();
                }
            }
            KEY_UP => {}
            KEY_DOWN => {
                let mut s = s.lock().unwrap();
                let b = board.lock().unwrap();
                if s.can_move_down(&b) {
                    s.down();
                }
            }
            KEY_SPACE => {
                let mut s = s.lock().unwrap();
                let b = board.lock().unwrap();
                s.rotate(&b);
            }
            _ => break,
        }
        view.render();
        ch = getch();
    }

    endwin();
}
