//! Mirrors `net.h4bbo.lisbon.messages.outgoing.games.LOUNGEINFO`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct LOUNGEINFO;

impl LOUNGEINFO {
    /// Mirrors the `LOUNGEINFO()` constructor.
    pub fn new() -> Self {
        Self
    }
}

impl MessageComposer for LOUNGEINFO {
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(0); // Rank write (rank name/points writes are
        // commented out in the Java).
    }

    fn get_header(&self) -> i16 {
        231 // "Cg"
    }
}
