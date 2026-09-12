//! Mirrors `net.h4bbo.lisbon.messages.outgoing.games.GAMEINSTANCE`.
use crate::game::entity::entity::Entity;
use crate::game::games::enums::game_state::GameState;
use crate::game::games::enums::game_type::GameType;
use crate::game::games::game::Game;
use crate::game::games::history::game_history::GameHistory;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct GAMEINSTANCE {
    game: Option<Game>,
    finished_game: Option<GameHistory>,
}

impl GAMEINSTANCE {
    /// Mirrors the `GAMEINSTANCE(Game)` constructor.
    pub fn new_game(game: Game) -> Self {
        Self {
            game: Some(game),
            finished_game: None,
        }
    }

    /// Mirrors the `GAMEINSTANCE(GameHistory)` constructor.
    pub fn new_history(finished_game: GameHistory) -> Self {
        Self {
            game: None,
            finished_game: Some(finished_game),
        }
    }
}

impl MessageComposer for GAMEINSTANCE {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        if self.finished_game.is_none() {
            let game = self.game.as_ref().unwrap();
            response.write_int(game.get_game_state().get_state_id());

            let write_allowed_power_ups = |response: &mut NettyResponse, game: &Game| {
                if let Some(battleball_game) = game.as_battle_ball() {
                    let allowed_power_ups = battleball_game.get_allowed_power_ups();

                    let power_ups = allowed_power_ups
                        .iter()
                        .map(|power_up| power_up.to_string())
                        .collect::<Vec<_>>()
                        .join(",");
                    response.write_string(power_ups);
                }
            };

            if game.get_game_state() == GameState::Waiting {
                response.write_int(game.get_id());
                response.write_string(game.get_name());

                // Host
                response.write_int(game.get_game_creator_id());
                response.write_string(game.get_game_creator());

                if game.get_game_type() == GameType::Snowstorm {
                    if let Some(snow_storm_game) = game.as_snow_storm() {
                        response.write_int(snow_storm_game.get_game_length_choice());
                    }
                }

                response.write_int(game.get_map_id());
                response.write_int(game.get_spectators().len() as i32);
                response.write_int(game.get_team_amount());

                for i in 0..game.get_team_amount() {
                    let Some(team) = game.get_team(i) else {
                        // Java `NullPointerException` equivalent.
                        continue;
                    };
                    let players = team.lock().get_players();

                    response.write_int(players.len() as i32);

                    for player in players {
                        let player = player.lock();
                        let instance_id = player
                            .get_player()
                            .lock()
                            .get_room_user()
                            .map(|room_user| room_user.get_instance_id())
                            .unwrap_or(-1);
                        response.write_int(instance_id);
                        response.write_string(
                            player.get_player().lock().get_details().get_name(),
                        );
                    }
                }

                write_allowed_power_ups(response, game);
            }

            if game.get_game_state() == GameState::Started {
                response.write_int(game.get_id());
                response.write_string(game.get_name());
                response.write_string(game.get_game_creator());

                if game.get_game_type() == GameType::Snowstorm {
                    if let Some(snow_storm_game) = game.as_snow_storm() {
                        response.write_int(snow_storm_game.get_game_length_choice());
                    }
                }

                response.write_int(game.get_map_id());
                response.write_int(game.get_team_amount());

                for i in 0..game.get_team_amount() {
                    let Some(team) = game.get_team(i) else {
                        // Java `NullPointerException` equivalent.
                        continue;
                    };
                    let players = team.lock().get_active_players();

                    response.write_int(players.len() as i32);

                    for player in players {
                        //response.writeInt(player.getPlayer().getRoomUser().getInstanceId());
                        response.write_string(
                            player.lock().get_player().lock().get_details().get_name(),
                        );
                    }
                }

                write_allowed_power_ups(response, game);
            }
        } else {
            let finished_game = self.finished_game.as_ref().unwrap();

            response.write_int(GameState::Ended.get_state_id());
            response.write_int(finished_game.get_id());
            response.write_string(finished_game.get_name());
            response.write_string(finished_game.get_game_creator());

            if finished_game.get_game_type() == Some(GameType::Snowstorm) {
                response.write_int(
                    finished_game
                        .get_extra_data()
                        .parse::<i32>()
                        .unwrap_or(0),
                );
            }

            if let Some(history_data) = finished_game.get_history_data() {
                let mut team_data = history_data.get_team_data();

                response.write_int(finished_game.get_map_id());
                response.write_int(team_data.len() as i32);

                for players in team_data.values_mut() {
                    let team_score: i32 = players.iter().map(|p| p.get_score()).sum();

                    response.write_int(players.len() as i32);

                    for player in players.iter_mut() {
                        response.write_string(player.get_name().unwrap_or(""));
                        response.write_int(player.get_score());
                    }

                    response.write_int(team_score);
                }
            }

            if finished_game.get_game_type() == Some(GameType::Battleball) {
                let allowed_power_ups = finished_game.get_allowed_power_ups();
                let power_ups = allowed_power_ups
                    .iter()
                    .map(|power_up| power_up.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                response.write_string(power_ups);
            }
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        233 // "Ci"
    }
}
