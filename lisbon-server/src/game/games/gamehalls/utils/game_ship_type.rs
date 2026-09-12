//! Mirrors `net.h4bbo.lisbon.game.games.gamehalls.utils.GameShipType`.

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GameShipType {
    AircraftCarrier,
    Battleship,
    Cruiser,
    Destroyer,
}

impl GameShipType {
    /// Mirrors `getById(int)` (returns `None` for an unknown id).
    pub fn get_by_id(id: i32) -> Option<Self> {
        for ship_type in [
            Self::AircraftCarrier,
            Self::Battleship,
            Self::Cruiser,
            Self::Destroyer,
        ] {
            if ship_type.get_id() == id {
                return Some(ship_type);
            }
        }

        None
    }

    /// Mirrors `getMaxAllowed()`.
    pub fn get_max_allowed(&self) -> i32 {
        match self {
            Self::AircraftCarrier => 1,
            Self::Battleship => 2,
            Self::Cruiser => 3,
            Self::Destroyer => 4,
        }
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        match self {
            Self::AircraftCarrier => 5,
            Self::Battleship => 4,
            Self::Cruiser => 3,
            Self::Destroyer => 2,
        }
    }

    /// Mirrors `getLength()`.
    pub fn get_length(&self) -> i32 {
        match self {
            Self::AircraftCarrier => 5,
            Self::Battleship => 4,
            Self::Cruiser => 3,
            Self::Destroyer => 2,
        }
    }
}
