//! Mirrors `net.h4bbo.lisbon.messages.incoming.user.GET_POSSIBLE_ACHIEVEMENTS`.
use crate::game::player::player::Player;
use crate::messages::outgoing::user::possible_achievements::POSSIBLE_ACHIEVEMENTS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GET_POSSIBLE_ACHIEVEMENTS;

impl MessageEvent for GET_POSSIBLE_ACHIEVEMENTS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let possible_achievements = player
            .get_achievement_manager()
            .get_possible_achievements();

        player.send(&POSSIBLE_ACHIEVEMENTS::new(possible_achievements));

        Ok(())
    }
}
