//! Mirrors `net.h4bbo.lisbon.messages.incoming.games.INITIATECREATEGAME`.
use crate::game::entity::entity::Entity;
use crate::game::games::enums::game_type::GameType;
use crate::game::games::game_parameter::GameParameter;
use crate::game::player::player::Player;
use crate::game::triggers::game_lobby_trigger::GameLobbyTrigger;
use crate::messages::outgoing::games::gameparameters::GAMEPARAMETERS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct INITIATECREATEGAME;

impl MessageEvent for INITIATECREATEGAME {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
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

        let game_type = game_lobby_trigger.get_game_type();
        let mut parameters: Vec<GameParameter> = Vec::new();

        if game_type == GameType::Battleball {
            parameters = vec![
                GameParameter::with_min_max("fieldType", true, "1", 1, 5),
                GameParameter::with_min_max("numTeams", true, "2", 2, 4),
                GameParameter::new("allowedPowerups", true, "1,2,3,4,5,6,7,8"),
                GameParameter::new("name", true, ""),
            ];
        }

        if game_type == GameType::Snowstorm {
            parameters = vec![
                GameParameter::with_min_max("fieldType", true, "1", 1, 7),
                GameParameter::with_min_max("numTeams", true, "2", 2, 4),
                GameParameter::with_min_max("gameLengthChoice", true, "1", 1, 3),
                GameParameter::new("name", true, ""),
            ];
        }

        if !parameters.is_empty() {
            player.send(&GAMEPARAMETERS::new(parameters));
        }

        Ok(())
    }
}
