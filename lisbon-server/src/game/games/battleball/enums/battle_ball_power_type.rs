//! Mirrors `net.h4bbo.lisbon.game.games.battleball.enums.BattleBallPowerType`.

use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BattleBallPowerType {
    Lightblub,
    Spring,
    Flashlight,
    Cannon,
    BoxOfPins,
    Harlequin,
    Bomb,
    Drill,
    QuestionMark,
}

impl BattleBallPowerType {
    /// Mirrors `random()`.
    pub fn random() -> Self {
        let values = Self::values();
        values[rand::thread_rng().gen_range(0..values.len())]
    }

    /// Mirrors `getById(int)` (returns `None` on an unknown id; Java
    /// returns `null`).
    pub fn get_by_id(power_up_id: i32) -> Option<Self> {
        for power_up in Self::values() {
            if power_up.get_power_up_id() == power_up_id {
                return Some(*power_up);
            }
        }

        None
    }

    /// Mirrors `getPowerUpId()`.
    pub fn get_power_up_id(&self) -> i32 {
        match self {
            Self::Lightblub => 1,
            Self::Spring => 2,
            Self::Flashlight => 3,
            Self::Cannon => 4,
            Self::BoxOfPins => 5,
            Self::Harlequin => 6,
            Self::Bomb => 7,
            Self::Drill => 8,
            Self::QuestionMark => 9,
        }
    }

    /// Mirrors `values()`.
    pub fn values() -> &'static [BattleBallPowerType] {
        &[
            Self::Lightblub,
            Self::Spring,
            Self::Flashlight,
            Self::Cannon,
            Self::BoxOfPins,
            Self::Harlequin,
            Self::Bomb,
            Self::Drill,
            Self::QuestionMark,
        ]
    }
}
