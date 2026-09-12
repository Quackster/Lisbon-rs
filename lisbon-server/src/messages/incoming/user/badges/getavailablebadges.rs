//! Mirrors `net.h4bbo.lisbon.messages.incoming.user.badges.GETAVAILABLEBADGES`.
use crate::game::player::player::Player;
use crate::messages::outgoing::guides::init_tutor_service_status::INIT_TUTOR_SERVICE_STATUS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GETAVAILABLEBADGES;

impl MessageEvent for GETAVAILABLEBADGES {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        if !player.is_logged_in() {
            return Ok(());
        }

        player.get_badge_manager().refresh_badges(player);
        player.get_achievement_manager().process_achievements(player, true);

        if player.get_guide_manager().is_guide() {
            player.send(&INIT_TUTOR_SERVICE_STATUS::new(1));
        }

        Ok(())
    }
}
