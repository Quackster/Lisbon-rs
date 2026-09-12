//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.user.WHISPER`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::outgoing::rooms::user::chat_message::ChatMessageType;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct WHISPER;

impl MessageEvent for WHISPER {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        if player.get_room_user().and_then(|ru| ru.get_room()).is_none() {
            return Ok(());
        }

        let contents = reader.read_string();

        let username = contents.split(' ').next().unwrap_or_default();
        let message = contents.splitn(2, ' ').nth(1).unwrap_or_default();

        if message.trim().is_empty() {
            return Ok(());
        }

        let mut receive_messages: Vec<Arc<Mutex<Player>>> = Vec::new();

        if let Some(self_handle) = PlayerManager::get_instance()
            .get_player_by_id(player.get_details().get_id())
        {
            receive_messages.push(self_handle);
        }

        if let Some(whisper_user) = PlayerManager::get_instance().get_player_by_name(username) {
            let ignored = {
                let guard = whisper_user.lock();
                guard.get_ignored_list().contains(player.get_details().get_name())
            };

            if !ignored {
                receive_messages.push(whisper_user);
            }
        }

        if let Some(room_user) = player.get_room_user() {
            room_user.talk_to(message, ChatMessageType::Whisper, &receive_messages);
        }

        Ok(())
    }
}
