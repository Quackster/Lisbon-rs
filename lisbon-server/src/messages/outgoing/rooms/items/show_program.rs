//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.items.SHOWPROGRAM`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct SHOWPROGRAM {
    pub data: Vec<String>,
}

impl SHOWPROGRAM {
    pub fn new(data: Vec<String>) -> Self {
        Self { data }
    }
}

impl MessageComposer for SHOWPROGRAM {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write(self.data.join(" "));
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        71 // "AG"
    }
}
