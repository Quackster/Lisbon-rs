//! Mirrors `net.h4bbo.lisbon.game.games.battleball.enums.BattleBallTileState`.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BattleBallTileState {
    Default,
    Touched,
    Clicked,
    Pressed,
    Sealed,
}

impl BattleBallTileState {
    /// Mirrors `getTileStateId()`.
    pub fn get_tile_state_id(&self) -> i32 {
        match self {
            Self::Default => 0,
            Self::Touched => 1,
            Self::Clicked => 2,
            Self::Pressed => 3,
            Self::Sealed => 4,
        }
    }

    /// Mirrors `getStateById(int)` (returns `None` on an unknown id; Java
    /// returns `null`).
    pub fn get_state_by_id(id: i32) -> Option<Self> {
        for state in Self::values() {
            if state.get_tile_state_id() == id {
                return Some(*state);
            }
        }

        None
    }

    /// Mirrors `values()`.
    pub fn values() -> &'static [BattleBallTileState] {
        &[
            Self::Default,
            Self::Touched,
            Self::Clicked,
            Self::Pressed,
            Self::Sealed,
        ]
    }
}
