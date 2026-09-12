//! Mirrors `net.h4bbo.lisbon.messages.outgoing.games.GAMEEND`.
use std::collections::HashMap;

use crate::game::entity::entity::Entity;
use crate::game::games::enums::game_type::GameType;
use crate::game::games::game_manager::GameManager;
use crate::game::games::player::game_team::GameTeam;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct GAMEEND {
    game_type: GameType,
    teams: HashMap<i32, GameTeam>,
}

impl GAMEEND {
    /// Mirrors the `GAMEEND(GameType, Map<Integer, GameTeam>)` constructor.
    pub fn new(game_type: GameType, teams: HashMap<i32, GameTeam>) -> Self {
        Self { game_type, teams }
    }
}

impl MessageComposer for GAMEEND {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(
            GameManager::get_instance()
                .get_restart_seconds(self.game_type),
        );
        response.write_int(self.teams.len() as i32);

        for team in self.teams.values() {
            let players = team.get_players();
            response.write_int(players.len() as i32);

            if !players.is_empty() {
                for game_player in &players {
                    let game_player = game_player.lock();
                    response.write_int(game_player.get_object_id());
                    response.write_string(
                        game_player
                            .get_player()
                            .lock()
                            .get_details()
                            .get_name(),
                    );
                    response.write_int(game_player.get_score());
                }

                response.write_int(team.get_points());
            }
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        248
    }
}
