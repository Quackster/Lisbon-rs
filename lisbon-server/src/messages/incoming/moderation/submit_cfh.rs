//! Mirrors `net.h4bbo.lisbon.messages.incoming.moderation.SUBMIT_CFH`.
use crate::game::entity::entity::Entity;
use crate::game::moderation::cfh::call_for_help_manager::CallForHelpManager;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct SUBMIT_CFH;

impl MessageEvent for SUBMIT_CFH {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        if player.get_room_user().and_then(|room_user| room_user.get_room()).is_none() {
            return Ok(());
        }

        let message = reader.read_string();

        if message.is_empty() {
            return Ok(());
        }

        // Only allow one call for help per user
        if CallForHelpManager::get_instance().has_pending_call(player) {
            return Ok(());
        }

        CallForHelpManager::get_instance().submit_call(player, &message);

        Ok(())
    }
}
