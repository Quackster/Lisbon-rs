//! Mirrors `net.h4bbo.lisbon.messages.outgoing.navigator.RECOMMENDED_ROOM_LIST`.
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct RECOMMENDED_ROOM_LIST<'a> {
    player: &'a Player,
    room_list: Vec<Room>,
}

impl<'a> RECOMMENDED_ROOM_LIST<'a> {
    /// Mirrors the `RECOMMENDED_ROOM_LIST(Player, List<Room>)` constructor.
    pub fn new(player: &'a Player, room_list: Vec<Room>) -> Self {
        Self { player, room_list }
    }
}

impl MessageComposer for RECOMMENDED_ROOM_LIST<'_> {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.room_list.len() as i32);

        for room in &self.room_list {
            response.write_int(room.get_id());
            response.write_string(room.get_data().get_name());

            if room.is_owner(self.player.get_details().get_id())
                || room.get_data().show_owner_name()
                || self.player.has_fuse(&Fuseright::SeeAllRoomowners)
            {
                response.write_string(room.get_data().get_owner_name());
            } else {
                response.write_string("-");
            }

            response.write_string(room.get_data().get_access_type());
            response.write_int(room.get_data().get_visitors_now());
            response.write_int(room.get_data().get_visitors_max());
            response.write_string(room.get_data().get_description());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        351 // "E_"
    }
}
