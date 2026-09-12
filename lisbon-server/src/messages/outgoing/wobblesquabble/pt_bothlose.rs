//! Mirrors `net.h4bbo.lisbon.messages.outgoing.wobblesquabble.PT_BOTHLOSE`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct PT_BOTHLOSE;

impl MessageComposer for PT_BOTHLOSE {
    fn compose(&self, _response: &mut NettyResponse) {}

    fn get_header(&self) -> i16 {
        120
    }
}
