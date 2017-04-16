use std::default::Default;
use std::ops::{Index, IndexMut};

use poyo::Poyo;
use poyopoyo::PoyoPoyo;
use size::Size;
use direction::Direction;
use direction::Direction::*;
use observable::{MutObserverRef, MutObservable};
use event::Event;
use position::Position;

pub type PoyoRows = Vec<Vec<Option<Poyo>>>;

pub struct Field {
    observers: Vec<MutObserverRef>,
    size: Size,
    current: Option<PoyoPoyo>,
    poyos: PoyoRows,
}

impl Index<Position> for Field {
    type Output = Option<Poyo>;
    fn index(&self, index: Position) -> &Self::Output {
        &self.poyos[index.y as usize][index.x as usize]
    }
}

impl IndexMut<Position> for Field {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.poyos[index.y as usize][index.x as usize]
    }
}

impl MutObservable for Field {
    fn register_mut(&mut self, observer: MutObserverRef) {
        self.observers.push(observer);
    }
}

impl Field {
    pub fn new(size: Size) -> Self {
        let poyos = vec![vec![None; size.width]; size.height];
        Field {
            size,
            current: None,
            poyos,
            observers: vec![],
        }
    }

    pub fn move_current(&mut self, d: Direction) {
        if self.current_can_move(&d) {
            if let Some(mut c) = self.current.take() {
                let (x1, x2) = c.x();
                let (y1, y2) = c.y();
                self.poyos[y1 as usize][x1 as usize] = None;
                self.poyos[y2 as usize][x2 as usize] = None;
                c.moves(d);
                self.current = Some(c);
                self.update_field();
                self.update();
            }
        }
    }

    pub fn rotate_current(&mut self) {
        if self.current_can_rotate() {
            if let Some(mut c) = self.current.take() {
                let (x1, x2) = c.x();
                let (y1, y2) = c.y();
                self.poyos[y1 as usize][x1 as usize] = None;
                self.poyos[y2 as usize][x2 as usize] = None;
                c.rotate();
                self.current = Some(c);
                self.update_field();
                self.update();
            }
        }
    }

    fn current_can_rotate(&self) -> bool {
        if let Some(ref current) = self.current {
            let pos = current.rotated_position();
            let res = !self.is_filled(pos.x, pos.y);
            res
        } else {
            false
        }
    }

    pub fn fix_current(&mut self) {
        if let Some(_) = self.current.take() {
            self.fall_poyos();
            while self.check() > 0 {
                self.fall_poyos();
            }
            self.current = Some(PoyoPoyo::rand());
            self.update_field();
            self.update();
        }
    }

    fn update(&self) {
        for o in &self.observers {
            if let Some(observer) = o.upgrade() {
                let mut observer = observer.lock().unwrap();
                observer.notify_mut(&Event::MovePoyo(&self.poyos))
            }
        }
    }

    pub fn on_init(&self) {
        for o in &self.observers {
            if let Some(observer) = o.upgrade() {
                let mut observer = observer.lock().unwrap();
                observer.notify_mut(&Event::MovePoyo(&self.poyos))
            }
        }
    }

    pub fn set_current(&mut self, v: PoyoPoyo) {
        self.current = Some(v.clone());
        let (x1, x2) = v.x();
        let (y1, y2) = v.y();
        self.poyos[y1 as usize][x1 as usize] = Some(v.0);
        self.poyos[y2 as usize][x2 as usize] = Some(v.1);
    }

    pub fn update_field(&mut self) {
        let mut new_field = vec![vec![None;self.size.width]; self.size.height];
        for ps in self.poyos.clone() {
            for p in ps {
                if let Some(p) = p {
                    new_field[p.y() as usize][p.x() as usize] = Some(p);
                }
            }
        }
        if let Some(p) = self.current.clone() {
            let (x1, x2) = p.x();
            let (y1, y2) = p.y();
            new_field[y1 as usize][x1 as usize] = Some(p.0.clone());
            new_field[y2 as usize][x2 as usize] = Some(p.1.clone());
        }
        self.poyos = new_field;
    }

    pub fn width(&self) -> usize {
        self.size.width
    }

    pub fn height(&self) -> usize {
        self.size.height
    }

    pub fn current_can_move(&self, d: &Direction) -> bool {
        if let Some(ref current) = self.current {
            match d {
                &Left => {
                    let (y1, y2) = current.y();
                    let x = current.left() - 1;
                    let is_leftend = x < self.leftend();
                    if is_leftend {
                        return false;
                    }
                    !(self.is_filled(x, y1) || self.is_filled(x, y2))
                }
                &Right => {
                    let (y1, y2) = current.y();
                    let x = current.right() + 1;
                    let is_rightend = self.rightend() <= x;
                    if is_rightend {
                        return false;
                    }
                    !(self.is_filled(x, y1) || self.is_filled(x, y2))
                }
                &Down => {
                    let (x1, x2) = current.x();
                    let y = current.bottom() + 1;
                    let is_bottom = self.bottom() <= y;
                    if is_bottom {
                        return false;
                    }
                    !(self.is_filled(x1, y) || self.is_filled(x2, y))
                }
                _ => false,
            }
        } else {
            false
        }
    }

    pub fn is_filled(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || self.rightend() <= x || self.bottom() <= y {
            return true;
        }
        let (x, y) = (x as usize, y as usize);
        self.poyos[y][x].is_some()
    }

    pub fn leftend(&self) -> i32 {
        0
    }

    pub fn top(&self) -> i32 {
        0
    }

    pub fn rightend(&self) -> i32 {
        self.width() as i32
    }

    pub fn bottom(&self) -> i32 {
        self.height() as i32
    }

    pub fn fall_poyos(&mut self) {
        self.transpose();
        self.right_align();
        self.transpose();
        self.reset_position();
    }

    fn transpose(&mut self) {
        let mut res = Vec::with_capacity(self.poyos[0].len());
        for i in 0..self.poyos[0].len() {
            let mut new_row = Vec::with_capacity(self.poyos.len());
            for row in &self.poyos {
                new_row.push(row[i]);
            }
            res.push(new_row);
        }
        self.poyos = res;
    }

    fn right_align(&mut self) {
        let mut res = Vec::with_capacity(self.poyos.len());
        for row in &self.poyos {
            let somes: Vec<&Option<Poyo>> = row.iter().filter(|c| c.is_some()).collect();
            let mut new_row = vec![None; row.len()-somes.len()];
            new_row.extend(somes);
            res.push(new_row);
        }
        self.poyos = res;
    }

    fn reset_position(&mut self) {
        let mut res = Vec::with_capacity(self.poyos.len());
        for (y, row) in self.poyos.iter().enumerate() {
            let mut new_row = vec![];
            for (x, col) in row.iter().enumerate() {
                if let &Some(mut p) = col {
                    let position = Position::new(x as i32, y as i32);
                    p.update_position(position);
                    new_row.push(Some(p));
                } else {
                    new_row.push(None);
                }
            }
            res.push(new_row);
        }
        self.poyos = res;
    }

    pub fn check(&mut self) -> usize {
        let poyos = self.poyos.clone();
        let mut removed = 0;
        for row in poyos.iter() {
            for col in row.iter() {
                if let &Some(v) = col {
                    let (count, counted) = v.count_same_color(&self);
                    if count >= 4 {
                        removed += count;
                        for pos in counted {
                            self[pos] = None;
                        }
                    }
                }
            }
        }
        removed
    }
}

impl Default for Field {
    fn default() -> Self {
        let size = Size::new(6, 12);
        Field::new(size)
    }
}

#[cfg(test)]
mod test {
    use std::io::stderr;
    use std::io::Write;
    use super::Field;
    use color::Color;
    use position::Position;
    use poyo::Poyo;
    use poyopoyo::PoyoPoyo;
    use direction::Direction::*;
    use size::Size;

    #[test]
    fn test_field_current_poyo_reflects_poyos() {
        let mut field = Field::default();
        let p1 = Poyo::default();
        let p2 = Poyo::new(Position::new(1, 0), Color::Red);
        let pp = PoyoPoyo::new(p1.clone(), p2.clone());
        field.set_current(pp.clone());
        assert_eq!(field.current, Some(pp.clone()));
        let (x1, x2) = pp.x();
        let (y1, y2) = pp.y();
        assert_eq!(field.poyos[y1 as usize][x1 as usize], Some(p1));
        assert_eq!(field.poyos[y2 as usize][x2 as usize], Some(p2));
    }

    #[test]
    fn test_field_can_move_current() {
        let mut field = Field::default();
        let mut p1 = Poyo::default();
        let mut p2 = Poyo::new(Position::new(1, 0), Color::Red);
        let pp = PoyoPoyo::new(p1.clone(), p2.clone());
        let (x1, x2) = pp.x();
        let (y1, y2) = pp.y();
        field.set_current(pp);
        field.move_current(Down);
        assert_eq!(Some((y1 + 1, y2 + 1)), field.current.map(|p| p.y()));
        p1.moves(Down);
        p2.moves(Down);
        assert_eq!(field.poyos[y1 as usize][x1 as usize], None);
        assert_eq!(field.poyos[(y1 + 1) as usize][x1 as usize], Some(p1));
        assert_eq!(field.poyos[(y2 + 1) as usize][x2 as usize], Some(p2));
    }

    #[test]
    fn test_current_cannot_move_left_when_leftend() {
        let mut field = Field::default();
        let p1 = Poyo::default();
        let p2 = Poyo::new(Position::new(1, 0), Color::Red);
        let pp = PoyoPoyo::new(p1.clone(), p2.clone());
        field.set_current(pp);
        assert_eq!(field.current_can_move(&Left), false);
        assert_eq!(field.current_can_move(&Right), true);
    }

    #[test]
    fn test_current_cannot_move_right_when_rightend() {
        let mut field = Field::default();
        let p1 = Poyo::new(Position::new(field.rightend() - 2, 0), Color::Red);
        let p2 = Poyo::new(Position::new(field.rightend() - 1, 0), Color::Red);
        let pp = PoyoPoyo::new(p1.clone(), p2.clone());
        field.set_current(pp);
        assert_eq!(field.current_can_move(&Right), false);
        assert_eq!(field.current_can_move(&Left), true);
    }

    #[test]
    fn test_current_cannot_move_down_when_bottom() {
        let mut field = Field::default();
        let p1 = Poyo::new(Position::new(0, field.bottom() - 2), Color::Red);
        let p2 = Poyo::new(Position::new(0, field.bottom() - 1), Color::Red);
        let pp = PoyoPoyo::new(p1.clone(), p2.clone());
        field.set_current(pp.clone());
        assert_eq!(field.current_can_move(&Down), false);
    }

    #[test]
    fn test_current_cannot_move_left_when_already_filled() {
        let mut field = Field::default();
        let pp = PoyoPoyo::new(Poyo::new(Position::new(0, 0), Color::Red),
                               Poyo::new(Position::new(0, 1), Color::Red));
        field.set_current(pp);
        let pp = PoyoPoyo::new(Poyo::new(Position::new(1, 0), Color::Red),
                               Poyo::new(Position::new(2, 0), Color::Red));
        field.set_current(pp);
        assert_eq!(field.current_can_move(&Left), false);
    }

    #[test]
    fn test_transpose() {
        let temp: Vec<Vec<(i32, i32)>> = vec![vec![(0, 0), (1, 0), (2, 0)],
                                              vec![(0, 1), (1, 1), (2, 1)],
                                              vec![(0, 2), (1, 2), (2, 2)]];
        let poyos: Vec<Vec<Option<Poyo>>> = temp.iter()
            .map(|row| {
                     row.iter()
                         .map(|&(x, y)| Some(Poyo::new(Position::new(x, y), Color::Red)))
                         .collect()
                 })
            .collect();
        let size = Size::new(3, 3);
        let mut field = Field {
            size: size,
            poyos: poyos,
            ..Default::default()
        };
        field.transpose();
        let mut actual = vec![];
        for row in field.poyos {
            let mut nrow = vec![];
            for poyo in row {
                if let Some(poyo) = poyo {
                    nrow.push((poyo.x(), poyo.y()))
                }
            }
            actual.push(nrow);
        }
        let expected: Vec<Vec<(i32, i32)>> = vec![vec![(0, 0), (0, 1), (0, 2)],
                                                  vec![(1, 0), (1, 1), (1, 2)],
                                                  vec![(2, 0), (2, 1), (2, 2)]];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_rightalign() {
        let temp: Vec<Vec<Option<(i32, i32)>>> = vec![vec![Some((0, 0)), None, None],
                                                      vec![Some((0, 1)), None, Some((2, 1))],
                                                      vec![None, Some((1, 2)), None]];
        let poyos: Vec<Vec<Option<Poyo>>> = temp.iter()
            .map(|row| {
                     row.iter()
                         .map(|pos| {
                                  pos.map(|(x, y)| Poyo::new(Position::new(x, y), Color::Red))
                              })
                         .collect()
                 })
            .collect();
        let size = Size::new(3, 3);
        let mut field = Field {
            size: size,
            poyos: poyos,
            ..Default::default()
        };
        field.right_align();
        let expected: Vec<Vec<Option<(i32, i32)>>> = vec![vec![None, None, Some((0, 0))],
                                                          vec![None, Some((0, 1)), Some((2, 1))],
                                                          vec![None, None, Some((1, 2))]];
        let expected: Vec<Vec<Option<Poyo>>> = expected
            .iter()
            .map(|row| {
                     row.iter()
                         .map(|pos| {
                                  pos.map(|(x, y)| Poyo::new(Position::new(x, y), Color::Red))
                              })
                         .collect()
                 })
            .collect();
        assert_eq!(field.poyos, expected);
    }

    #[test]
    fn test_fall_poyos() {
        let temp: Vec<Vec<Option<(i32, i32)>>> = vec![vec![Some((0, 0)), None, None],
                                                      vec![Some((0, 1)), None, Some((2, 1))],
                                                      vec![None, Some((1, 2)), None]];
        let poyos: Vec<Vec<Option<Poyo>>> = temp.iter()
            .map(|row| {
                     row.iter()
                         .map(|pos| {
                                  pos.map(|(x, y)| Poyo::new(Position::new(x, y), Color::Red))
                              })
                         .collect()
                 })
            .collect();
        let size = Size::new(3, 3);
        let mut field = Field {
            size: size,
            poyos: poyos,
            ..Default::default()
        };
        field.fall_poyos();
        let expected: Vec<Vec<Option<(i32, i32)>>> =
            vec![vec![None, None, None],
                 vec![Some((0, 1)), None, None],
                 vec![Some((0, 2)), Some((1, 2)), Some((2, 2))]];
        let expected: Vec<Vec<Option<Poyo>>> = expected
            .iter()
            .map(|row| {
                     row.iter()
                         .map(|pos| {
                                  pos.map(|(x, y)| Poyo::new(Position::new(x, y), Color::Red))
                              })
                         .collect()
                 })
            .collect();
        assert_eq!(field.poyos, expected);
    }

    #[test]
    fn test_reset_position() {
        let temp = vec![vec![(8,8),(8,9),(9,9)];3];
        let poyos: Vec<Vec<Option<Poyo>>> = temp.iter()
            .map(|row| {
                     row.iter()
                         .map(|pos| Some(Poyo::new(Position::new(pos.0, pos.1), Color::Red)))
                         .collect()
                 })
            .collect();
        let size = Size::new(3, 3);
        let mut field = Field {
            size: size,
            poyos: poyos,
            ..Default::default()
        };
        field.fall_poyos();
        for (y, row) in field.poyos.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if let &Some(v) = col {
                    assert_eq!(v.x(), x as i32);
                    assert_eq!(v.y(), y as i32);
                }
            }
        }
    }

    #[test]
    fn test_fix_current() {
        let mut field = Field::default();
        let p1 = Poyo::new(Position::new(3, field.bottom() - 2), Color::Red);
        let p2 = Poyo::new(Position::new(3, field.bottom() - 1), Color::Red);
        let pp = PoyoPoyo::new(p1.clone(), p2.clone());
        field.set_current(pp);
        field.fix_current();
        let current = field.current.unwrap();
        assert_eq!(current.x(), (1, 1));
        assert_eq!(current.y(), (0, 1));
    }
}
