//! Mirrors `net.h4bbo.lisbon.game.games.battleball.objects.PowerUpUpdateObject`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::games::battleball::battle_ball_power_up::BattleBallPowerUp;
use crate::game::games::enums::game_object_type::GameObjectType;
use crate::game::games::game_object::GameObject;
use crate::server::netty::streams::NettyResponse;

pub struct PowerUpUpdateObject {
    power_up: Arc<Mutex<BattleBallPowerUp>>,
}

impl PowerUpUpdateObject {
    /// Mirrors the `PowerUpUpdateObject(BattleBallPowerUp)` constructor.
    pub fn new(power_up: Arc<Mutex<BattleBallPowerUp>>) -> Self {
        Self { power_up }
    }

    /// Mirrors `getPowerUp()`.
    pub fn get_power_up(&self) -> &Arc<Mutex<BattleBallPowerUp>> {
        &self.power_up
    }
}

impl GameObject for PowerUpUpdateObject {
    /// Mirrors `serialiseObject(NettyResponse)`.
    fn serialise_object(&self, response: &mut NettyResponse) {
        let power_up = self.power_up.lock();

        response.write_int(power_up.get_id());
        response.write_int(power_up.get_time_to_despawn());
        response.write_int(power_up.get_player_holding());
    }

    /// Mirrors `getGameObjectType()`.
    fn get_game_object_type(&self) -> GameObjectType {
        GameObjectType::BattleballPowerObject
    }

    /// Mirrors `getId()`.
    fn get_id(&self) -> i32 {
        self.power_up.lock().get_id()
    }
}
