//! Mirrors `net.h4bbo.lisbon.messages.outgoing.navigator.FLAT_RESULTS`.
use crate::game::room::room::Room;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct FLAT_RESULTS {
    room_list: Vec<Room>,
}

impl FLAT_RESULTS {
    /// Mirrors the `FLAT_RESULTS(List<Room>)` constructor.
    pub fn new(room_list: Vec<Room>) -> Self {
        Self { room_list }
    }
}

impl MessageComposer for FLAT_RESULTS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        for room in &self.room_list {
            response.write_delimeter(room.get_id(), '\t');
            response.write_delimeter(room.get_data().get_name(), '\t');
            response.write_delimeter(room.get_data().get_owner_name(), '\t');
            response.write_delimeter(room.get_data().get_access_type(), '\t');
            response.write_delimeter("x", '\t');
            response.write_delimeter(room.get_data().get_visitors_now(), '\t');
            response.write_delimeter(room.get_data().get_visitors_max(), '\t');
            response.write_delimeter("null", '\t');
            response.write_delimeter(room.get_data().get_description(), '\t');
            response.write('\r');
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        16 // "@P"
    }
}
