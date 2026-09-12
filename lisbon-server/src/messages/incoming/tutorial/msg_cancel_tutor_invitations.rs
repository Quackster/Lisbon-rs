//! Mirrors `net.h4bbo.lisbon.messages.incoming.tutorial.MSG_CANCEL_TUTOR_INVITATIONS`.
use crate::game::guides::guide_manager::GuideManager;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MSG_CANCEL_TUTOR_INVITATIONS;

impl MessageEvent for MSG_CANCEL_TUTOR_INVITATIONS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        if !player.get_guide_manager().is_guidable() {
            return Ok(());
        }

        if !player.get_guide_manager().is_waiting_for_guide() {
            return Ok(());
        }

        player.get_guide_manager().set_waiting_for_guide(false);
        player.get_guide_manager().set_guidable(false);
        let mut invited = player.get_guide_manager().get_invited();
        invited.clear();
        GuideManager::get_instance().try_clear_tutorial(player);

        Ok(())
    }
}
