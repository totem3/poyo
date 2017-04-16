use direction::Direction;
use field::PoyoRows;

#[derive(Debug)]
pub enum Event<'a> {
    MovePoyo(&'a PoyoRows),
    Input(i32),
    Exit,
}
