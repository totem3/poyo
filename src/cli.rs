use color::Color;
use game_state::GameState;
use ncurses::*;
use poyo::Poyo;
use size::Size;

// このCliViewはゲーム画面の描画用なんだよなぁ
// これを流用して他の画面作れるか？Elm的なアーキテクチャじゃないとだるいな
// これ自身がstateを持っているわけじゃないけど、Rowは持っている。
// 描画だけするものと状態を渡すものに分けたいな
// GameStateとstate内の状態を受け取って描画する方向に変更するのはどうか
pub struct CliView {
    max_size: Size,
    size: Size,
    win: WINDOW,
}

impl CliView {
    pub fn new(size: Size) -> Self {
        let mut max_width = 0;
        let mut max_height = 0;
        getmaxyx(stdscr(), &mut max_height, &mut max_width);
        let max_size = Size::new(max_width as usize, max_height as usize);
        let win: WINDOW = newwin(14, 8, 0, 0);

        CliView {
            max_size,
            size,
            win,
        }
    }
    pub fn init(&mut self) {
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
        self.win = newwin(14, 8, 0, 0);
    }

    pub fn draw(&self, state: &GameState) {
        wmove(self.win, 0, 0);
        wclear(self.win);
        match state {
            GameState::Start => {
                mvwprintw(self.win, 5, 1, "press");
                mvwprintw(self.win, 6, 1, "space");
                mvwprintw(self.win, 7, 1, "to");
                mvwprintw(self.win, 8, 1, "start");
                box_(self.win, '|' as u32, ' ' as u32);
            }
            GameState::Playing { ref poyos } => {
                for row in poyos.iter() {
                    for col in row.iter() {
                        if let &Some(p) = col {
                            self.print(&p);
                        }
                    }
                }
                box_(self.win, '|' as u32, ' ' as u32);
            }
            GameState::GameOver => {
                mvwprintw(self.win, 4, 2, "Game");
                mvwprintw(self.win, 5, 2, "Over");
                box_(self.win, '|' as u32, ' ' as u32);
            }
        }
        wrefresh(self.win);
    }

    fn print(&self, poyo: &Poyo) {
        let (x, y) = (poyo.x(), poyo.y());
        let (x, y) = CliView::translate(x, y);
        let color = poyo.color();
        let s = match color {
            Color::Red => "*",
            Color::Green => "+",
            Color::Yellow => "@",
            Color::Blue => "#",
        };
        wattron(self.win, COLOR_PAIR(color as i16));
        mvwprintw(self.win, y, x, s);
        wattroff(self.win, COLOR_PAIR(color as i16));
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
