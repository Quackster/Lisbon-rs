//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.user.TAG_LIST`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct TAG_LIST {
    user_id: i32,
    tags: Vec<String>,
}

impl TAG_LIST {
    /// Mirrors the `TAG_LIST(int, List<String>)` constructor.
    pub fn new(user_id: i32, tags: Vec<String>) -> Self {
        Self { user_id, tags }
    }
}

impl MessageComposer for TAG_LIST {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.user_id);
        response.write_int(self.tags.len() as i32);

        for tag in &self.tags {
            response.write_string(tag.as_str());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        350
    }
}
