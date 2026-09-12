//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.items.ITEM_DELIVERED`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct ITEM_DELIVERED;

impl MessageComposer for ITEM_DELIVERED {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, _response: &mut NettyResponse) {}

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        67
    }
}
