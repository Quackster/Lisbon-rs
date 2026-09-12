//! Mirrors `net.h4bbo.lisbon.game.games.gamehalls.utils.GameShipMove`.

use parking_lot::Mutex;
use std::sync::Arc;

use crate::game::games::gamehalls::utils::game_ship::GameShip;
use crate::game::games::gamehalls::utils::game_ship_move_result::GameShipMoveResult;
use crate::game::player::player::Player;

#[derive(Clone)]
pub struct GameShipMove {
    player: Arc<Mutex<Player>>,
    x: i32,
    y: i32,
    move_result: GameShipMoveResult,
    ship: Option<GameShip>,
}

impl GameShipMove {
    /// Mirrors the `GameShipMove(Player, int, int, GameShipMoveResult,
    /// GameShip)` constructor.
    pub fn new(
        player: Arc<Mutex<Player>>,
        x: i32,
        y: i32,
        move_result: GameShipMoveResult,
        ship: Option<GameShip>,
    ) -> Self {
        Self {
            player,
            x,
            y,
            move_result,
            ship,
        }
    }

    /// Mirrors `getPlayer()`.
    pub fn get_player(&self) -> &Arc<Mutex<Player>> {
        &self.player
    }

    /// Mirrors `getX()`.
    pub fn get_x(&self) -> i32 {
        self.x
    }

    /// Mirrors `getY()`.
    pub fn get_y(&self) -> i32 {
        self.y
    }

    /// Mirrors `getMoveResult()`.
    pub fn get_move_result(&self) -> GameShipMoveResult {
        self.move_result
    }

    /// Mirrors `getShip()`.
    pub fn get_ship(&self) -> Option<&GameShip> {
        self.ship.as_ref()
    }
}
