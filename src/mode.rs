#[derive(Debug)]
pub enum GameMode {
    ChoosingMode,
    Playing,
    Learning,
}

impl GameMode {
    pub fn new() -> GameMode {
        Self::ChoosingMode
    }
}
