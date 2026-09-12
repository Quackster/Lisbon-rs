//! Mirrors `net.h4bbo.lisbon.messages.outgoing.messenger.FRIEND_REQUESTS`.
use crate::game::messenger::messenger_user::MessengerUser;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct FRIEND_REQUESTS {
    requests: Vec<MessengerUser>,
}

impl FRIEND_REQUESTS {
    /// Mirrors the `FRIEND_REQUESTS(List<MessengerUser>)` constructor.
    pub fn new(requests: Vec<MessengerUser>) -> Self {
        Self { requests }
    }
}

impl MessageComposer for FRIEND_REQUESTS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.requests.len() as i32);
        response.write_int(self.requests.len() as i32);

        for messenger_user in &self.requests {
            response.write_int(messenger_user.get_user_id());
            response.write_string(messenger_user.get_username());
            response.write_string(messenger_user.get_user_id());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        314 // "BD"
    }
}
