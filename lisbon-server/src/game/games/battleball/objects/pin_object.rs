//! Mirrors `net.h4bbo.lisbon.game.games.battleball.objects.PinObject`.
use crate::game::games::enums::game_object_type::GameObjectType;
use crate::game::games::game_object::GameObject;
use crate::game::pathfinder::position::Position;
use crate::server::netty::streams::NettyResponse;

pub struct PinObject {
    id: i32,
    position: Position,
}

impl PinObject {
    /// Mirrors the `PinObject(int, Position)` constructor.
    pub fn new(id: i32, position: Position) -> Self {
        Self { id, position }
    }

    /// Mirrors `getPosition()`.
    pub fn get_position(&self) -> &Position {
        &self.position
    }
}

impl GameObject for PinObject {
    /// Mirrors `serialiseObject(NettyResponse)`.
    fn serialise_object(&self, response: &mut NettyResponse) {
        response.write_int(self.get_id());
        response.write_int(self.position.get_x());
        response.write_int(self.position.get_y());
        response.write_int(self.position.get_z() as i32);
    }

    /// Mirrors `getGameObjectType()`.
    fn get_game_object_type(&self) -> GameObjectType {
        GameObjectType::BattleballPinObject
    }

    /// Mirrors `getId()`.
    fn get_id(&self) -> i32 {
        self.id
    }
}
