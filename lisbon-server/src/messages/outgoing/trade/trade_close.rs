//! Mirrors `net.h4bbo.lisbon.messages.outgoing.trade.TRADE_CLOSE`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct TRADE_CLOSE;

impl MessageComposer for TRADE_CLOSE {
    /// Mirrors `compose(NettyResponse)` (an empty acknowledgement).
    fn compose(&self, _response: &mut NettyResponse) {}

    fn get_header(&self) -> i16 {
        110 // "An"
    }
}
