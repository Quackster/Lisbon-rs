//! Mirrors `net.h4bbo.lisbon.messages.incoming.handshake.SSO`.
use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct SSO;

impl MessageEvent for SSO {
    /// Not dispatched; the real handling lives in `handle_mut`, which the
    /// connection dispatcher invokes with exclusive `Player` access.
    fn handle(&self, _player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        Ok(())
    }

    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle_mut(&self, player: &mut Player, reader: &mut NettyRequest) -> Result<(), String> {
        if player.is_logged_in() {
            return Ok(());
        }

        let ticket = reader.read_string();

        if PlayerDao::login_ticket(&mut player.get_details_mut(), &ticket) {
            player.login();
        } else {
            player.kick_from_server();
        }

        Ok(())
    }
}
