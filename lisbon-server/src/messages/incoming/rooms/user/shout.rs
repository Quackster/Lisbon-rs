//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.user.SHOUT`.
use crate::game::commands::command_manager::CommandManager;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::rooms::user::chat_message::ChatMessageType;
use crate::messages::outgoing::rooms::user::typing_status::TYPING_STATUS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::string_util::StringUtil;

#[allow(non_camel_case_types)]
pub struct SHOUT;

impl MessageEvent for SHOUT {
    /// Not dispatched; the real handling lives in `handle_mut`, which the
    /// connection dispatcher invokes with exclusive `Player` access.
    fn handle(&self, _player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        Ok(())
    }

    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle_mut(&self, player: &mut Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room) = player.get_room_user().and_then(|ru| ru.get_room()) else {
            return Ok(());
        };

        let message = StringUtil::filter_input(&reader.read_string(), true);

        if player.get_room_user().map(|ru| ru.is_typing()).unwrap_or(false) {
            player.get_room_user().map(|ru| ru.set_typing(false));
            let instance_id = player
                .get_room_user()
                .map(|ru| ru.get_instance_id())
                .unwrap_or(0);
            room.send(&TYPING_STATUS::new(instance_id, false));
        }

        if message.is_empty() {
            return Ok(());
        }

        if CommandManager::get_instance().has_command(player, &message) {
            CommandManager::get_instance().invoke_command(player, &message);
            return Ok(());
        }

        if let Some(room_user) = player.get_room_user() {
            room_user.talk(&message, ChatMessageType::Shout);
        }

        Ok(())
    }
}
