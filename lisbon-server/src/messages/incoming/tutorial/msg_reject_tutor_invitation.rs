//! Mirrors `net.h4bbo.lisbon.messages.incoming.tutorial.MSG_REJECT_TUTOR_INVITATION`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::outgoing::tutorial::invite_cancelled::INVITE_CANCELLED;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MSG_REJECT_TUTOR_INVITATION;

impl MessageEvent for MSG_REJECT_TUTOR_INVITATION {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        if !player.get_guide_manager().is_guide() {
            return Ok(());
        }

        if player.get_guide_manager().get_invites().is_empty() {
            return Ok(());
        }

        let data = reader.read_string();

        if data.is_empty() || !data.chars().all(|c| c.is_ascii_digit()) {
            return Ok(());
        }

        let user_id = match data.parse::<i32>() {
            Ok(value) => value,
            Err(_) => return Ok(()),
        };

        if !player.get_guide_manager().has_invite(user_id) {
            return Ok(());
        }

        let Some(newb_guard) = PlayerManager::get_instance().get_player_by_id(user_id) else {
            return Ok(());
        };

        let newb = newb_guard.lock();
        if newb.get_room_user().and_then(|e| e.get_room()).is_none() {
            return Ok(());
        }

        player.get_guide_manager().remove_invite(user_id);
        player.send(&INVITE_CANCELLED);

        Ok(())
    }
}
