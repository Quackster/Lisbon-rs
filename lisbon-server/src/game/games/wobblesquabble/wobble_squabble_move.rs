//! Mirrors `net.h4bbo.lisbon.game.games.wobblesquabble.WobbleSquabbleMove`.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WobbleSquabbleMove {
    BalanceLeft,
    BalanceRight,
    HitLeft,
    HitRight,
    WalkForward,
    WalkBackward,
    Rebalance,
    None,
}

impl WobbleSquabbleMove {
    /// Mirrors `getMove(String)` (matches on the letter; returns `None` on
    /// an unknown letter, Java returns `null`).
    pub fn get_move(letter: &str) -> Option<Self> {
        for move_ in Self::values() {
            if move_.get_letter() == letter {
                return Some(*move_);
            }
        }

        None
    }

    /// Mirrors `values()`.
    pub fn values() -> &'static [WobbleSquabbleMove] {
        &[
            Self::BalanceLeft,
            Self::BalanceRight,
            Self::HitLeft,
            Self::HitRight,
            Self::WalkForward,
            Self::WalkBackward,
            Self::Rebalance,
            Self::None,
        ]
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        match self {
            Self::BalanceLeft => 1,
            Self::BalanceRight => 2,
            Self::HitLeft => 3,
            Self::HitRight => 4,
            Self::WalkForward => 5,
            Self::WalkBackward => 6,
            Self::Rebalance => 7,
            Self::None => 0,
        }
    }

    /// Mirrors `getLetter()`.
    pub fn get_letter(&self) -> &'static str {
        match self {
            Self::BalanceLeft => "A",
            Self::BalanceRight => "D",
            Self::HitLeft => "W",
            Self::HitRight => "E",
            Self::WalkForward => "X",
            Self::WalkBackward => "S",
            Self::Rebalance => "0",
            Self::None => "-",
        }
    }
}
