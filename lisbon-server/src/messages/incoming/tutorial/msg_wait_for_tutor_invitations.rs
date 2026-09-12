//! Mirrors `net.h4bbo.lisbon.messages.incoming.tutorial.MSG_WAIT_FOR_TUTOR_INVITATIONS`.
use crate::game::guides::guide_manager::GuideManager;
use crate::game::player::player::Player;
use crate::messages::outgoing::tutorial::enable_tutor_service_status::{
    ENABLE_TUTOR_SERVICE_STATUS, TutorEnableStatus,
};
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MSG_WAIT_FOR_TUTOR_INVITATIONS;

impl MessageEvent for MSG_WAIT_FOR_TUTOR_INVITATIONS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        if !player.get_guide_manager().is_guide() {
            return Ok(());
        }

        if player.get_guide_manager().is_waiting_for_invitations() {
            return Ok(());
        }

        if GuideManager::get_instance().is_disabled() {
            player.send(
                &ENABLE_TUTOR_SERVICE_STATUS::new(TutorEnableStatus::ServiceDisabled),
            );
            return Ok(());
        }

        if player
            .get_messenger()
            .map_or(false, |messenger| messenger.is_friends_limit_reached())
        {
            player.send(
                &ENABLE_TUTOR_SERVICE_STATUS::new(TutorEnableStatus::FriendlistFull),
            );
            return Ok(());
        }

        if player.get_guide_manager().get_guiding().len() >= GuideManager::MAX_SIMULTANEOUS_GUIDING
        {
            player.send(
                &ENABLE_TUTOR_SERVICE_STATUS::new(TutorEnableStatus::MaxNewbies),
            );
            return Ok(());
        }

        player.get_guide_manager().set_waiting_for_invitations(true);

        Ok(())
    }
}
