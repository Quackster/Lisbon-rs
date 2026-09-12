//! Mirrors `net.h4bbo.lisbon.messages.incoming.moderation.MODERATORACTION`.
use crate::game::entity::entity::Entity;
use crate::game::moderation::moderation_action_type::ModerationActionType;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MODERATORACTION;

impl MessageEvent for MODERATORACTION {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let target_type = reader.read_int();
        let action_type = reader.read_int();

        let alert_message = reader.read_string();
        let notes = reader.read_string();

        let Some(room) = player.get_room_user().and_then(|room_user| room_user.get_room()) else {
            return Ok(());
        };

        for moderation_action_type in [
            ModerationActionType::AlertUser,
            ModerationActionType::KickUser,
            ModerationActionType::BanUser,
            ModerationActionType::RoomAlert,
            ModerationActionType::RoomKick,
        ] {
            if moderation_action_type.get_target_type() == target_type
                && moderation_action_type.get_action_type() == action_type
            {
                moderation_action_type
                    .get_moderation_action()
                    .perform_action(player, &room, &alert_message, &notes, reader);
            }
        }

        Ok(())
    }
}
