//! Mirrors `net.h4bbo.lisbon.messages.incoming.messenger.FINDUSER`.
use crate::dao::mysql::messenger_dao::MessengerDao;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::outgoing::messenger::messenger_search::MESSENGER_SEARCH;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct FINDUSER;

impl MessageEvent for FINDUSER {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let search_query = reader.read_string();

        let user_list = MessengerDao::search(&search_query.to_lowercase());

        let Some(messenger) = player.get_messenger() else {
            return Ok(());
        };
        let friends = messenger.get_friends();

        let mut friends_list = Vec::new();
        let mut others = Vec::new();

        for user_id in user_list {
            let Some(user_details) =
                PlayerManager::get_instance().get_player_data_by_id(user_id)
            else {
                continue;
            };

            if friends.contains_key(&user_id) {
                friends_list.push(user_details);
            } else {
                others.push(user_details);
            }
        }

        friends_list.retain(|user_details| user_details.get_id() != player.get_details().get_id());
        others.retain(|user_details| user_details.get_id() != player.get_details().get_id());
        others.retain(|user_details| user_details.get_name() != "Abigail.Ryan");

        player.send(&MESSENGER_SEARCH::new(friends_list, others));

        Ok(())
    }
}
