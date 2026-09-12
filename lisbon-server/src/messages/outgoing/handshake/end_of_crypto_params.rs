//! Mirrors `net.h4bbo.lisbon.messages.outgoing.handshake.END_OF_CRYPTO_PARAMS`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct END_OF_CRYPTO_PARAMS;

impl MessageComposer for END_OF_CRYPTO_PARAMS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, _response: &mut NettyResponse) {}

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        278
    }
}
