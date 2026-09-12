//! Mirrors `net.h4bbo.lisbon.messages.outgoing.games.GAMELOCATION`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Default, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct GAMELOCATION;

impl MessageComposer for GAMELOCATION {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(-1);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        241 // "Cq"
    }
}
