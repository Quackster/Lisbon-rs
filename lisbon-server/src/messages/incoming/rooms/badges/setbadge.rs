//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.badges.SETBADGE`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::user::badges::userbadge::USERBADGE;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct SETBADGE;

impl MessageEvent for SETBADGE {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        // Unequip all previous badges
        for badge in player.get_badge_manager().get_badges() {
            player
                .get_badge_manager()
                .change_badge(badge.get_badge_code(), false, 0);
        }

        // Equip new badges
        while !reader.remaining_bytes().is_empty() {
            let slot_id = reader.read_int();
            let badge_code = reader.read_string();

            if slot_id > 0 && slot_id < 6 && !badge_code.is_empty() {
                player.get_badge_manager().change_badge(&badge_code, true, slot_id);
            }
        }

        // Notify users of badge updates
        if let Some(room_user) = player.get_room_user() {
            if let Some(room) = room_user.get_room() {
                room.send(&USERBADGE::new(
                    player.get_details().get_id(),
                    player.get_badge_manager().get_equipped_badges(),
                ));
            }
        }

        player.get_badge_manager().refresh_badges(player);
        player.get_badge_manager().save_queued_badges();

        Ok(())
    }
}
