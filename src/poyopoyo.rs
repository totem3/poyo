use std::cmp::{min, max};
use std::default::Default;

use poyo::Poyo;
use direction::Direction;
use direction::Direction::*;
use position::Position;
use color::Color;

#[derive(Debug,Clone,PartialEq)]
pub struct PoyoPoyo(pub Poyo, pub Poyo);

impl PoyoPoyo {
    pub fn new(p1: Poyo, p2: Poyo) -> Self {
        PoyoPoyo(p1, p2)
    }

    pub fn rand() -> Self {
        PoyoPoyo(Poyo::new(Position::new(1, 0), Color::rand()),
                 Poyo::new(Position::new(1, 1), Color::rand()))
    }

    pub fn x(&self) -> (i32, i32) {
        (self.0.x(), self.1.x())
    }

    pub fn y(&self) -> (i32, i32) {
        (self.0.y(), self.1.y())
    }

    pub fn left(&self) -> i32 {
        min(self.0.x(), self.1.x())
    }

    pub fn right(&self) -> i32 {
        max(self.0.x(), self.1.x())
    }

    pub fn top(&self) -> i32 {
        min(self.0.y(), self.1.y())
    }

    pub fn bottom(&self) -> i32 {
        max(self.0.y(), self.1.y())
    }

    pub fn moves(&mut self, d: Direction) {
        self.0.moves(d);
        self.1.moves(d);
    }

    pub fn rotate(&mut self) {
        if self.0.x() < self.1.x() {
            self.1.moves(Left);
            self.1.moves(Up);
        } else if self.0.x() > self.1.x() {
            self.1.moves(Right);
            self.1.moves(Down);
        } else if self.0.y() > self.1.y() {
            self.1.moves(Left);
            self.1.moves(Down);
        } else {
            self.1.moves(Right);
            self.1.moves(Up);
        }
    }

    pub fn rotated_position(&self) -> Position {
        if self.0.x() < self.1.x() {
            Position::new(self.1.x() - 1, self.1.y() - 1)
        } else if self.0.x() > self.1.x() {
            Position::new(self.1.x() + 1, self.1.y() + 1)
        } else if self.0.y() > self.1.y() {
            Position::new(self.1.x() - 1, self.1.y() + 1)
        } else {
            Position::new(self.1.x() + 1, self.1.y() - 1)
        }
    }
}

impl Default for PoyoPoyo {
    fn default() -> Self {
        PoyoPoyo(Poyo::new(Position::new(1, 0), Color::Red),
                 Poyo::new(Position::new(1, 1), Color::Red))
    }
}

#[cfg(test)]
mod test {
    use super::PoyoPoyo;
    use poyo::Poyo;
    use direction::Direction::*;
    use position::Position;
    use color::Color;

    #[test]
    fn test_poyopoyo_can_move() {
        let p1 = Poyo::default();
        let p2 = Poyo::new(Position::new(1, 0), Color::Red);
        let mut pp = PoyoPoyo::new(p1, p2);
        pp.moves(Left);
        assert_eq!(pp.x(), (-1, 0));
        pp.moves(Up);
        assert_eq!(pp.y(), (-1, -1));
        pp.moves(Down);
        assert_eq!(pp.y(), (0, 0));
        pp.moves(Right);
        assert_eq!(pp.x(), (0, 1));
    }

    #[test]
    fn test_poyopoyo_rotate() {
        let p1 = Poyo::default();
        let p2 = Poyo::new(Position::new(1, 0), Color::Red);
        let mut pp = PoyoPoyo::new(p1, p2);
        pp.rotate();
        let (x1, x2) = pp.x();
        let (y1, y2) = pp.y();
        assert_eq!(x1, 0);
        assert_eq!(y1, 0);
        assert_eq!(x2, 0);
        assert_eq!(y2, -1);

        pp.rotate();
        let (x1, x2) = pp.x();
        let (y1, y2) = pp.y();
        assert_eq!(x1, 0);
        assert_eq!(y1, 0);
        assert_eq!(x2, -1);
        assert_eq!(y2, 0);

        pp.rotate();
        let (x1, x2) = pp.x();
        let (y1, y2) = pp.y();
        assert_eq!(x1, 0);
        assert_eq!(y1, 0);
        assert_eq!(x2, 0);
        assert_eq!(y2, 1);

        pp.rotate();
        let (x1, x2) = pp.x();
        let (y1, y2) = pp.y();
        assert_eq!(x1, 0);
        assert_eq!(y1, 0);
        assert_eq!(x2, 1);
        assert_eq!(y2, 0);
    }
}
