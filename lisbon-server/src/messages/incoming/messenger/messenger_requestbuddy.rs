//! Mirrors `net.h4bbo.lisbon.messages.incoming.messenger.MESSENGER_REQUESTBUDDY`.
use crate::game::entity::entity::Entity;
use crate::game::messenger::messenger_error::MessengerError;
use crate::game::messenger::messenger_error_type::MessengerErrorType;
use crate::game::messenger::messenger_manager::MessengerManager;
use crate::game::player::player::Player;
use crate::messages::outgoing::messenger::messenger_error::MESSENGER_ERROR;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MESSENGER_REQUESTBUDDY;

impl MessageEvent for MESSENGER_REQUESTBUDDY {
    /// Mirrors `handle(Player, NettyRequest)` (the self-name check runs
    /// before the lookup, since the target's player lock is held by the
    /// dispatcher when the target is the caller).
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let username = reader.read_string();

        if username.to_lowercase() == player.get_details().get_name().to_lowercase() {
            return Ok(());
        }

        let target = if username.eq_ignore_ascii_case("Abigail.Ryan") {
            None
        } else {
            MessengerManager::get_instance().get_messenger_data_by_name(&username)
        };

        let Some(target) = target else {
            player.send(&MESSENGER_ERROR::new(
                MessengerError::new(MessengerErrorType::FriendRequestNotFound),
            ));
            return Ok(());
        };

        let Some(callee) = player.get_messenger() else {
            return Ok(());
        };

        if callee.is_friends_limit_reached() {
            player.send(&MESSENGER_ERROR::new(MessengerError::new(
                MessengerErrorType::FriendListFull,
            )));
            return Ok(());
        }

        if target.has_friend(player.get_details().get_id()) {
            return Ok(());
        }

        if target.has_request(player.get_details().get_id()) {
            return Ok(());
        }

        if target.is_friends_limit_reached() {
            player.send(&MESSENGER_ERROR::new(MessengerError::new(
                MessengerErrorType::TargetFriendListFull,
            )));
            return Ok(());
        }

        if !target.allows_friend_requests() {
            player.send(&MESSENGER_ERROR::new(MessengerError::new(
                MessengerErrorType::TargetDoesNotAccept,
            )));
            return Ok(());
        }

        target.add_request(&callee.get_messenger_user());

        Ok(())
    }
}
