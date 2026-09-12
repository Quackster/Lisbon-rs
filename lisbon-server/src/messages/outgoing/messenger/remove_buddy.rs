//! Mirrors `net.h4bbo.lisbon.messages.outgoing.messenger.REMOVE_BUDDY`.
use crate::game::messenger::messenger_user::MessengerUser;
use crate::game::player::player::Player;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct REMOVE_BUDDY<'a> {
    friend: MessengerUser,
    player: &'a Player,
}

impl<'a> REMOVE_BUDDY<'a> {
    /// Mirrors the `REMOVE_BUDDY(Player, MessengerUser)` constructor.
    pub fn new(player: &'a Player, friend: MessengerUser) -> Self {
        Self { friend, player }
    }
}

impl MessageComposer for REMOVE_BUDDY<'_> {
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
        response.write_int(-1);
        response.write_int(self.friend.get_user_id());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        13
    }
}
