//! Mirrors `net.h4bbo.lisbon.game.games.battleball.enums.BattleBallColourState`.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BattleBallColourState {
    Disabled,
    Default,
    Red,
    Blue,
    Yellow,
    Green,
}

impl BattleBallColourState {
    /// Mirrors `getColourId()`.
    pub fn get_colour_id(&self) -> i32 {
        match self {
            Self::Disabled => -2,
            Self::Default => -1,
            Self::Red => 0,
            Self::Blue => 1,
            Self::Yellow => 2,
            Self::Green => 3,
        }
    }

    /// Mirrors `getColourById(int)` (returns `None` on an unknown id; Java
    /// returns `null`).
    pub fn get_colour_by_id(id: i32) -> Option<Self> {
        for colour in Self::values() {
            if colour.get_colour_id() == id {
                return Some(*colour);
            }
        }

        None
    }

    /// Mirrors `values()`.
    pub fn values() -> &'static [BattleBallColourState] {
        &[
            Self::Disabled,
            Self::Default,
            Self::Red,
            Self::Blue,
            Self::Yellow,
            Self::Green,
        ]
    }
}
