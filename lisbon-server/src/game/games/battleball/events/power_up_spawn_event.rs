//! Mirrors `net.h4bbo.lisbon.game.games.battleball.events.PowerUpSpawnEvent`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::games::game_object::GameObject;
use crate::game::games::battleball::battle_ball_power_up::BattleBallPowerUp;
use crate::game::games::battleball::objects::power_object::PowerObject;
use crate::game::games::enums::game_event_type::GameEventType;
use crate::game::games::game_event::GameEvent;
use crate::server::netty::streams::NettyResponse;

pub struct PowerUpSpawnEvent {
    power_up: Arc<Mutex<BattleBallPowerUp>>,
}

impl PowerUpSpawnEvent {
    /// Mirrors the `PowerUpSpawnEvent(BattleBallPowerUp)` constructor.
    pub fn new(power_up: Arc<Mutex<BattleBallPowerUp>>) -> Self {
        Self { power_up }
    }
}

impl GameEvent for PowerUpSpawnEvent {
    /// Mirrors `serialiseEvent(NettyResponse)`.
    fn serialise_event(&self, response: &mut NettyResponse) {
        response.write_int(1);
        PowerObject::new(Arc::downgrade(&self.power_up)).serialise_object(response);
    }

    /// Mirrors `getGameEventType()`.
    fn get_game_event_type(&self) -> GameEventType {
        GameEventType::BattleballObjectSpawn
    }
}
