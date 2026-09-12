//! Mirrors `net.h4bbo.lisbon.messages.incoming.user.GET_INFO`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::user::user_object::USER_OBJECT;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GET_INFO;

impl MessageEvent for GET_INFO {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        if !player.is_logged_in() {
            return Ok(());
        }

        player.get_badge_manager().refresh_badges(player);
        player.get_achievement_manager().process_achievements(player, true);

        player.send(&USER_OBJECT::new(player.get_details().clone()));

        Ok(())
    }
}
