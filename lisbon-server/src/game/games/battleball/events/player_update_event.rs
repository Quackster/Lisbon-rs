//! Mirrors `net.h4bbo.lisbon.game.games.battleball.events.PlayerUpdateEvent`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::games::game_object::GameObject;
use crate::game::games::battleball::objects::player_update_object::PlayerUpdateObject;
use crate::game::games::enums::game_event_type::GameEventType;
use crate::game::games::game_event::GameEvent;
use crate::game::games::player::game_player::GamePlayer;
use crate::server::netty::streams::NettyResponse;

pub struct PlayerUpdateEvent {
    game_player: Arc<Mutex<GamePlayer>>,
}

impl PlayerUpdateEvent {
    /// Mirrors the `PlayerUpdateEvent(GamePlayer)` constructor.
    pub fn new(game_player: Arc<Mutex<GamePlayer>>) -> Self {
        Self { game_player }
    }
}

impl GameEvent for PlayerUpdateEvent {
    /// Mirrors `serialiseEvent(NettyResponse)`.
    fn serialise_event(&self, response: &mut NettyResponse) {
        response.write_int(0);
        PlayerUpdateObject::new(Arc::clone(&self.game_player))
            .serialise_object(response);
    }

    /// Mirrors `getGameEventType()`.
    fn get_game_event_type(&self) -> GameEventType {
        GameEventType::BattleballObjectSpawn
    }
}
