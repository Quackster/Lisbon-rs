//! Mirrors `net.h4bbo.lisbon.messages.outgoing.messenger.FRIENDS_UPDATE`.
use crate::game::messenger::messenger::Messenger;
use crate::game::messenger::messenger_user::MessengerUser;
use crate::game::player::player::Player;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct FRIENDS_UPDATE<'a> {
    messenger: &'a Messenger,
    player: &'a Player,
    friends_updated: Vec<MessengerUser>,
}

impl<'a> FRIENDS_UPDATE<'a> {
    /// Mirrors the `FRIENDS_UPDATE(Player, Messenger)` constructor (the
    /// `friendsUpdated` list is drained from the messenger).
    pub fn new(player: &'a Player, messenger: &'a Messenger) -> Self {
        let friends_updated = messenger.get_friends_update();
        Self {
            messenger,
            player,
            friends_updated,
        }
    }
}

impl MessageComposer for FRIENDS_UPDATE<'_> {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        let categories = self.messenger.get_categories();
        response.write_int(categories.len() as i32);
        for category in &categories {
            response.write_int(category.get_id());
            response.write_string(category.get_name());
        }
        response.write_int(self.friends_updated.len() as i32);
        for friend in &self.friends_updated {
            response.write_int(0);
            let mut friend = friend.clone();
            friend.serialise(self.player, response);
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        13 // "@M"
    }
}
