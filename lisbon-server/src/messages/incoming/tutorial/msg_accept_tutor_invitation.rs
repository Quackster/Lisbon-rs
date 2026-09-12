//! Mirrors `net.h4bbo.lisbon.messages.incoming.tutorial.MSG_ACCEPT_TUTOR_INVITATION`.
use crate::game::entity::entity::Entity;
use crate::game::guides::guide_manager::GuideManager;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::outgoing::tutorial::invite_follow_failed::INVITE_FOLLOW_FAILED;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MSG_ACCEPT_TUTOR_INVITATION;

impl MessageEvent for MSG_ACCEPT_TUTOR_INVITATION {
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
            player.send(&INVITE_FOLLOW_FAILED);
            return Ok(());
        };

        let newb = newb_guard.lock();
        let Some(newb_room) = newb.get_room_user().and_then(|e| e.get_room()) else {
            player.send(&INVITE_FOLLOW_FAILED);
            return Ok(());
        };

        if !newb_room.is_owner(newb.get_details().get_id()) {
            player.send(&INVITE_FOLLOW_FAILED);
            return Ok(());
        }

        player.get_guide_manager().remove_invite(user_id);
        player.get_guide_manager().set_invited_by(newb.get_details().get_id());

        let player_room = player.get_room_user().and_then(|e| e.get_room());

        if player_room.is_none()
            || player_room
                .as_ref()
                .map_or(false, |room| room.get_id() != newb_room.get_id())
        {
            if let Some(room_user) = player.get_room_user() {
                room_user.set_authenticate_id(newb_room.get_id());
            }
            newb_room.forward(player, false);
        } else {
            GuideManager::get_instance().tutor_enter_room(player, &*newb);
        }

        Ok(())
    }
}
