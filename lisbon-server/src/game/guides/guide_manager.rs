//! Mirrors `net.h4bbo.lisbon.game.guides.GuideManager`.
use std::sync::{Arc, OnceLock};

use parking_lot::Mutex;

use crate::dao::mysql::alerts_dao::AlertsDao;
use crate::game::alerts::alert_type::AlertType;
use crate::dao::mysql::messenger_dao::MessengerDao;
use crate::dao::mysql::player_statistics_dao::PlayerStatisticsDao;
use crate::game::achievements::achievement_manager::AchievementManager;
use crate::game::achievements::achievement_type::AchievementType;
use crate::game::entity::entity::Entity;
use crate::game::guides::guide_invite_task::GuideInviteTask;
use crate::game::messenger::messenger_user::MessengerUser;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::game::player::statistics::player_statistic::PlayerStatistic;
use crate::messages::outgoing::tutorial::inviting_completed::{INVITING_COMPLETED, InvitationResult};
use crate::util::config::game_configuration::GameConfiguration;
use crate::util::date_util::DateUtil;

pub struct GuideManager;

impl GuideManager {
    /// Mirrors `MAX_SIMULTANEOUS_GUIDING`.
    pub const MAX_SIMULTANEOUS_GUIDING: usize = 10;

    /// Mirrors `getInstance()`.
    ///
    /// The Java `GuideInviteTask` is registered on the `GameScheduler`
    /// with a 10 second fixed rate; the Rust `GameScheduler` has no
    /// fixed-rate primitive, so the task loops on its own thread.
    pub fn get_instance() -> &'static GuideManager {
        static INSTANCE: OnceLock<GuideManager> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let task = GuideInviteTask::new();
            std::thread::spawn(move || loop {
                task.run();
                std::thread::sleep(std::time::Duration::from_secs(10));
            });

            GuideManager
        })
    }

    /// Mirrors `getGuidesAvailable()`.
    pub fn get_guides_available(&self) -> Vec<Arc<Mutex<Player>>> {
        let mut guides = Vec::new();

        for player in PlayerManager::get_instance().get_players().iter().cloned() {
            let eligible = {
                let player = player.lock();

                if !player.get_guide_manager().is_guide() {
                    false
                } else if player
                    .get_guide_manager()
                    .get_guiding()
                    .len()
                    >= Self::MAX_SIMULTANEOUS_GUIDING
                {
                    false
                } else {
                    true
                }
            };

            if eligible {
                guides.push(player);
            }
        }

        guides
    }

    /// Mirrors `getAvaliableBeginners()`.
    pub fn get_avaliable_beginners(&self) -> Vec<Arc<Mutex<Player>>> {
        let mut beginners = Vec::new();

        for player in PlayerManager::get_instance().get_players().iter().cloned() {
            let eligible = {
                let player = player.lock();

                if player.get_guide_manager().is_guide()
                    || !player.get_guide_manager().is_guidable()
                    || !player.get_guide_manager().is_waiting_for_guide()
                {
                    false
                } else {
                    match player
                        .get_room_user()
                        .and_then(|room_user| room_user.get_room())
                    {
                        Some(room) => {
                            !room.is_public_room() && room.is_owner(player.get_details().get_id())
                        }
                        None => false,
                    }
                }
            };

            if eligible {
                beginners.push(player);
            }
        }

        beginners
    }

    /// Mirrors `isGuide(Player)`.
    pub fn is_guide(&self, player: &Player) -> bool {
        let guide_group_id = GameConfiguration::get_instance().get_integer("guides.group.id");

        if guide_group_id < 1 {
            return false;
        }

        let Some(guide_group) = player.get_joined_group(guide_group_id) else {
            return false;
        };

        let days_since_joined = (DateUtil::get_current_time_seconds() as i64
            - player.get_details().get_join_date())
            / 86_400;

        if !(days_since_joined >= 30) {
            return false;
        }

        guide_group.is_member(player.get_details().get_id())
    }

    /// Mirrors `tutorEnterRoom(Player, Player)`.
    pub fn tutor_enter_room(&self, guide: &Player, newb: &Player) {
        if newb.get_statistic_manager().get_int_value(PlayerStatistic::GuidedBy) > 0 {
            return;
        }

        newb.send(&INVITING_COMPLETED::new(InvitationResult::Success));

        if let Some(newb_messenger) = newb.get_messenger() {
            newb_messenger.add_friend(newb, &MessengerUser::from_details(guide.get_details()));
        }

        newb.get_guide_manager().set_guidable(false);
        newb.get_statistic_manager()
            .set_long_value(PlayerStatistic::GuidedBy, guide.get_details().get_id() as i64);
        newb.get_statistic_manager()
            .set_long_value(PlayerStatistic::HasTutorial, 0);
        guide.get_guide_manager().refresh_guiding_users();
    }

    /// Mirrors `isDisabled()`.
    pub fn is_disabled(&self) -> bool {
        if !GameConfiguration::get_instance().get_bool("tutorial.enabled") {
            return true;
        }

        let guide_group_id = GameConfiguration::get_instance().get_integer("guides.group.id");

        guide_group_id < 1
    }

    /// Mirrors `tryProgress(Player)`.
    pub fn try_progress(&self, player: &Player) {
        let statistics_manager = player.get_statistic_manager();
        let guide_id = statistics_manager.get_int_value(PlayerStatistic::GuidedBy);

        if guide_id <= 0 {
            return;
        }

        let online_time = statistics_manager.get_long_value(PlayerStatistic::OnlineTime) / 60;
        let time_required = GameConfiguration::get_instance().get_long("guide.completion.minutes");
        let has_met_online_requirement = online_time >= time_required;

        if !has_met_online_requirement {
            if !MessengerDao::friend_exists(player.get_details().get_id(), guide_id) {
                statistics_manager.set_long_value(PlayerStatistic::GuidedBy, 0);
                statistics_manager.set_long_value(PlayerStatistic::HasTutorial, 1);
                statistics_manager.set_long_value(PlayerStatistic::IsGuidable, 1);
            }
        } else {
            statistics_manager.set_long_value(PlayerStatistic::GuidedBy, 0);
            statistics_manager.set_long_value(PlayerStatistic::HasTutorial, 0);
            statistics_manager.set_long_value(PlayerStatistic::IsGuidable, 0);

            PlayerStatisticsDao::increment_statistic(guide_id, PlayerStatistic::PlayersGuided, 1);
            let total_guided = PlayerStatisticsDao::get_statistic_long(guide_id, PlayerStatistic::PlayersGuided);

            AlertsDao::create_alert(
                guide_id,
                AlertType::TutorScore,
                &format!(
                    "You have just completed guiding another new player! You have now guided a total of {} players.",
                    total_guided
                ),
            );
            AchievementManager::get_instance()
                .try_progress(&AchievementType::Student, player);
        }
    }

    /// Mirrors `checkGuidingFriends(Player)`.
    pub fn check_guiding_friends(&self, player: &Player) {
        if player.get_guide_manager().is_guide() {
            for user in player.get_guide_manager().get_guiding().iter() {
                if !player
                    .get_messenger()
                    .map_or(false, |m| m.has_friend(user.get_user_id()))
                {
                    PlayerStatisticsDao::update_statistic(
                        user.get_user_id(),
                        PlayerStatistic::GuidedBy,
                        "0",
                    );
                }
            }
        }
    }

    /// Mirrors `tryClearTutorial(Player)`.
    pub fn try_clear_tutorial(&self, player: &Player) {
        player.get_guide_manager().set_has_tutorial(false);
        player.get_guide_manager().set_can_use_tutorial(false);
        player.get_guide_manager().set_cancel_tutorial(false);
        player
            .get_statistic_manager()
            .set_long_value(PlayerStatistic::HasTutorial, 0);

        if player.get_guide_manager().is_guidable() {
            player.get_guide_manager().set_guidable(false);
        }
    }
}
