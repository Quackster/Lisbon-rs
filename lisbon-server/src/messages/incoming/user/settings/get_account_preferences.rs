//! Mirrors `net.h4bbo.lisbon.messages.incoming.user.settings.GET_ACCOUNT_PREFERENCES`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::user::settings::account_preferences::ACCOUNT_PREFERENCES;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GET_ACCOUNT_PREFERENCES;

impl MessageEvent for GET_ACCOUNT_PREFERENCES {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        player.send(
            &ACCOUNT_PREFERENCES::new(
                player.get_details().get_sound_setting(),
                player.get_guide_manager().has_tutorial(),
            ),
        );

        Ok(())
    }
}
