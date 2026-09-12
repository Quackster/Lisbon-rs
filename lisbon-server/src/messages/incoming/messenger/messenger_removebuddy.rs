//! Mirrors `net.h4bbo.lisbon.messages.incoming.messenger.MESSENGER_REMOVEBUDDY`.
use crate::game::messenger::messenger_error::MessengerError;
use crate::game::messenger::messenger_error_type::MessengerErrorType;
use crate::game::messenger::messenger_manager::MessengerManager;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::outgoing::messenger::messenger_error::MESSENGER_ERROR;
use crate::messages::outgoing::messenger::remove_buddy::REMOVE_BUDDY;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MESSENGER_REMOVEBUDDY;

impl MessageEvent for MESSENGER_REMOVEBUDDY {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let size = reader.read_int();

        let Some(messenger) = player.get_messenger() else {
            return Ok(());
        };

        let me_as_friend = messenger.get_messenger_user();

        for _ in 0..size {
            let friend_id = reader.read_int();

            if !messenger.has_friend(friend_id) {
                player.send(&MESSENGER_ERROR::new(MessengerError::new(
                    MessengerErrorType::ConcurrencyError,
                )));
                return Ok(());
            }

            let Some(friend) = messenger.get_friend(friend_id) else {
                continue;
            };

            if !messenger.remove_friend(friend_id) {
                player.send(&MESSENGER_ERROR::new(MessengerError::new(
                    MessengerErrorType::ConcurrencyError,
                )));
                return Ok(());
            }

            let Some(friend_messenger) = MessengerManager::get_instance()
                .get_messenger_data_by_id(friend_id)
            else {
                player.send(&MESSENGER_ERROR::new(MessengerError::new(
                    MessengerErrorType::ConcurrencyError,
                )));
                return Ok(());
            };

            if !friend_messenger.remove_friend(me_as_friend.get_user_id()) {
                player.send(&MESSENGER_ERROR::new(MessengerError::new(
                    MessengerErrorType::ConcurrencyError,
                )));
                return Ok(());
            }

            if let Some(player_friend) = PlayerManager::get_instance().get_player_by_id(friend_id) {
                let player_friend = player_friend.lock();
                player_friend.send(&REMOVE_BUDDY::new(&player_friend, me_as_friend.clone()));
            }

            player.send(&REMOVE_BUDDY::new(player, friend));
        }

        Ok(())
    }
}
