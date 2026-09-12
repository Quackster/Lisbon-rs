//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.user.RATEFLAT`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct RATEFLAT;

impl MessageEvent for RATEFLAT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(mut room) = room_user.get_room() else {
            return Ok(());
        };

        if room.is_public_room() {
            return Ok(());
        }

        // Room owner is not allowed to vote on his own room
        if room.get_data().get_owner_id() == player.get_details().get_id() {
            return Ok(());
        }

        let answer = reader.read_int();

        // It's either negative or positive
        if answer != 1 && answer != -1 {
            return Ok(());
        }

        let user_id = player.get_details().get_id();

        if room.has_voted(user_id) {
            return Ok(());
        }

        room.add_vote(answer, user_id);

        Ok(())
    }
}
