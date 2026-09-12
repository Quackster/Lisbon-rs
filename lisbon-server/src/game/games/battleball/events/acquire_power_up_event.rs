//! Mirrors `net.h4bbo.lisbon.game.games.battleball.events.AcquirePowerUpEvent`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::games::battleball::battle_ball_power_up::BattleBallPowerUp;
use crate::game::games::enums::game_event_type::GameEventType;
use crate::game::games::game_event::GameEvent;
use crate::game::games::player::game_player::GamePlayer;
use crate::server::netty::streams::NettyResponse;

pub struct AcquirePowerUpEvent {
    power_up: Arc<Mutex<BattleBallPowerUp>>,
    game_player: Arc<Mutex<GamePlayer>>,
}

impl AcquirePowerUpEvent {
    /// Mirrors the `AcquirePowerUpEvent(GamePlayer, BattleBallPowerUp)` constructor.
    pub fn new(
        game_player: Arc<Mutex<GamePlayer>>,
        power_up: Arc<Mutex<BattleBallPowerUp>>,
    ) -> Self {
        Self {
            power_up,
            game_player,
        }
    }
}

impl GameEvent for AcquirePowerUpEvent {
    /// Mirrors `serialiseEvent(NettyResponse)`.
    fn serialise_event(&self, response: &mut NettyResponse) {
        response.write_int(self.game_player.lock().get_object_id());
        response.write_int(self.power_up.lock().get_id());
        response.write_int(
            self.power_up
                .lock()
                .get_power_type()
                .get_power_up_id(),
        );
    }

    /// Mirrors `getGameEventType()`.
    fn get_game_event_type(&self) -> GameEventType {
        GameEventType::BattleballPowerUpGet
    }
}
