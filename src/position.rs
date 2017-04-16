use std::default::Default;
use std::fmt;

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn left(&self) -> Self {
        Position {
            x: self.x - 1,
            y: self.y,
        }
    }
    pub fn right(&self) -> Self {
        Position {
            x: self.x + 1,
            y: self.y,
        }
    }
    pub fn up(&self) -> Self {
        Position {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub fn down(&self) -> Self {
        Position {
            x: self.x,
            y: self.y + 1,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
