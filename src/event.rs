use field::PoyoRows;

#[derive(Debug)]
pub enum Event {
    MovePoyo(PoyoRows),
    Input(i32),
    Exit,
    FrameUpdate,
    Tick,
}
