//! Mirrors `net.h4bbo.lisbon.messages.incoming.games.GETINSTANCELIST`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::triggers::game_lobby_trigger::GameLobbyTrigger;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::config::game_configuration::GameConfiguration;

#[allow(non_camel_case_types)]
pub struct GETINSTANCELIST;

impl MessageEvent for GETINSTANCELIST {
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

        // Don't show panel and lounge info if create game is disabled
        let enabled = GameConfiguration::get_instance().get_bool(
            format!(
                "{}.create.game.enabled",
                game_lobby_trigger.get_game_type().get_name()
            )
            .as_str(),
        );
        if !enabled {
            return Ok(());
        }

        player.send(&game_lobby_trigger.get_instance_list());

        Ok(())
    }
}
