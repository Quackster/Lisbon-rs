//! Mirrors `net.h4bbo.lisbon.messages.outgoing.user.settings.ACCOUNT_PREFERENCES`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct ACCOUNT_PREFERENCES {
    sound_enabled: bool,
    has_tutorial: bool,
}

impl ACCOUNT_PREFERENCES {
    /// Mirrors the `ACCOUNT_PREFERENCES(boolean, boolean)` constructor.
    pub fn new(sound_enabled: bool, has_tutorial: bool) -> Self {
        Self {
            sound_enabled,
            has_tutorial,
        }
    }
}

impl MessageComposer for ACCOUNT_PREFERENCES {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_bool(self.sound_enabled);
        response.write_bool(self.has_tutorial);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        308
    }
}
