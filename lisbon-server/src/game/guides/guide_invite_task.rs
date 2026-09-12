//! Mirrors `net.h4bbo.lisbon.game.guides.GuideInviteTask`.
use crate::game::entity::entity::Entity;
use crate::game::guides::guide_manager::GuideManager;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::outgoing::tutorial::guide_found::GUIDE_FOUND;
use crate::messages::outgoing::tutorial::inviting_completed::{INVITING_COMPLETED, InvitationResult};
use crate::util::date_util::DateUtil;

pub struct GuideInviteTask;

impl GuideInviteTask {
    /// Mirrors the `GuideInviteTask` constructor.
    pub fn new() -> Self {
        Self
    }

    /// Mirrors `run()`.
    pub fn run(&self) {
        for beginner in GuideManager::get_instance().get_avaliable_beginners() {
            beginner
                .lock()
                .get_guide_manager()
                .get_invited()
                .retain(|user_id| PlayerManager::get_instance().get_player_by_id(*user_id).is_some());

            for guide in GuideManager::get_instance().get_guides_available() {
                let guide_player = guide.lock();

                if !guide_player.get_guide_manager().is_waiting_for_invitations() {
                    continue;
                }

                let guide_id = guide_player.get_details().get_id();

                if guide_player
                    .get_guide_manager()
                    .has_invite(beginner.lock().get_details().get_id())
                {
                    continue;
                }

                if beginner.lock().get_guide_manager().has_invited(guide_id) {
                    continue;
                }

                let beginner_id = beginner.lock().get_details().get_id();
                let beginner_name = beginner.lock().get_details().get_name().to_string();

                beginner.lock().send(&GUIDE_FOUND::new(guide_id));

                guide_player
                    .get_guide_manager()
                    .add_invite(&guide_player, beginner_id, &beginner_name);
                beginner.lock().get_guide_manager().add_invited(guide_id);
            }

            if beginner.lock().get_guide_manager().get_invited().is_empty() {
                let started = beginner
                    .lock()
                    .get_guide_manager()
                    .get_started_for_waiting_guides_time();

                if DateUtil::get_current_time_seconds() > started {
                    let player = beginner.lock();
                    player.get_guide_manager().set_waiting_for_guide(false);
                    player.get_guide_manager().set_started_for_waiting_guides_time(0);
                    drop(player);
                    beginner
                        .lock()
                        .send(&INVITING_COMPLETED::new(InvitationResult::Failure));
                }
            }
        }
    }
}
