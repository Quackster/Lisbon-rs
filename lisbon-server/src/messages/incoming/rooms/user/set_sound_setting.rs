//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.user.SET_SOUND_SETTING`.
use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct SET_SOUND_SETTING;

impl MessageEvent for SET_SOUND_SETTING {
    /// Not dispatched; the real handling lives in `handle_mut`, which the
    /// connection dispatcher invokes with exclusive `Player` access.
    fn handle(&self, _player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        Ok(())
    }

    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle_mut(&self, player: &mut Player, reader: &mut NettyRequest) -> Result<(), String> {
        let enabled = reader.read_boolean();
        let user_id = player.get_details_mut().get_id();
        player.get_details_mut().set_sound_setting(enabled);

        PlayerDao::save_sound_setting(user_id, enabled);

        Ok(())
    }
}
