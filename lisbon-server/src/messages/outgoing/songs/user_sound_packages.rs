//! Mirrors `net.h4bbo.lisbon.messages.outgoing.songs.USER_SOUND_PACKAGES`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct USER_SOUND_PACKAGES {
    hand_soundsets: Vec<i32>,
}

impl USER_SOUND_PACKAGES {
    /// Mirrors the `USER_SOUND_PACKAGES(List<Integer>)` constructor.
    pub fn new(hand_soundsets: Vec<i32>) -> Self {
        Self { hand_soundsets }
    }
}

impl MessageComposer for USER_SOUND_PACKAGES {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.hand_soundsets.len() as i32);

        for hand_soundset in &self.hand_soundsets {
            response.write_int(*hand_soundset);
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        302 // "Dn"
    }
}
