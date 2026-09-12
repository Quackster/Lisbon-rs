//! Mirrors `net.h4bbo.lisbon.messages.incoming.moderation.REQUEST_CFH`.
use crate::game::entity::entity::Entity;
use crate::game::moderation::cfh::call_for_help_manager::CallForHelpManager;
use crate::game::player::player::Player;
use crate::messages::outgoing::moderation::cfh_ack::CFH_ACK;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct REQUEST_CFH;

impl MessageEvent for REQUEST_CFH {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        // Retrieve open calls from the current user
        let call = CallForHelpManager::get_instance()
            .get_pending_call(player.get_details().get_id());

        // Send details
        player.send(&CFH_ACK::new(call));

        Ok(())
    }
}
