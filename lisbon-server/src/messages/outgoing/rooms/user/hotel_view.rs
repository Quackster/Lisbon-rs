//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.user.HOTEL_VIEW`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct HOTEL_VIEW;

impl MessageComposer for HOTEL_VIEW {
    fn compose(&self, _response: &mut NettyResponse) {}

    fn get_header(&self) -> i16 {
        // "@R"
        18
    }
}
