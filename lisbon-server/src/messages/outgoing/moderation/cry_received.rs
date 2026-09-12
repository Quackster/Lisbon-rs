//! Mirrors `net.h4bbo.lisbon.messages.outgoing.moderation.CRY_RECEIVED`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct CRY_RECEIVED;

impl MessageComposer for CRY_RECEIVED {
    fn compose(&self, response: &mut NettyResponse) {
        response.write_string("H");
    }

    fn get_header(&self) -> i16 {
        321
    }
}
