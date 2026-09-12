//! Mirrors `net.h4bbo.lisbon.game.games.battleball.events.ActivatePowerUpEvent`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::games::battleball::battle_ball_power_up::BattleBallPowerUp;
use crate::game::games::enums::game_event_type::GameEventType;
use crate::game::games::game_event::GameEvent;
use crate::game::games::player::game_player::GamePlayer;
use crate::server::netty::streams::NettyResponse;

pub struct ActivatePowerUpEvent {
    power_up: Arc<Mutex<BattleBallPowerUp>>,
    game_player: Arc<Mutex<GamePlayer>>,
}

impl ActivatePowerUpEvent {
    /// Mirrors the `ActivatePowerUpEvent(GamePlayer, BattleBallPowerUp)` constructor.
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

impl GameEvent for ActivatePowerUpEvent {
    /// Mirrors `serialiseEvent(NettyResponse)`.
    fn serialise_event(&self, response: &mut NettyResponse) {
        let game_player = self.game_player.lock();
        let player = game_player.get_player().lock();

        let position = player
            .get_room_user()
            .map(|room_user| room_user.get_position())
            .unwrap_or_default();

        response.write_int(game_player.get_object_id());
        response.write_int(self.power_up.lock().get_id());
        response.write_int(position.get_rotation());
        response.write_int(
            self.power_up
                .lock()
                .get_power_type()
                .get_power_up_id(),
        );
    }

    /// Mirrors `getGameEventType()`.
    fn get_game_event_type(&self) -> GameEventType {
        GameEventType::BattleballPowerUpActivate
    }
}
