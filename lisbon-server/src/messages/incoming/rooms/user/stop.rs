//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.user.STOP`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::room::enums::status_type::StatusType;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct STOP;

impl MessageEvent for STOP {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let stop_what = reader.contents().unwrap_or_default();

        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };

        if stop_what == "Dance" {
            room_user.remove_status(StatusType::Dance);
            room_user.set_needs_update(true);
        }

        if stop_what == "CarryItem" {
            room_user.remove_status(StatusType::CarryItem);
            room_user.set_needs_update(true);
        }

        room_user.reset_room_timer();

        Ok(())
    }
}
