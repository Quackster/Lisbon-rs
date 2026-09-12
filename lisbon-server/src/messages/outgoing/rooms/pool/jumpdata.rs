//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.pool.JUMPDATA`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct JUMPDATA {
    instance_id: i32,
    diving_handle: String,
}

impl JUMPDATA {
    /// Mirrors the `JUMPDATA(int, String)` constructor.
    pub fn new(instance_id: i32, diving_handle: &str) -> Self {
        Self {
            instance_id,
            diving_handle: diving_handle.to_string(),
        }
    }
}

impl MessageComposer for JUMPDATA {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_delimeter(self.instance_id, '\u{000D}');
        response.write(self.diving_handle.as_str());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        74 // "AJ"
    }
}
