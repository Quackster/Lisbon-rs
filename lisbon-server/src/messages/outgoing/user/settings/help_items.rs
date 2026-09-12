//! Mirrors `net.h4bbo.lisbon.messages.outgoing.user.settings.HELP_ITEMS`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct HELP_ITEMS {
    tutorial_flags: Vec<i32>,
}

impl HELP_ITEMS {
    /// Mirrors the `HELP_ITEMS(List<Integer>)` constructor.
    pub fn new(tutorial_flags: Vec<i32>) -> Self {
        Self { tutorial_flags }
    }
}

impl MessageComposer for HELP_ITEMS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.tutorial_flags.len() as i32);
        for flag in &self.tutorial_flags {
            response.write_int(*flag);
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        352 // "E`"
    }
}
