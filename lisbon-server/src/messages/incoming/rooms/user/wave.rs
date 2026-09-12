//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.user.WAVE`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct WAVE;

impl MessageEvent for WAVE {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };

        room_user.wave();
        room_user.reset_room_timer();

        Ok(())
    }
}
