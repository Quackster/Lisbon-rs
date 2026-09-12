//! Mirrors `net.h4bbo.lisbon.messages.incoming.messenger.MESSENGER_SENDMSG`.
use crate::dao::mysql::messenger_dao::MessengerDao;
use crate::game::entity::entity::Entity;
use crate::game::messenger::messenger_message::MessengerMessage;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::game::wordfilter::wordfilter_manager::WordfilterManager;
use crate::messages::outgoing::messenger::instant_message_error::INSTANT_MESSAGE_ERROR;
use crate::messages::outgoing::messenger::messenger_msg::MESSENGER_MSG;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::date_util::DateUtil;
use crate::util::string_util::StringUtil;

#[allow(non_camel_case_types)]
pub struct MESSENGER_SENDMSG;

impl MessageEvent for MESSENGER_SENDMSG {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let user_id = reader.read_int();

        let original_message = reader.read_string();
        let message = WordfilterManager::filter_mandatory_sentence(
            &StringUtil::filter_input(&original_message, false),
        );

        if message.trim().is_empty() {
            return Ok(());
        }

        if WordfilterManager::has_bannable_sentence(player, &original_message) {
            WordfilterManager::perform_ban(player);
            return Ok(());
        }

        let Some(messenger) = player.get_messenger() else {
            return Ok(());
        };

        if messenger.get_friends().get(&user_id).is_none() {
            player.send(&INSTANT_MESSAGE_ERROR::new(6, user_id));
            return Ok(());
        }

        let Some(friend_player) = PlayerManager::get_instance().get_player_by_id(user_id) else {
            player.send(&INSTANT_MESSAGE_ERROR::new(5, user_id));
            return Ok(());
        };

        let friend_player = friend_player.lock();

        let chat_message = if friend_player.get_details().is_word_filter_enabled() {
            WordfilterManager::filter_sentence(&message)
        } else {
            message.clone()
        };

        let message_id =
            MessengerDao::new_message(player.get_details().get_id(), user_id, &original_message);

        let msg = MessengerMessage::new(
            message_id,
            user_id,
            player.get_details().get_id(),
            DateUtil::get_current_time_seconds() as i64,
            &chat_message,
        );

        friend_player.send(&MESSENGER_MSG::new(msg));

        Ok(())
    }
}
