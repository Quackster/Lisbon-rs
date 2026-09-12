//! Mirrors `net.h4bbo.lisbon.messages.incoming.user.GETAVAILABLESETS`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::handshake::available_sets::AVAILABLE_SETS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::config::game_configuration::GameConfiguration;

#[allow(non_camel_case_types)]
pub struct GETAVAILABLESETS;

impl MessageEvent for GETAVAILABLESETS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let parts_key = if player.get_details().has_club_subscription() {
            "users.figure.parts.club"
        } else {
            "users.figure.parts.default"
        };
        let parts = format!("[{}]", GameConfiguration::get_instance().get_string(parts_key));
        player.send(&AVAILABLE_SETS::new(parts));

        Ok(())
    }
}
