//! Mirrors `net.h4bbo.lisbon.game.games.gamehalls.utils.GameShipMoveResult`.

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GameShipMoveResult {
    Hit,
    Miss,
    Sink,
}

impl GameShipMoveResult {
    /// Mirrors `getSymbol()`.
    pub fn get_symbol(&self) -> &'static str {
        match self {
            GameShipMoveResult::Hit => "X",
            GameShipMoveResult::Miss => "O",
            GameShipMoveResult::Sink => "S",
        }
    }
}
