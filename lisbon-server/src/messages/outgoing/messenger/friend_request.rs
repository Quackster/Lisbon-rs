//! Mirrors `net.h4bbo.lisbon.messages.outgoing.messenger.FRIEND_REQUEST`.
use crate::game::messenger::messenger_user::MessengerUser;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct FRIEND_REQUEST {
    requester: MessengerUser,
}

impl FRIEND_REQUEST {
    /// Mirrors the `FRIEND_REQUEST(MessengerUser)` constructor.
    pub fn new(requester: MessengerUser) -> Self {
        Self { requester }
    }
}

impl MessageComposer for FRIEND_REQUEST {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.requester.get_user_id());
        response.write_string(self.requester.get_username());
        response.write_string(self.requester.get_user_id());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        132
    }
}
