//! Mirrors `net.h4bbo.lisbon.game.games.battleball.events.DespawnObjectEvent`.
use crate::game::games::enums::game_event_type::GameEventType;
use crate::game::games::game_event::GameEvent;
use crate::server::netty::streams::NettyResponse;

pub struct DespawnObjectEvent {
    game_object_id: i32,
}

impl DespawnObjectEvent {
    /// Mirrors the `DespawnObjectEvent(int)` constructor.
    pub fn new(game_object_id: i32) -> Self {
        Self { game_object_id }
    }
}

impl GameEvent for DespawnObjectEvent {
    /// Mirrors `serialiseEvent(NettyResponse)`.
    fn serialise_event(&self, response: &mut NettyResponse) {
        response.write_int(self.game_object_id);
    }

    /// Mirrors `getGameEventType()`.
    fn get_game_event_type(&self) -> GameEventType {
        GameEventType::BattleballDespawnObject
    }
}
