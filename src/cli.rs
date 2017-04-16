use ncurses::*;
use event::Event;
use observer::MutObserver;
use poyo::Poyo;
use size::Size;
use field::PoyoRows;
use color::Color;

use std::io::{self, Write};

pub struct CliView {
    max_size: Size,
    size: Size,
    poyos: PoyoRows,
    // win: WINDOW,
}

impl CliView {
    pub fn new(size: Size) -> Self {
        let mut max_width = 0;
        let mut max_height = 0;
        getmaxyx(stdscr(), &mut max_height, &mut max_width);
        let max_size = Size::new(max_width as usize, max_height as usize);
        let poyos = vec![];

        CliView {
            max_size,
            size,
            poyos,
            // win: win,
        }
    }
    pub fn init(&self) {
        initscr();
        if !has_colors() {
            endwin();
            panic!("terminal does not support colors");
        }
        start_color();
        raw();
        noecho();
        keypad(stdscr(), true);
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        init_pair(Color::Red as i16, 0, 1);
        init_pair(Color::Green as i16, 0, 2);
        init_pair(Color::Yellow as i16, 0, 3);
        init_pair(Color::Blue as i16, 0, 4);
        refresh();
    }

    pub fn draw(&self) {
        let win: WINDOW = newwin(14, 8, 0, 0);
        box_(win, 0, 0);
        for (y, row) in self.poyos.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if let &Some(p) = col {
                    CliView::print(win, &p);
                }
            }
        }
        wrefresh(win);
    }

    fn print(win: WINDOW, poyo: &Poyo) {
        let (x, y) = (poyo.x(), poyo.y());
        let (x, y) = CliView::translate(x, y);
        let color = poyo.color();
        let s = match color {
            Color::Red => "*",
            Color::Green => "+",
            Color::Yellow => "@",
            Color::Blue => "#",
        };
        wattron(win, COLOR_PAIR(color as i16));
        mvwprintw(win, y, x, s);
        wattroff(win, COLOR_PAIR(color as i16));
    }

    fn translate(x: i32, y: i32) -> (i32, i32) {
        (x + 1, y + 1)
    }

    pub fn exit(&self) {
        endwin();
    }

    pub fn width(&self) -> usize {
        self.size.width
    }

    pub fn height(&self) -> usize {
        self.size.height
    }
}

impl MutObserver for CliView {
    fn notify_mut(&mut self, event: &Event) {
        match event {
            &Event::MovePoyo(f) => self.poyos = f.clone(),
            _ => {}
        }
    }
}
