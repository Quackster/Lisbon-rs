//! Mirrors `net.h4bbo.lisbon.game.games.enums.GameType`.

use crate::util::config::game_configuration::GameConfiguration;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GameType {
    Battleball,
    Snowstorm,
    WobbleSquabble,
}

impl GameType {
    /// Mirrors `GameType.valueOf` (returns `None` on an unknown name; Java
    /// throws `IllegalArgumentException`).
    pub fn from_str(name: &str) -> Option<Self> {
        match name.to_uppercase().as_str() {
            "BATTLEBALL" => Some(Self::Battleball),
            "SNOWSTORM" => Some(Self::Snowstorm),
            "WOBBLE_SQUABBLE" => Some(Self::WobbleSquabble),
            _ => None,
        }
    }

    /// Mirrors `values()`.
    pub fn values() -> &'static [GameType] {
        &[Self::Battleball, Self::Snowstorm, Self::WobbleSquabble]
    }

    /// Mirrors `name()` (lower-case, as used by the configuration keys).
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::Battleball => "battleball",
            Self::Snowstorm => "snowstorm",
            Self::WobbleSquabble => "wobble_squabble",
        }
    }

    /// Get the cost of tickets required to play each game.
    pub fn get_ticket_cost(&self) -> i32 {
        GameConfiguration::get_instance()
            .get_integer(&format!("{}.ticket.charge", self.get_name()))
    }

    /// Mirrors `getLobbyModel()`.
    pub fn get_lobby_model(&self) -> &'static str {
        match self {
            Self::Battleball => "bb_lobby_1",
            Self::Snowstorm => "snowwar_lobby_1",
            Self::WobbleSquabble => "md_a",
        }
    }

    /// Mirrors `getTypeId()`.
    pub fn get_type_id(&self) -> i32 {
        match self {
            Self::Battleball => 1,
            Self::Snowstorm => 0,
            Self::WobbleSquabble => 1,
        }
    }
}
