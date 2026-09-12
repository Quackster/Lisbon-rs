//! Mirrors `net.h4bbo.lisbon.messages.incoming.handshake.TRY_LOGIN`.
use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::player::player::Player;
use crate::messages::outgoing::alert::localised_error::LOCALISED_ERROR;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::string_util::StringUtil;

#[allow(non_camel_case_types)]
pub struct TRY_LOGIN;

impl MessageEvent for TRY_LOGIN {
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

        let username = StringUtil::filter_input(&reader.read_string(), true);
        let password = StringUtil::filter_input(&reader.read_string(), true);

        if PlayerDao::login(&mut player.get_details_mut(), &username, &password) {
            player.login();
        } else {
            player.send(&LOCALISED_ERROR::new("Login incorrect"));
        }

        Ok(())
    }
}
