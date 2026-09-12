//! Mirrors `net.h4bbo.lisbon.messages.outgoing.messenger.MESSENGER_SEARCH`.
use crate::game::entity::entity::Entity;
use crate::game::player::player_details::PlayerDetails;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;
use crate::util::date_util::{DateUtil, LONG_DATE};

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct MESSENGER_SEARCH {
    friends: Vec<PlayerDetails>,
    others: Vec<PlayerDetails>,
}

impl MESSENGER_SEARCH {
    /// Mirrors the `MESSENGER_SEARCH(List<PlayerDetails>, List<PlayerDetails>)`
    /// constructor.
    pub fn new(friends: Vec<PlayerDetails>, others: Vec<PlayerDetails>) -> Self {
        Self { friends, others }
    }

    /// Mirrors `serialiseSearch(NettyResponse, PlayerDetails)`.
    fn serialise_search(&self, response: &mut NettyResponse, player_details: &PlayerDetails) {
        response.write_int(player_details.get_id());
        response.write_string(player_details.get_name());
        response.write_string(player_details.get_motto());

        let player = PlayerManager::get_instance().get_player_by_id(player_details.get_id());
        let is_online = PlayerManager::get_instance().is_player_online(player_details.get_id());

        let (in_room, room_name) = match player {
            Some(player) => {
                let player = player.lock();

                match player
                    .get_room_user()
                    .and_then(|room_user| room_user.get_room())
                {
                    Some(room) => (true, room.get_data().get_name().to_string()),
                    None => (false, String::new()),
                }
            }
            None => (false, String::new()),
        };

        response.write_bool(is_online);
        response.write_bool(is_online && in_room);
        response.write_string(if is_online && in_room {
            room_name.as_str()
        } else {
            ""
        });

        response.write_bool(player_details.get_sex().eq_ignore_ascii_case("m"));
        response.write_string(if is_online {
            player_details.get_figure()
        } else {
            ""
        });
        response.write_string(DateUtil::get_date(
            player_details.get_last_online(),
            LONG_DATE,
        ));
    }
}

impl MessageComposer for MESSENGER_SEARCH {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.friends.len() as i32);

        for player_details in &self.friends {
            self.serialise_search(response, player_details);
        }

        response.write_int(self.others.len() as i32);

        for player_details in &self.others {
            self.serialise_search(response, player_details);
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        435 // Fs
    }
}
