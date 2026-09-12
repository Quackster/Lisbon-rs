//! Mirrors `net.h4bbo.lisbon.messages.outgoing.handshake.CRYPTO_PARAMETERS`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct CRYPTO_PARAMETERS;

impl MessageComposer for CRYPTO_PARAMETERS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(0);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        277
    }
}
