//! Mirrors `net.h4bbo.lisbon.messages.outgoing.user.IGNORED_LIST`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct IGNORED_LIST {
    ignore_list: Vec<String>,
}

impl IGNORED_LIST {
    /// Mirrors the `IGNORED_LIST(Set<String>)` constructor.
    pub fn new(ignore_list: Vec<String>) -> Self {
        Self { ignore_list }
    }
}

impl MessageComposer for IGNORED_LIST {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.ignore_list.len() as i32);

        for username in &self.ignore_list {
            response.write_string(username.as_str());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        420
    }
}
