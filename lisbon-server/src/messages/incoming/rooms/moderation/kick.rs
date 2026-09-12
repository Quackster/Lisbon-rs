//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.moderation.KICK`.
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::game::texts::texts_manager::TextsManager;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct KICK;

impl MessageEvent for KICK {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let player_name = reader.contents().unwrap_or_default();

        let Some(target_arc) = PlayerManager::get_instance().get_player_by_name(&player_name)
        else {
            return Ok(());
        };
        let target = target_arc.lock();

        if target
            .get_room_user()
            .and_then(|room_user| room_user.get_room())
            .is_none()
        {
            return Ok(());
        }

        let Some(room) = player
            .get_room_user()
            .and_then(|room_user| room_user.get_room())
        else {
            return Ok(());
        };

        if target.get_details().get_id() == player.get_details().get_id() {
            return Ok(()); // Can't kick yourself!
        }

        // Don't allow kicking room owners if you aren't a moderator
        if room.is_owner(target.get_details().get_id()) && !player.has_fuse(&Fuseright::Kick) {
            return Ok(());
        }

        // Don't allow kicking if they have permissions to kick too
        if target.has_fuse(&Fuseright::Kick) {
            player.send(&ALERT::new(
                &TextsManager::get_instance().get_value("modtool_rankerror"),
            ));
            return Ok(());
        }

        // Don't allow kicking if you don't have room rights and don't have fuse rights
        if !room.has_rights(player.get_details().get_id()) && !player.has_fuse(&Fuseright::Kick) {
            player.send(&ALERT::new(
                &TextsManager::get_instance().get_value("modtool_rankerror"),
            ));
            return Ok(());
        }

        target.get_room_user().map(|room_user| room_user.kick(false));

        Ok(())
    }
}
