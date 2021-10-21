use color::Color;
use direction::Direction;
use direction::Direction::*;
use field::Field;
use position::Position;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Poyo {
    position: Position,
    color: Color,
}

impl Default for Poyo {
    fn default() -> Self {
        Poyo {
            color: Color::Red,
            position: Position::default(),
        }
    }
}

impl Poyo {
    pub fn new(position: Position, color: Color) -> Self {
        Poyo { position, color }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn x(&self) -> i32 {
        self.position.x
    }

    pub fn y(&self) -> i32 {
        self.position.y
    }

    pub fn moves(&mut self, d: Direction) {
        match d {
            Left => self.position.x -= 1,
            Right => self.position.x += 1,
            Up => self.position.y -= 1,
            Down => self.position.y += 1,
        }
    }

    pub fn update_position(&mut self, pos: Position) {
        self.position = pos;
    }

    pub fn is_same_color(&self, other: &Poyo) -> bool {
        self.color() == other.color()
    }

    pub fn count_same_color(&self, field: &Field) -> (usize, HashSet<Position>) {
        self._count_same_color(field, HashSet::new())
    }

    fn _count_same_color(
        &self,
        field: &Field,
        counted: HashSet<Position>,
    ) -> (usize, HashSet<Position>) {
        let mut counted = counted.clone();
        counted.insert(self.position);
        let mut count = 1;
        if !self.is_leftend(field) {
            if let Some(p) = field[self.position.left()] {
                if !counted.contains(&self.position.left()) && self.is_same_color(&p) {
                    counted.insert(self.position.left());
                    let (c, _counted) = p._count_same_color(field, counted);
                    count += c;
                    counted = _counted;
                }
            }
        }
        if !self.is_rightend(field) {
            if let Some(p) = field[self.position.right()] {
                if !counted.contains(&self.position.right()) && self.is_same_color(&p) {
                    counted.insert(self.position.right());
                    let (c, _counted) = p._count_same_color(field, counted);
                    count += c;
                    counted = _counted;
                }
            }
        }
        if !self.is_top(field) {
            if let Some(p) = field[self.position.up()] {
                if !counted.contains(&self.position.up()) && self.is_same_color(&p) {
                    counted.insert(self.position.up());
                    let (c, _counted) = p._count_same_color(field, counted);
                    count += c;
                    counted = _counted;
                }
            }
        }
        if !self.is_bottom(field) {
            if let Some(p) = field[self.position.down()] {
                if !counted.contains(&self.position.down()) && self.is_same_color(&p) {
                    counted.insert(self.position.down());
                    let (c, _counted) = p._count_same_color(field, counted);
                    count += c;
                    counted = _counted;
                }
            }
        }
        (count, counted)
    }

    pub fn is_leftend(&self, field: &Field) -> bool {
        field.leftend() == self.x()
    }

    pub fn is_rightend(&self, field: &Field) -> bool {
        field.rightend() - 1 == self.x()
    }

    pub fn is_top(&self, field: &Field) -> bool {
        field.top() == self.y()
    }

    pub fn is_bottom(&self, field: &Field) -> bool {
        field.bottom() - 1 == self.y()
    }
}

#[test]
fn test_poyo_can_move() {
    let mut poyo = Poyo::default();
    poyo.moves(Left);
    assert_eq!(poyo.x(), -1);
    poyo.moves(Right);
    assert_eq!(poyo.x(), 0);
    poyo.moves(Up);
    assert_eq!(poyo.x(), 0);
    assert_eq!(poyo.y(), -1);
    poyo.moves(Down);
    poyo.moves(Down);
    assert_eq!(poyo.y(), 1);
}
