//! Mirrors `net.h4bbo.lisbon.messages.outgoing.user.currencies.NO_TICKETS`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct NO_TICKETS;

impl MessageComposer for NO_TICKETS {
    fn compose(&self, _response: &mut NettyResponse) {}

    fn get_header(&self) -> i16 {
        // "AI"
        73
    }
}
