//! Mirrors `net.h4bbo.lisbon.game.moderation.actions.ModeratorKickUserAction`.

use crate::game::texts::texts_manager::TextsManager;
use crate::dao::mysql::moderation_dao::ModerationDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::moderation::moderation_action::ModerationAction;
use crate::game::moderation::moderation_action_type::ModerationActionType;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::game::room::room::Room;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::messages::outgoing::moderation::moderator_alert::MODERATOR_ALERT;
use crate::messages::outgoing::rooms::user::hotel_view::HOTEL_VIEW;
use crate::server::netty::streams::NettyRequest;

pub struct ModeratorKickUserAction;

impl ModerationAction for ModeratorKickUserAction {
    /// Mirrors `performAction(Player, Room, String, String, NettyRequest)`.
    fn perform_action(
        &self,
        player: &Player,
        _room: &Room,
        alert_message: &str,
        notes: &str,
        reader: &mut NettyRequest,
    ) {
        if !player.has_fuse(&Fuseright::Kick) {
            return;
        }

        let alert_user = reader.read_string();

        if let Some(target_arc) = PlayerManager::get_instance().get_player_by_name(&alert_user) {
            let target = target_arc.lock();

            if target.get_details().get_id() == player.get_details().get_id() {
                return;
            }

            if target.has_fuse(&Fuseright::Kick) {
                player.send(&ALERT::new(&TextsManager::get_instance().get_value("modtool_rankerror")));
                return;
            }

            if let Some(room_user) = target.get_room_user() {
                // Port note: Java NPEs when `getRoomUser()` is null.
                room_user.kick(false);
            }

            target.send(&HOTEL_VIEW);
            target.send(&MODERATOR_ALERT::new(alert_message));

            ModerationDao::add_log(
                ModerationActionType::KickUser,
                player.get_details().get_id(),
                target.get_details().get_id(),
                alert_message,
                notes,
            );
        } else {
            player.send(&ALERT::new("Target user is not online."));
        }
    }
}
