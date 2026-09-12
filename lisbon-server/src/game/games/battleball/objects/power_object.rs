//! Mirrors `net.h4bbo.lisbon.game.games.battleball.objects.PowerObject`.
// Port note: the Java strong reference would form an `Arc` cycle with
// the `BattleBallPowerUp` (which holds this object); the back-reference
// is a `Weak`.
use std::sync::{Arc, Weak};

use parking_lot::Mutex;

use crate::game::games::battleball::battle_ball_power_up::BattleBallPowerUp;
use crate::game::games::enums::game_object_type::GameObjectType;
use crate::game::games::game_object::GameObject;
use crate::server::netty::streams::NettyResponse;

pub struct PowerObject {
    power_up: Weak<Mutex<BattleBallPowerUp>>,
}

impl PowerObject {
    /// Mirrors the `PowerObject(BattleBallPowerUp)` constructor.
    pub fn new(power_up: Weak<Mutex<BattleBallPowerUp>>) -> Self {
        Self { power_up }
    }

    /// Mirrors `getPowerUp()`.
    pub fn get_power_up(&self) -> Option<Arc<Mutex<BattleBallPowerUp>>> {
        self.power_up.upgrade()
    }
}

impl GameObject for PowerObject {
    /// Mirrors `serialiseObject(NettyResponse)`.
    fn serialise_object(&self, response: &mut NettyResponse) {
        let Some(power_up) = self.power_up.upgrade() else {
            return;
        };
        let power_up = power_up.lock();
        let tile = power_up.get_tile().lock();

        response.write_int(power_up.get_id());
        response.write_int(power_up.get_time_to_despawn());
        response.write_int(power_up.get_player_holding());
        response.write_int(power_up.get_power_type().get_power_up_id());
        response.write_int(tile.get_position().get_x());
        response.write_int(tile.get_position().get_y());
        response.write_int(tile.get_position().get_z() as i32);
    }

    /// Mirrors `getGameObjectType()`.
    fn get_game_object_type(&self) -> GameObjectType {
        GameObjectType::BattleballPowerObject
    }

    /// Mirrors `getId()`.
    fn get_id(&self) -> i32 {
        self.power_up
            .upgrade()
            .map(|power_up| power_up.lock().get_id())
            .unwrap_or(-1)
    }
}
