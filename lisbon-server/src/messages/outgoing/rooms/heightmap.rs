//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.HEIGHTMAP`.
use crate::game::room::models::room_model::RoomModel;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct HEIGHTMAP {
    heightmap: String,
}

impl HEIGHTMAP {
    /// Mirrors the `HEIGHTMAP(String)` constructor.
    pub fn new(heightmap: &str) -> Self {
        Self {
            heightmap: heightmap.to_string(),
        }
    }

    /// Mirrors the `HEIGHTMAP(RoomModel)` constructor.
    pub fn from_room_model(room_model: &RoomModel) -> Self {
        Self {
            heightmap: room_model.get_heightmap().to_string(),
        }
    }
}

impl MessageComposer for HEIGHTMAP {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write(self.heightmap.as_str());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        31 // "@_"
    }
}
