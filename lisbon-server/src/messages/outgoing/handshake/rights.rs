//! Mirrors `net.h4bbo.lisbon.messages.outgoing.handshake.RIGHTS`.
use crate::game::fuserights::fuseright::Fuseright;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
pub struct RIGHTS {
    avaliable_fuserights: Vec<Fuseright>,
}

impl RIGHTS {
    /// Mirrors the `RIGHTS(List<Fuseright>)` constructor.
    pub fn new(avaliable_fuserights: Vec<Fuseright>) -> Self {
        Self {
            avaliable_fuserights,
        }
    }
}

impl MessageComposer for RIGHTS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        for fuseright in &self.avaliable_fuserights {
            response.write_string(fuseright.name());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        2 // "@B"
    }
}
