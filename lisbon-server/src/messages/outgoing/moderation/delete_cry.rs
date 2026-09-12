//! Mirrors `net.h4bbo.lisbon.messages.outgoing.moderation.DELETE_CRY`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct DELETE_CRY {
    cry_id: i32,
}

impl DELETE_CRY {
    /// Mirrors the `DELETE_CRY(int)` constructor.
    pub fn new(cry_id: i32) -> Self {
        Self { cry_id }
    }
}

impl MessageComposer for DELETE_CRY {
    fn compose(&self, response: &mut NettyResponse) {
        response.write_string(self.cry_id);
    }

    fn get_header(&self) -> i16 {
        // "DQ"
        273
    }
}
