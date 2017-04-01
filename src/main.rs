extern crate ncurses;
extern crate rand;

mod board;
mod yopi;
mod view;

use ncurses::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use board::Board;
use yopi::YopiYopi;
use view::View;
use view::Renderable;

const BOARD_HEIGHT: i32 = 12 + 2;
const BOARD_WIDTH: i32 = 6 + 2;

fn main() {
    let (start_x, start_y) = View::init_view();
    yopi::init_colors();

    let board = Board::new (
        BOARD_WIDTH,
        BOARD_HEIGHT,
        Vec::new(),
    );

    let s = YopiYopi::rand();

    // Renderableのvecとして渡しつつ、他ではYopiYopiとかBoardそのものとして扱いたいんだけどどうすればいいのか
    // + Send が必要なだけだった
    let board = Arc::new(Mutex::new(board));
    let s = Arc::new(Mutex::new(s));
    let mut objects:Vec<Arc<Mutex<Renderable+Send>>> = Vec::new();
    objects.push(board.clone());
    objects.push(s.clone());
    let view = View::new( start_x, start_y, objects );
    let _ = thread::spawn(move || {
        loop {
            view.render(&view);
            thread::sleep(Duration::new(0,16666666));
        }
    });

    {
        let mut s = s.clone();
        let board = board.clone();
        let _ = thread::spawn(move || {
            loop {
                thread::sleep(Duration::new(1,0));
                {
                    let mut s = s.lock().unwrap();
                    let b = board.lock().unwrap();
                    if s.can_move_down(&b) {
                        s.down();
                    }
                }
                let mut s = s.lock().unwrap();
                let mut b = board.lock().unwrap();
                if s.is_bottom(&b) {
                    b.add(&s);
                    s.replace(YopiYopi::rand());
                }
            }
        });

    }

    let t = thread::spawn(move || {
        let mut ch = getch();
        while ch != 0x71 {
            // This block is necessary to avoid deadlock
            {
                let mut s = s.lock().unwrap();
                let b = board.lock().unwrap();
                s.moves(ch, &b);
            }
            ch = getch();
        }
    });

    match t.join() {
        Ok(_) => {},
        Err(_) => {
            println!("child process is dead");
        }
    };

    endwin();
}
