use rand::random;
use std::convert::From;

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Color {
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
}

impl From<u8> for Color {
    fn from(v: u8) -> Self {
        match v % 4 {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Yellow,
            3 => Color::Blue,
            _ => Color::Red,
        }
    }
}

impl Color {
    pub fn rand() -> Self {
        Color::from(random::<u8>())
    }
}
