//! Mirrors `net.h4bbo.lisbon.messages.incoming.games.GAMEPARAMETERVALUES`.
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::triggers::game_lobby_trigger::GameLobbyTrigger;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::string_util::StringUtil;

#[allow(non_camel_case_types)]
pub struct GAMEPARAMETERVALUES;

impl MessageEvent for GAMEPARAMETERVALUES {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        // BcPA@IfieldTypeHI@HnumTeamsHJ@OallowedPowerupsI@O1,2,3,4,5,6,7,8@DnameI@DtestH
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        let Some(trigger) = room.get_model().and_then(|model| model.get_room_trigger()) else {
            return Ok(());
        };
        let Some(game_lobby_trigger) = trigger.as_game_lobby() else {
            return Ok(());
        };

        let mut game_parameters: HashMap<String, Box<dyn Any>> = HashMap::new();

        let parameters = reader.read_int();

        for _ in 0..parameters {
            let parameter = reader.read_string();
            let is_text_value = reader.read_boolean();

            let value: Box<dyn Any> = if is_text_value {
                Box::new(StringUtil::filter_input(&reader.read_string(), true))
            } else {
                Box::new(reader.read_int())
            };

            game_parameters.insert(parameter, value);
        }

        // The Java `createGame(player, ...)` uses the owning `Player`;
        // it is resolved back to the Arc via the entity manager.
        let player_id = player.get_details().get_id();
        let Some(player_arc) = room
            .get_entity_manager()
            .get_players()
            .into_iter()
            .find(|p| p.lock().get_details().get_id() == player_id)
        else {
            return Ok(());
        };

        game_lobby_trigger.create_game(&player_arc, &game_parameters);

        room.send(&game_lobby_trigger.get_instance_list());

        Ok(())
    }
}
