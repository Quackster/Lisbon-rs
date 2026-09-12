//! Mirrors `net.h4bbo.lisbon.game.games.battleball.objects.PlayerUpdateObject`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::games::enums::game_object_type::GameObjectType;
use crate::game::games::game_object::GameObject;
use crate::game::games::player::game_player::GamePlayer;
use crate::server::netty::streams::NettyResponse;

pub struct PlayerUpdateObject {
    game_player: Arc<Mutex<GamePlayer>>,
}

impl PlayerUpdateObject {
    /// Mirrors the `PlayerUpdateObject(GamePlayer)` constructor (the
    // object id is the `GamePlayer` user id in the Java source).
    pub fn new(game_player: Arc<Mutex<GamePlayer>>) -> Self {
        Self { game_player }
    }

    /// Mirrors the `gamePlayer.getUserId()` access used by the Java
    // constructor.
    pub fn get_user_id(&self) -> i32 {
        self.game_player.lock().get_user_id()
    }
}

impl GameObject for PlayerUpdateObject {
    /// Mirrors `serialiseObject(NettyResponse)`.
    fn serialise_object(&self, response: &mut NettyResponse) {
        let game_player = self.game_player.lock();
        let player = game_player.get_player().lock();

        let position = player
            .get_room_user()
            .map(|room_user| room_user.get_position())
            .unwrap_or_default();

        response.write_int(game_player.get_object_id());
        response.write_int(position.get_x());
        response.write_int(position.get_y());
        response.write_int(position.get_z() as i32);
        response.write_int(position.get_rotation());
        response.write_int(game_player.get_player_state().get_state_id());
        response.write_int(game_player.get_colouring_for_opponent_id());
    }

    /// Mirrors `getGameObjectType()`.
    fn get_game_object_type(&self) -> GameObjectType {
        GameObjectType::BattleballPlayerObject
    }

    /// Mirrors `getId()`.
    fn get_id(&self) -> i32 {
        self.game_player.lock().get_user_id()
    }
}
