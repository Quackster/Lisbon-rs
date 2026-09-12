//! Mirrors `net.h4bbo.lisbon.game.moderation.actions.ModeratorRoomAlertAction`.

use crate::dao::mysql::moderation_dao::ModerationDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::moderation::moderation_action::ModerationAction;
use crate::game::moderation::moderation_action_type::ModerationActionType;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::messages::outgoing::moderation::moderator_alert::MODERATOR_ALERT;
use crate::server::netty::streams::NettyRequest;

pub struct ModeratorRoomAlertAction;

impl ModerationAction for ModeratorRoomAlertAction {
    /// Mirrors `performAction(Player, Room, String, String, NettyRequest)`.
    fn perform_action(
        &self,
        player: &Player,
        _room: &Room,
        alert_message: &str,
        notes: &str,
        _reader: &mut NettyRequest,
    ) {
        if !player.has_fuse(&Fuseright::RoomAlert) {
            return;
        }

        if let Some(room) = player
            .get_room_user()
            .and_then(|room_user| room_user.get_room())
        {
            for target in room.get_entity_manager().get_players() {
                target.lock().send(&MODERATOR_ALERT::new(alert_message));
            }
        }

        ModerationDao::add_log(
            ModerationActionType::RoomAlert,
            player.get_details().get_id(),
            -1,
            alert_message,
            notes,
        );
    }
}
