//! Mirrors `net.h4bbo.lisbon.game.games.enums.GameState`.

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GameState {
    Waiting,
    Started,
    Ended,
}

impl GameState {
    /// Mirrors `getStateId()`.
    pub fn get_state_id(&self) -> i32 {
        match self {
            GameState::Waiting => 0,
            GameState::Started => 1,
            GameState::Ended => 2,
        }
    }
}
