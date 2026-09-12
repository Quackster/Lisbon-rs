//! Mirrors `net.h4bbo.lisbon.messages.outgoing.messenger.MESSENGER_INIT`.
use crate::game::messenger::messenger::Messenger;
use crate::game::messenger::messenger_user::MessengerUser;
use crate::game::player::player::Player;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;
use crate::util::config::game_configuration::GameConfiguration;

#[allow(non_camel_case_types)]
pub struct MESSENGER_INIT<'a> {
    player: &'a Player,
    friends_limit: i32,
    friends: Vec<MessengerUser>,
}

impl<'a> MESSENGER_INIT<'a> {
    /// Mirrors the `MESSENGER_INIT(Player, Messenger)` constructor.
    pub fn new(player: &'a Player, data: &Messenger) -> Self {
        let friends_limit = data.get_friends_limit();
        let friends = data.get_friends().values().cloned().collect();
        Self {
            player,
            friends_limit,
            friends,
        }
    }
}

impl MessageComposer for MESSENGER_INIT<'_> {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        let config = GameConfiguration::get_instance();
        let normal_friends_limit = config.get_integer("messenger.max.friends.nonclub");
        let club_friends_limit = config.get_integer("messenger.max.friends.club");
        response.write_int(self.friends_limit);
        response.write_int(normal_friends_limit);
        response.write_int(club_friends_limit);
        if let Some(messenger) = self.player.get_messenger() {
            let categories = messenger.get_categories();
            response.write_int(categories.len() as i32);
            for category in &categories {
                response.write_int(category.get_id());
                response.write_string(category.get_name());
            }
        }
        response.write_int(self.friends.len() as i32);
        for friend in &self.friends {
            let mut friend = friend.clone();
            friend.serialise(self.player, response);
        }
        response.write_int(0);
        response.write_int(0);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        12 // "@L"
    }
}
