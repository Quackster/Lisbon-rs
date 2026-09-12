//! Mirrors `net.h4bbo.lisbon.game.games.GameSpawn`.

use crate::game::games::enums::game_type::GameType;
use crate::game::pathfinder::position::Position;

#[derive(Clone, Debug)]
pub struct GameSpawn {
    position: Position,
    team_id: i32,
    map_id: i32,
    game_type: GameType,
}

impl GameSpawn {
    /// Mirrors the `GameSpawn(int, int, String, int, int, int)` constructor
    /// (Java's `super(x, y, 0, z, z)` is `Position::with_rotations`; the
    /// unknown-name `valueOf` throw is an `expect` here).
    pub fn new(team_id: i32, map_id: i32, game_type: &str, x: i32, y: i32, z: i32) -> Self {
        Self {
            position: Position::with_rotations(x, y, 0.0, z, z),
            team_id,
            map_id,
            game_type: GameType::from_str(&game_type.to_uppercase())
                .expect("unknown GameType"),
        }
    }

    /// Mirrors `Position.getPosition` (inherited).
    pub fn get_position(&self) -> &Position {
        &self.position
    }

    /// Mirrors `getTeamId()`.
    pub fn get_team_id(&self) -> i32 {
        self.team_id
    }

    /// Mirrors `getMapId()`.
    pub fn get_map_id(&self) -> i32 {
        self.map_id
    }

    /// Mirrors `getGameType()`.
    pub fn get_game_type(&self) -> GameType {
        self.game_type
    }
}
