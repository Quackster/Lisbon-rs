//! Mirrors `net.h4bbo.lisbon.messages.outgoing.games.FULLGAMESTATUS`.
use crate::game::games::enums::game_state::GameState;
use crate::game::games::enums::game_type::GameType;
use crate::game::games::game::Game;
use crate::game::games::game_manager::GameManager;
use crate::messages::outgoing::games::snowstorm_game_status::SNOWSTORM_GAMESTATUS;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct FULLGAMESTATUS {
    game: Game,
}

impl FULLGAMESTATUS {
    /// Mirrors the `FULLGAMESTATUS(Game)` constructor.
    pub fn new(game: Game) -> Self {
        Self { game }
    }
}

impl MessageComposer for FULLGAMESTATUS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        if self.game.get_game_type() == GameType::Battleball {
            response.write_int(GameState::Started.get_state_id());
            response.write_int(self.game.get_preparing_game_seconds_left());
            response.write_int(
                GameManager::get_instance()
                    .get_preparing_seconds(self.game.get_game_type()),
            );

            let objects = self.game.get_objects();
            // TODO: Objects here (inherited from the Java).
            response.write_int(objects.len() as i32);

            for game_object in &objects {
                response.write_int(
                    game_object
                        .get_game_object_type()
                        .get_object_id(),
                ); // type, 0 = player
                game_object.serialise_object(response);
            }

            if let Some(room_model) = self.game.get_room_model() {
                response.write_int(room_model.get_map_size_y());
                response.write_int(room_model.get_map_size_x());

                for y in 0..room_model.get_map_size_y() {
                    for x in 0..room_model.get_map_size_x() {
                        match self.game.get_tile(x, y) {
                            None => {
                                response.write_int(-1);
                                response.write_int(0);
                            }
                            Some(tile) => {
                                let tile = tile.lock();
                                response.write_int(tile.get_colour().get_colour_id());
                                response.write_int(tile.get_state().get_tile_state_id());
                            }
                        }
                    }
                }

                response.write_int(1);
                // TODO: Show events on game load (inherited from the
                // Java).
                response.write_int(0);
            }
        } else {
            let objects = self.game.get_objects();
            let turns = self
                .game
                .get_update_task()
                .map(|task| task.get_executing_turns().lock().clone())
                .unwrap_or_default();

            response.write_int(self.game.get_game_state().get_state_id());
            response.write_int(self.game.get_preparing_game_seconds_left());
            response.write_int(
                GameManager::get_instance()
                    .get_preparing_seconds(self.game.get_game_type()),
            );
            response.write_int(objects.len() as i32); // TODO: Objects here

            for obj in &objects {
                obj.serialise_object(response);
            }

            response.write_bool(false);
            response.write_int(self.game.get_team_amount());

            SNOWSTORM_GAMESTATUS::new(turns).compose(response);
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        243 // "Cs"
    }
}
