//! Mirrors `net.h4bbo.lisbon.messages.outgoing.messenger.FRIENDLIST`.
use crate::game::messenger::messenger_user::MessengerUser;
use crate::game::player::player::Player;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct FRIENDLIST<'a> {
    player: &'a Player,
    friends: Vec<MessengerUser>,
}

impl<'a> FRIENDLIST<'a> {
    /// Mirrors the `FRIENDLIST(Player, List<MessengerUser>)` constructor.
    pub fn new(player: &'a Player, friends: Vec<MessengerUser>) -> Self {
        Self { player, friends }
    }
}

impl MessageComposer for FRIENDLIST<'_> {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.friends.len() as i32);
        for friend in &self.friends {
            let mut friend = friend.clone();
            friend.serialise(self.player, response);
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        263 // "DG"
    }
}
