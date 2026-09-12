//! Mirrors `net.h4bbo.lisbon.game.games.gamehalls.utils.GameShip`.

use std::hash::{Hash, Hasher};

use crate::game::games::gamehalls::game_battle_ship::GameBattleShip;
use crate::game::games::gamehalls::utils::game_ship_move_result::GameShipMoveResult;
use crate::game::games::gamehalls::utils::game_ship_type::GameShipType;
use crate::game::pathfinder::position::Position;

fn positions_equal(a: &Position, b: &Position) -> bool {
    a.get_x() == b.get_x()
        && a.get_y() == b.get_y()
        && a.get_z() == b.get_z()
        && a.get_head_rotation() == b.get_head_rotation()
        && a.get_body_rotation() == b.get_body_rotation()
}

/// Mirrors `GameShip`.
// Port note: the Java `game` back-reference is omitted; `get_hits` /
// `is_hit_twice` / `is_destroyed` take the game as a parameter instead.
#[derive(Clone, Debug)]
pub struct GameShip {
    ship_type: GameShipType,
    position: Position,
    player: i32,
    is_horizontal: bool,
}

impl PartialEq for GameShip {
    fn eq(&self, other: &Self) -> bool {
        self.ship_type == other.ship_type
            && self.player == other.player
            && self.is_horizontal == other.is_horizontal
            && positions_equal(&self.position, &other.position)
    }
}

impl Eq for GameShip {}

impl Hash for GameShip {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // `Position` is not `Hash`; hash its fields instead.
        self.ship_type.hash(state);
        self.player.hash(state);
        self.is_horizontal.hash(state);
        self.position.get_x().hash(state);
        self.position.get_y().hash(state);
        (self.position.get_z() as i64).hash(state);
        self.position.get_head_rotation().hash(state);
        self.position.get_body_rotation().hash(state);
    }
}

impl GameShip {
    /// Mirrors the `GameShip(GameBattleShip, GameShipType, Position, int,
    /// boolean)` constructor (the `game` argument is dropped, see module
    /// note).
    pub fn new(
        ship_type: GameShipType,
        position: Position,
        player: i32,
        is_horizontal: bool,
    ) -> Self {
        Self {
            ship_type,
            position,
            player,
            is_horizontal,
        }
    }

    /// Mirrors `getShipType()`.
    pub fn get_ship_type(&self) -> GameShipType {
        self.ship_type
    }

    /// Mirrors `getPosition()`.
    pub fn get_position(&self) -> &Position {
        &self.position
    }

    /// Mirrors `getPlayer()`.
    pub fn get_player(&self) -> i32 {
        self.player
    }

    /// Mirrors `getHits()` (the `game` back-reference is a parameter).
    pub fn get_hits(&self, game: &GameBattleShip) -> i32 {
        let mut hits = 0;

        for i in 0..self.ship_type.get_length() {
            let ship_x = self.position.get_x() + if self.is_horizontal { i } else { 0 };
            let ship_y = self.position.get_y() + if self.is_horizontal { 0 } else { i };

            let ship_move = game
                .get_player_list_map()
                .get(&game.get_opposite_player_num(self.player))
                .and_then(|moves| {
                    moves
                        .iter()
                        .find(|move_| move_.get_x() == ship_x && move_.get_y() == ship_y)
                });

            if ship_move.is_none() {
                continue;
            }

            if ship_move.unwrap().get_move_result() == GameShipMoveResult::Hit {
                hits += 1;
            }
        }

        hits
    }

    /// Mirrors `isHitTwice()`.
    pub fn is_hit_twice(&self, game: &GameBattleShip) -> bool {
        let hits = self.get_hits(game);
        hits >= 2 && hits != self.ship_type.get_length()
    }

    /// Mirrors `isDestroyed()`.
    pub fn is_destroyed(&self, game: &GameBattleShip) -> bool {
        self.get_hits(game) == self.ship_type.get_length()
    }

    /// Mirrors `isHorizontal()`.
    pub fn is_horizontal(&self) -> bool {
        self.is_horizontal
    }
}
