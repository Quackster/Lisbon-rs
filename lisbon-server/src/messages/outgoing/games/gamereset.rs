//! Mirrors `net.h4bbo.lisbon.messages.outgoing.games.GAMERESET`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::games::enums::game_type::GameType;
use crate::game::games::game::Game;
use crate::game::games::player::game_player::GamePlayer;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct GAMERESET {
    time_until_game_start: i32,
    game_player_list: Vec<Arc<Mutex<GamePlayer>>>,
    game: Game,
}

impl GAMERESET {
    /// Mirrors the `GAMERESET(int, List<GamePlayer>, Game)` constructor.
    pub fn new(
        time_until_game_start: i32,
        game_player_list: Vec<Arc<Mutex<GamePlayer>>>,
        game: Game,
    ) -> Self {
        Self {
            time_until_game_start,
            game_player_list,
            game,
        }
    }
}

impl MessageComposer for GAMERESET {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.time_until_game_start);

        if self.game.get_game_type() == GameType::Battleball {
            response.write_int(self.game_player_list.len() as i32);

            for game_player in &self.game_player_list {
                let game_player = game_player.lock();
                response.write_int(game_player.get_object_id());

                let player = game_player.get_player().lock();
                if let Some(room_user) = player.get_room_user() {
                    let position = room_user.get_position();
                    response.write_int(position.get_x());
                    response.write_int(position.get_y());
                    response.write_int(position.get_rotation());
                }
            }
        } else {
            let objects = self.game.get_objects();
            response.write_int(objects.len() as i32);

            for game_object in &objects {
                game_object.serialise_object(response);
            }
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        249
    }
}
