//! Mirrors `net.h4bbo.lisbon.game.games.battleball.enums.BattleBallPlayerState`.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BattleBallPlayerState {
    Normal,
    Stunned,
    TurboBoost,
    HighJumps,
    CleaningTiles,
    ColouringForOpponent,
    ClimbingIntoCannon,
    FlyingThroughAir,
    BallBroken,
}

impl BattleBallPlayerState {
    /// Mirrors `getStateId()`.
    pub fn get_state_id(&self) -> i32 {
        match self {
            Self::Normal => 0,
            Self::Stunned => 1,
            Self::TurboBoost => 2,
            Self::HighJumps => 3,
            Self::CleaningTiles => 4,
            Self::ColouringForOpponent => 5,
            Self::ClimbingIntoCannon => 6,
            Self::FlyingThroughAir => 7,
            Self::BallBroken => 8,
        }
    }
}
