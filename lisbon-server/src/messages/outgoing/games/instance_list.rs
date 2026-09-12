//! Mirrors `net.h4bbo.lisbon.messages.outgoing.games.INSTANCELIST`.
use crate::game::games::enums::game_state::GameState;
use crate::game::games::enums::game_type::GameType;
use crate::game::games::game::Game;
use crate::game::games::history::game_history::GameHistory;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct INSTANCELIST {
    created_games: Vec<Game>,
    started_games: Vec<Game>,
    finished_games: Vec<GameHistory>,
}

impl INSTANCELIST {
    /// Mirrors the `INSTANCELIST(List<Game>, List<GameHistory>)`
    /// constructor.
    pub fn new(games_by_type: Vec<Game>, finished_games: Vec<GameHistory>) -> Self {
        let created_games = games_by_type
            .iter()
            .filter(|game| game.get_game_state() == GameState::Waiting)
            .cloned()
            .collect();
        let started_games = games_by_type
            .iter()
            .filter(|game| game.get_game_state() == GameState::Started)
            .cloned()
            .collect();

        Self {
            created_games,
            started_games,
            finished_games,
        }
    }
}

impl MessageComposer for INSTANCELIST {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.created_games.len() as i32);

        for game in &self.created_games {
            response.write_int(game.get_id());
            response.write_string(&game.get_name());

            response.write_int(game.get_game_creator_id());
            response.write_string(&game.get_game_creator());

            if game.get_game_type() == GameType::Snowstorm {
                if let Some(snow_storm_game) = game.as_snow_storm() {
                    response.write_int(snow_storm_game.get_game_length());
                }
            }

            response.write_int(game.get_map_id());
        }

        response.write_int(self.started_games.len() as i32);

        for game in &self.started_games {
            response.write_int(game.get_id());
            response.write_string(&game.get_name());
            response.write_string(&game.get_game_creator());

            if game.get_game_type() == GameType::Snowstorm {
                if let Some(snow_storm_game) = game.as_snow_storm() {
                    response.write_int(snow_storm_game.get_game_length());
                }
            }

            response.write_int(game.get_map_id());
        }

        response.write_int(self.finished_games.len() as i32);

        for game in &self.finished_games {
            response.write_int(game.get_id());
            response.write_string(game.get_name());
            response.write_string(game.get_game_creator());

            if game.get_game_type() == Some(GameType::Snowstorm) {
                response.write_int(game.get_game_length());
            }

            response.write_int(game.get_map_id());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        232 // "Ch"
    }
}
