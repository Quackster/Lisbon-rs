//! Mirrors `net.h4bbo.lisbon.messages.incoming.messenger.INVITE_FRIEND`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::outgoing::messenger::instant_message_invitation::INSTANT_MESSAGE_INVITATION;
use crate::messages::outgoing::messenger::invitation_error::INVITATION_ERROR;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::string_util::StringUtil;

#[allow(non_camel_case_types)]
pub struct INVITE_FRIEND;

impl MessageEvent for INVITE_FRIEND {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(_room) = player.get_room_user().and_then(|room_user| room_user.get_room()) else {
            return Ok(());
        };

        let users = reader.read_int();

        let Some(messenger) = player.get_messenger() else {
            return Ok(());
        };
        let friends_map = messenger.get_friends();

        let mut friends: Vec<Arc<Mutex<Player>>> = Vec::new();

        for _ in 0..users {
            let user_id = reader.read_int();

            if !friends_map.contains_key(&user_id) {
                player.send(&INVITATION_ERROR);
                break;
            }

            let Some(friend) = PlayerManager::get_instance().get_player_by_id(user_id) else {
                player.send(&INVITATION_ERROR);
                continue;
            };

            friends.push(friend);
        }

        let message = StringUtil::filter_input(&reader.read_string(), false);

        for friend in &friends {
            friend.lock().send(&INSTANT_MESSAGE_INVITATION::new(
                player.get_details().get_id(),
                &message,
            ));
        }

        Ok(())
    }
}
