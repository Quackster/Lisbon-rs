//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.moderation.LETUSERIN`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::outgoing::rooms::flatnotallowedtoenter::FLATNOTALLOWEDTOENTER;
use crate::messages::outgoing::rooms::flat_letin::FLAT_LETIN;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct LETUSERIN;

impl MessageEvent for LETUSERIN {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room) = player
            .get_room_user()
            .and_then(|room_user| room_user.get_room())
        else {
            return Ok(());
        };

        if !room.has_rights(player.get_details().get_id()) {
            return Ok(());
        }

        let username = reader.read_string();
        let success_token = reader.contents().unwrap_or_default();
        let can_enter = success_token == "A";

        let Some(entering) = PlayerManager::get_instance().get_player_by_name(&username) else {
            return Ok(());
        };
        let entering = entering.lock();

        if can_enter {
            entering
                .get_room_user()
                .map(|room_user| room_user.set_authenticate_id(room.get_id()));
            entering.send(&FLAT_LETIN);
        } else {
            entering.send(&FLATNOTALLOWEDTOENTER);
        }

        Ok(())
    }
}
