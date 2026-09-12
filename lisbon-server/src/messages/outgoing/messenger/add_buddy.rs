//! Mirrors `net.h4bbo.lisbon.messages.outgoing.messenger.ADD_BUDDY`.
use crate::game::messenger::messenger_user::MessengerUser;
use crate::game::player::player::Player;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct ADD_BUDDY<'a> {
    friend: MessengerUser,
    player: &'a Player,
}

impl<'a> ADD_BUDDY<'a> {
    /// Mirrors the `ADD_BUDDY(Player, MessengerUser)` constructor.
    pub fn new(player: &'a Player, friend: MessengerUser) -> Self {
        Self { friend, player }
    }
}

impl MessageComposer for ADD_BUDDY<'_> {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        if let Some(messenger) = self.player.get_messenger() {
            let categories = messenger.get_categories();
            response.write_int(categories.len() as i32);
            for category in &categories {
                response.write_int(category.get_id());
                response.write_string(category.get_name());
            }
        }
        response.write_int(1);
        response.write_int(1);
        let mut friend = self.friend.clone();
        friend.serialise(self.player, response);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        13
    }
}
