//! Mirrors `net.h4bbo.lisbon.messages.incoming.user.settings.GET_SOUND_SETTING`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::user::settings::sound_setting::SOUND_SETTING;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GET_SOUND_SETTING;

impl MessageEvent for GET_SOUND_SETTING {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        player.send(&SOUND_SETTING::new(player.get_details().clone()));

        Ok(())
    }
}
