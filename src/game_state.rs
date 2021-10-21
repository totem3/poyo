use field::PoyoRows;
pub enum GameState {
    Start,
    Playing { poyos: PoyoRows },
    GameOver,
}
