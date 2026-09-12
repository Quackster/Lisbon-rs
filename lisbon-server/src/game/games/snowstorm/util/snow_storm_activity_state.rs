//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.util.SnowStormActivityState`.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SnowStormActivityState {
    ActivityStateInvincibleAfterStun,
    ActivityStateStunned,
    ActivityStateCreating,
    ActivityStateNormal,
}

impl SnowStormActivityState {
    /// Mirrors `getStateId()`.
    pub fn get_state_id(&self) -> i32 {
        match self {
            Self::ActivityStateInvincibleAfterStun => 3,
            Self::ActivityStateStunned => 2,
            Self::ActivityStateCreating => 0,
            Self::ActivityStateNormal => 0,
        }
    }

    /// Mirrors `getTimer()`.
    pub fn get_timer(&self) -> i32 {
        match self {
            Self::ActivityStateInvincibleAfterStun => 60,
            Self::ActivityStateStunned => 125,
            Self::ActivityStateCreating => 20,
            Self::ActivityStateNormal => 0,
        }
    }

    /// Mirrors `getTimeInMS()`.
    pub fn get_time_in_ms(&self) -> i32 {
        let timer = self.get_timer();

        if timer > 0 {
            (timer / 5) * 300
        } else {
            0
        }
    }
}
