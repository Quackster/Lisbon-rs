//! Mirrors `net.h4bbo.lisbon.messages.incoming.messenger.MESSENGER_ACCEPTBUDDY`.
use crate::game::messenger::messenger_error::MessengerError;
use crate::game::messenger::messenger_error_type::MessengerErrorType;
use crate::game::messenger::messenger_manager::MessengerManager;
use crate::game::player::player::Player;
use crate::messages::outgoing::messenger::buddy_request_result::BUDDY_REQUEST_RESULT;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MESSENGER_ACCEPTBUDDY;

impl MessageEvent for MESSENGER_ACCEPTBUDDY {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let mut errors = Vec::new();

        let amount = reader.read_int();

        let Some(messenger) = player.get_messenger() else {
            return Ok(());
        };

        for _ in 0..amount {
            let user_id = reader.read_int();

            let Some(new_buddy) = messenger.get_request(user_id) else {
                errors.push(MessengerError::new(MessengerErrorType::FriendRequestNotFound));
                continue;
            };

            let Some(new_buddy_data) = MessengerManager::get_instance()
                .get_messenger_data_by_id(user_id)
            else {
                continue;
            };

            if messenger.is_friends_limit_reached() {
                let mut error = MessengerError::new(MessengerErrorType::FriendListFull);
                error.set_causer(new_buddy.get_username());
                errors.push(error);
                continue;
            }

            if new_buddy_data.is_friends_limit_reached() {
                let mut error = MessengerError::new(MessengerErrorType::TargetFriendListFull);
                error.set_causer(new_buddy.get_username());
                errors.push(error);
                continue;
            }

            messenger.add_friend(player, &new_buddy);
        }

        player.send(&BUDDY_REQUEST_RESULT::new(errors));

        Ok(())
    }
}
