//! Mirrors `net.h4bbo.lisbon.game.games.wobblesquabble.WobbleSquabbleManager`.
use std::sync::{Arc, OnceLock};

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::games::wobblesquabble::wobble_squabble_game::WobbleSquabbleGame;
use crate::game::games::wobblesquabble::wobble_squabble_player::WobbleSquabblePlayer;
use crate::game::player::player::Player;

pub struct WobbleSquabbleManager;

impl WobbleSquabbleManager {
    /// Mirrors `WS_GAME_TICKET_COST`.
    pub const WS_GAME_TICKET_COST: i32 = 1;

    /// Mirrors `WS_BALANCE_POINTS`.
    pub const WS_BALANCE_POINTS: i32 = 35;

    /// Mirrors `WS_HIT_POINTS`.
    pub const WS_HIT_POINTS: i32 = 13;

    /// Mirrors `WS_HIT_BALANCE_POINTS`.
    pub const WS_HIT_BALANCE_POINTS: i32 = 10;

    /// Mirrors `WS_GAME_TIMEOUT_SECS`.
    pub const WS_GAME_TIMEOUT_SECS: i32 = 60;

    /// Returns true or false if the user is in a game of wobble squabble.
    pub fn is_playing(&self, player: &Player) -> bool {
        let Some(room_user) = player.get_room_user() else {
            return false;
        };

        let Some(room) = room_user.get_room() else {
            return false;
        };

        if !room.get_task_manager().has_task(self.get_name()) {
            return false;
        }

        let Some(ws_game) = room.get_task_manager().get_task(self.get_name()) else {
            return false;
        };

        ws_game
            .downcast_ref::<WobbleSquabbleGame>()
            .map_or(false, |ws_game| {
                ws_game
                    .get_player_by_id(player.get_details().get_id())
                    .is_some()
            })
    }

    /// Gets the wobble squabble player instance.
    pub fn get_player(
        &self,
        player: &Player,
    ) -> Option<Arc<Mutex<WobbleSquabblePlayer>>> {
        let Some(room_user) = player.get_room_user() else {
            return None;
        };

        let Some(room) = room_user.get_room() else {
            return None;
        };

        if !room.get_task_manager().has_task(self.get_name()) {
            return None;
        }

        let Some(ws_game) = room.get_task_manager().get_task(self.get_name()) else {
            return None;
        };

        ws_game
            .downcast_ref::<WobbleSquabbleGame>()?
            .get_player_by_id(player.get_details().get_id())
    }

    /// Get the static instance of the wobble squabble manager.
    pub fn get_instance() -> &'static Self {
        static INSTANCE: OnceLock<WobbleSquabbleManager> = OnceLock::new();
        INSTANCE.get_or_init(|| WobbleSquabbleManager)
    }

    /// Gets the name of the wobble squabble game task.
    pub fn get_name(&self) -> &'static str {
        "WobbleGameTask"
    }
}
