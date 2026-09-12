//! Mirrors `net.h4bbo.lisbon.messages.incoming.tutorial.MSG_CANCEL_WAIT_FOR_TUTOR_INVITATIONS`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MSG_CANCEL_WAIT_FOR_TUTOR_INVITATIONS;

impl MessageEvent for MSG_CANCEL_WAIT_FOR_TUTOR_INVITATIONS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        if !player.get_guide_manager().is_guide() {
            return Ok(());
        }

        player.get_guide_manager().set_waiting_for_invitations(false);
        player.get_guide_manager().clear_invites();

        // Remove your user from the newbs that invited you
        let players = PlayerManager::get_instance().get_players();
        for player_entry in players.iter() {
            let entry = player_entry.lock();
            let mut invited = entry.get_guide_manager().get_invited();
            invited.retain(|id| *id != player.get_details().get_id());
        }

        Ok(())
    }
}
