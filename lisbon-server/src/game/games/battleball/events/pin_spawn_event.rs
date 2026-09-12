//! Mirrors `net.h4bbo.lisbon.game.games.battleball.events.PinSpawnEvent`.
use crate::game::games::game_object::GameObject;
use crate::game::games::battleball::objects::pin_object::PinObject;
use crate::game::games::enums::game_event_type::GameEventType;
use crate::game::games::game_event::GameEvent;
use crate::game::pathfinder::position::Position;
use crate::server::netty::streams::NettyResponse;

pub struct PinSpawnEvent {
    id: i32,
    position: Position,
}

impl PinSpawnEvent {
    /// Mirrors the `PinSpawnEvent(int, Position)` constructor.
    pub fn new(id: i32, position: Position) -> Self {
        Self { id, position }
    }
}

impl GameEvent for PinSpawnEvent {
    /// Mirrors `serialiseEvent(NettyResponse)`.
    fn serialise_event(&self, response: &mut NettyResponse) {
        response.write_int(2);
        PinObject::new(self.id, self.position.clone()).serialise_object(response);
    }

    /// Mirrors `getGameEventType()`.
    fn get_game_event_type(&self) -> GameEventType {
        GameEventType::BattleballObjectSpawn
    }
}
