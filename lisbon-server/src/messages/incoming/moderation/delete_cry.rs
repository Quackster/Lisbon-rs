//! Mirrors `net.h4bbo.lisbon.messages.incoming.moderation.DELETE_CRY`.
use crate::game::entity::entity::Entity;
use crate::game::moderation::cfh::call_for_help_manager::CallForHelpManager;
use crate::game::player::player::Player;
use crate::messages::outgoing::moderation::cfh_ack::CFH_ACK;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct DELETE_CRY;

impl MessageEvent for DELETE_CRY {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        // Retrieve open calls for current user
        let Some(cfh) = CallForHelpManager::get_instance()
            .get_pending_call(player.get_details().get_id())
        else {
            return Ok(());
        };

        // Delete call for help
        CallForHelpManager::get_instance().delete_call(&cfh);

        // Notify client about the deleted call for help
        player.send(&CFH_ACK::new(None));

        Ok(())
    }
}
