//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.user.WALK`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct WALK;

impl MessageEvent for WALK {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };

        if !room_user.is_walking_allowed() {
            return Ok(());
        }

        let x = reader.read_base64();
        let y = reader.read_base64();

        room_user.walk_to(x, y);

        Ok(())
    }
}
