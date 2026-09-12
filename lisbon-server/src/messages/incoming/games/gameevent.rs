//! Mirrors `net.h4bbo.lisbon.messages.incoming.games.GAMEEVENT`.
use std::sync::Arc;

use crate::game::entity::entity::Entity;
use crate::game::games::battleball::events::activate_power_up_event::ActivatePowerUpEvent;
use crate::game::games::enums::game_type::GameType;
use crate::game::games::snowstorm::messages::snow_storm_message_handler::SnowStormMessageHandler;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GAMEEVENT;

impl MessageEvent for GAMEEVENT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };

        if !room_user.is_walking_allowed() {
            return Ok(());
        }

        let Some(game_player_arc) = room_user.get_game_player() else {
            return Ok(());
        };
        let Some(game) = game_player_arc.lock().get_game() else {
            return Ok(());
        };

        let event_type = reader.read_int(); // Instance ID? Useless?

        if game.get_game_type() == GameType::Snowstorm {
            let Some(snow_storm_game) = game.as_snow_storm() else {
                return Ok(());
            };

            SnowStormMessageHandler::get_instance()
                .handle_message(event_type, reader, &snow_storm_game, &game_player_arc);
        } else {
            // Jump request
            if event_type == 2 {
                if !room_user.is_walking_allowed() {
                    return Ok(());
                }

                let x = reader.read_int();
                let y = reader.read_int();

                room_user.walk_to(x, y);
            }

            // Use power up request
            if event_type == 4 {
                let power_id = reader.read_int();

                if let Some(battleball_game) = game.as_battle_ball() {
                    let user_id = game_player_arc.lock().get_user_id();

                    if !battleball_game
                        .get_stored_powers()
                        .lock()
                        .contains_key(&user_id)
                    {
                        return Ok(());
                    }

                    let power_list = battleball_game
                        .get_stored_powers()
                        .lock()
                        .get(&user_id)
                        .cloned()
                        .unwrap_or_default();

                    let power_up = power_list
                        .iter()
                        .find(|power| power.lock().get_id() == power_id);

                    if let Some(power_up) = power_up {
                        battleball_game
                            .get_events_queue()
                            .lock()
                            .push(Box::new(ActivatePowerUpEvent::new(
                                Arc::clone(&game_player_arc),
                                Arc::clone(power_up),
                            )));

                        battleball_game
                            .get_stored_powers()
                            .lock()
                            .get_mut(&user_id)
                            .map(|list| {
                                list.retain(|power| power.lock().get_id() != power_id);
                            });

                        let position = room_user.get_position();
                        power_up.lock().use_power(Arc::clone(&game_player_arc), &position);
                    }
                }
            }
        }

        Ok(())
    }
}
