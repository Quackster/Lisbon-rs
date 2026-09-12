//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.pool.SIGN`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::room::enums::status_type::StatusType;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct SIGN;

impl MessageEvent for SIGN {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        if room_user.get_room().is_none() {
            return Ok(());
        }

        let contents = reader.contents().unwrap_or_default();

        if contents.is_empty() || !contents.chars().all(|c| c.is_ascii_digit()) {
            return Ok(());
        }

        let vote: i32 = contents.parse().unwrap_or(0);

        if vote < 0 {
            return Ok(());
        }

        if vote <= 7 {
            room_user.set_lido_vote(vote + 3);
        }

        room_user.set_status_timed(StatusType::Sign, &contents, 5, None, -1, -1);
        room_user.set_needs_update(true);
        room_user.reset_room_timer();

        Ok(())
    }
}
