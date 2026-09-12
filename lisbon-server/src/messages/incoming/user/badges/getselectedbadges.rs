//! Mirrors `net.h4bbo.lisbon.messages.incoming.user.badges.GETSELECTEDBADGES`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::outgoing::user::badges::userbadge::USERBADGE;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GETSELECTEDBADGES;

impl MessageEvent for GETSELECTEDBADGES {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        if reader.contents().is_none_or(|content| content.is_empty()) {
            return Ok(());
        }

        if player.get_room_user().and_then(|e| e.get_room()).is_none() {
            return Ok(());
        }

        let user_id = reader.read_int();

        let Some(badge_player) = PlayerManager::get_instance().get_player_by_id(user_id) else {
            return Ok(());
        };

        let badge_player = badge_player.lock();
        let equipped_badges = badge_player.get_badge_manager().get_equipped_badges();
        player.send(&USERBADGE::new(user_id, equipped_badges));

        Ok(())
    }
}
