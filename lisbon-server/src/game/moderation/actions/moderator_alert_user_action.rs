//! Mirrors `net.h4bbo.lisbon.game.moderation.actions.ModeratorAlertUserAction`.

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
use crate::server::netty::streams::NettyRequest;

pub struct ModeratorAlertUserAction;

impl ModerationAction for ModeratorAlertUserAction {
    /// Mirrors `performAction(Player, Room, String, String, NettyRequest)`.
    fn perform_action(
        &self,
        player: &Player,
        _room: &Room,
        alert_message: &str,
        notes: &str,
        reader: &mut NettyRequest,
    ) {
        if !player.has_fuse(&Fuseright::RoomAlert) {
            return;
        }

        let alert_user = reader.read_string();

        if let Some(target_arc) = PlayerManager::get_instance().get_player_by_name(&alert_user) {
            let target = target_arc.lock();
            target.send(&MODERATOR_ALERT::new(alert_message));
            ModerationDao::add_log(
                ModerationActionType::AlertUser,
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
