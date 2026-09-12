//! Mirrors `net.h4bbo.lisbon.messages.incoming.messenger.FOLLOW_FRIEND`.
use crate::game::entity::entity::Entity;
use crate::game::messenger::messenger_user::MessengerUser;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::outgoing::messenger::follow_error::FOLLOW_ERROR;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct FOLLOW_FRIEND;

impl MessageEvent for FOLLOW_FRIEND {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let friend_id = reader.read_int();

        let Some(messenger) = player.get_messenger() else {
            return Ok(());
        };

        if !messenger.has_friend(friend_id) {
            player.send(&FOLLOW_ERROR::new(0));
            return Ok(());
        }

        let Some(friend) = PlayerManager::get_instance().get_player_by_id(friend_id) else {
            player.send(&FOLLOW_ERROR::new(1));
            return Ok(());
        };

        let friend = friend.lock();
        let friend_room = friend.get_room_user().and_then(|room_user| room_user.get_room());
        let messenger_user = MessengerUser::from_details(friend.get_details());

        if friend_room.is_none() || !messenger_user.can_follow_friend(player) {
            player.send(&FOLLOW_ERROR::new(2));
            return Ok(());
        }

        if !friend.get_details().does_allow_stalking() {
            player.send(&FOLLOW_ERROR::new(3));
            return Ok(());
        }

        let Some(friend_room) = friend_room else {
            return Ok(());
        };

        friend_room.forward(player, false);

        Ok(())
    }
}
