//! Mirrors `net.h4bbo.lisbon.messages.outgoing.navigator.FLAT_NORESULTS`.
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct FLAT_NORESULTS<'a> {
    room_list: Vec<Room>,
    player: &'a Player,
}

impl<'a> FLAT_NORESULTS<'a> {
    /// Mirrors the `FLAT_NORESULTS(List<Room>, Player)` constructor.
    pub fn new(room_list: Vec<Room>, player: &'a Player) -> Self {
        Self { room_list, player }
    }
}

impl MessageComposer for FLAT_NORESULTS<'_> {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        for room in &self.room_list {
            response.write_delimeter(room.get_id(), '\t');
            response.write_delimeter(room.get_data().get_name(), '\t');

            if room.is_owner(self.player.get_details().get_id())
                || room.get_data().show_owner_name()
                || self.player.has_fuse(&Fuseright::SeeAllRoomowners)
            {
                response.write_delimeter(room.get_data().get_owner_name(), '\t');
            } else {
                response.write_delimeter("-", '\t');
            }

            response.write_delimeter(room.get_data().get_access_type(), '\t');
            response.write_delimeter("x", '\t');
            response.write_delimeter(room.get_data().get_visitors_now(), '\t');
            response.write_delimeter(room.get_data().get_visitors_max(), '\t');
            response.write_delimeter("null", '\t');
            response.write_delimeter(room.get_data().get_description(), '\t');
            response.write_delimeter(room.get_data().get_description(), '\t');
            response.write('\r');
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        55 // "@w"
    }
}
