//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.objects.SnowStormMachineObject`.
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

use crate::game::games::enums::game_object_type::GameObjectType;
use crate::game::games::game_object::GameObject;
use crate::game::games::snowstorm::snow_storm_game::SnowStormGame;
use crate::game::pathfinder::position::Position;
use crate::server::netty::streams::NettyResponse;

pub struct SnowStormMachineObject {
    object_id: i32,
    x: i32,
    y: i32,
    snowball_count: AtomicI32,
    last_refill_time: i64,
}

impl SnowStormMachineObject {
    /// Mirrors the `SnowStormMachineObject(int, int, int, int)` constructor.
    pub fn new(object_id: i32, x: i32, y: i32, snowball_count: i32) -> Self {
        Self {
            object_id,
            x,
            y,
            snowball_count: AtomicI32::new(snowball_count),
            last_refill_time: 0,
        }
    }

    /// Mirrors `getLastRefillTime()`.
    pub fn get_last_refill_time(&self) -> i64 {
        self.last_refill_time
    }

    /// Mirrors `setLastRefillTime(long)`.
    pub fn set_last_refill_time(&mut self, last_refill_time: i64) {
        self.last_refill_time = last_refill_time
    }

    /// Mirrors `getPosition()`.
    pub fn get_position(&self) -> Position {
        Position::new_xy(self.x, self.y)
    }

    /// Mirrors `getSnowballs().get()` (the Java `AtomicInteger` is an
    /// `i32` snapshot here).
    pub fn get_snowballs(&self) -> i32 {
        self.snowball_count.load(Ordering::SeqCst)
    }

    /// Mirrors `getSnowballs().incrementAndGet()`.
    pub fn increment_snowballs(&self) -> i32 {
        self.snowball_count.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// Mirrors `getSnowballs().decrementAndGet()`.
    pub fn decrement_snowballs(&self) -> i32 {
        self.snowball_count.fetch_sub(1, Ordering::SeqCst) - 1
    }

    /// Mirrors `isPlayerCollectingSnowballs(SnowStormGame)`.
    pub fn is_player_collecting_snowballs(
        &self,
        game: &Arc<SnowStormGame>,
    ) -> bool {
        let player_position = Position::new_xy(self.x, self.y + 1);
        let map = game.get_map();
        let Some(map) = map else {
            return false;
        };

        let Some(_tile) = map.get_tile(&player_position) else {
            return false;
        };

        let players = game.get_active_players();
        let player = players.iter().find(|p| {
            p.lock()
                .get_snow_storm_attributes()
                .get_current_position()
                .map(|c| c == &player_position)
                .unwrap_or(false)
        }).cloned();

        let Some(player) = player else {
            return false;
        };

        let walkable = player
            .lock()
            .get_snow_storm_attributes()
            .is_walkable();

        walkable
    }
}

impl GameObject for SnowStormMachineObject {
    /// Mirrors `serialiseObject(NettyResponse)`.
    fn serialise_object(&self, response: &mut NettyResponse) {
        response.write_int(GameObjectType::SnowwarSnowmachineObject.get_object_id());
        response.write_int(self.object_id);
        response.write_int(SnowStormGame::convert_to_world_coordinate(self.x));
        response.write_int(SnowStormGame::convert_to_world_coordinate(self.y));
        response.write_int(self.get_snowballs());
    }

    /// Mirrors `getGameObjectType()`.
    fn get_game_object_type(&self) -> GameObjectType {
        GameObjectType::SnowwarSnowmachineObject
    }

    /// Mirrors `getId()`.
    fn get_id(&self) -> i32 {
        self.object_id
    }
}
