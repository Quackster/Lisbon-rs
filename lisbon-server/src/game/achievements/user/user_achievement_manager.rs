//! Mirrors `net.h4bbo.lisbon.game.achievements.user.UserAchievementManager`.

use parking_lot::Mutex;

use crate::dao::mysql::achievement_dao::AchievementDao;
use crate::game::achievements::achievement_info::AchievementInfo;
use crate::game::achievements::achievement_manager::AchievementManager;
use crate::game::achievements::achievement_type::AchievementType;
use crate::game::achievements::user::user_achievement::UserAchievement;
use crate::game::club::club_subscription::ClubSubscription;
use crate::game::entity::entity::Entity;
use crate::game::guides::guide_manager::GuideManager;
use crate::game::player::player::Player;
use crate::game::groups::group_member_rank::GroupMemberRank;
use crate::util::config::game_configuration::GameConfiguration;

pub struct UserAchievementManager {
    player_id: Mutex<i32>,
    user_achievements: Mutex<Vec<UserAchievement>>,
}

impl UserAchievementManager {
    /// Mirrors the `UserAchievementManager` constructor.
    pub fn new() -> Self {
        Self {
            player_id: Mutex::new(-1),
            user_achievements: Mutex::new(Vec::new()),
        }
    }

    /// Mirrors `loadAchievements(int)`.
    pub fn load_achievements(&self, player_id: i32) {
        *self.player_id.lock() = player_id;
        *self.user_achievements.lock() = AchievementDao::get_user_achievements(player_id);
    }

    /// Mirrors `getPlayerId()`.
    pub fn get_player_id(&self) -> i32 {
        *self.player_id.lock()
    }

    /// Mirrors `getUserAchievements()`.
    pub fn get_user_achievements(&self) -> Vec<UserAchievement> {
        self.user_achievements.lock().clone()
    }

    /// Mirrors `locateAchievement(AchievementType)`.
    pub fn locate_achievement(&self, achievement_type: &AchievementType) -> Option<UserAchievement> {
        let player_id = *self.player_id.lock();
        let mut user_achievements = self.user_achievements.lock();

        let mut optional: Vec<UserAchievement> = user_achievements
            .iter()
            .filter(|achievement| {
                achievement
                    .get_achievement_info()
                    .is_some_and(|info| info.get_name() == achievement_type.get_name())
            })
            .cloned()
            .collect();

        optional.sort_by_key(|achievement| {
            achievement
                .get_achievement_info()
                .map(|info| info.get_level())
                .unwrap_or(0)
        });

        let Some(latest_achievement) = optional.pop() else {
            let Some(achievement_info) = AchievementManager::get_instance()
                .locate_achievement(achievement_type, 1)
            else {
                return None;
            };

            let user_achievement = UserAchievement::new(achievement_info.get_id(), player_id, 0);

            AchievementDao::new_user_achievement(player_id, &user_achievement);

            user_achievements.push(user_achievement.clone());

            return Some(user_achievement);
        };

        let Some(progress_required) = latest_achievement
            .get_achievement_info()
            .map(|info| info.get_progress_required())
        else {
            return Some(latest_achievement);
        };

        if latest_achievement.get_progress() >= progress_required {
            let Some(level) = latest_achievement
                .get_achievement_info()
                .map(|info| info.get_level())
            else {
                return Some(latest_achievement);
            };

            let Some(next_achievement) = AchievementManager::get_instance()
                .locate_achievement(achievement_type, level + 1)
            else {
                return None;
            };

            let user_achievement = UserAchievement::new(next_achievement.get_id(), player_id, 0);

            AchievementDao::new_user_achievement(player_id, &user_achievement);

            user_achievements.push(user_achievement.clone());

            return Some(user_achievement);
        }

        Some(latest_achievement)
    }

    /// Mirrors `getPossibleAchievements()`.
    pub fn get_possible_achievements(&self) -> Vec<AchievementInfo> {
        let user_achievements = self.user_achievements.lock();
        let mut possible: Vec<AchievementInfo> = Vec::new();

        for achievement_info in AchievementManager::get_instance().get_achievements().values() {
            let completed = user_achievements.iter().any(|user_achievement| {
                user_achievement
                    .get_achievement_info()
                    .is_some_and(|info| {
                        info.get_name() == achievement_info.get_name()
                            && user_achievement.get_progress() >= achievement_info.get_progress_required()
                    })
            });

            if completed {
                continue;
            }

            let same_name_count = possible
                .iter()
                .filter(|info| info.get_name() == achievement_info.get_name())
                .count();

            if same_name_count >= 5 {
                continue;
            }

            let next_completed = user_achievements.iter().any(|user_achievement| {
                user_achievement
                    .get_achievement_info()
                    .is_some_and(|info| {
                        info.get_name() == achievement_info.get_name()
                            && user_achievement.get_progress() >= achievement_info.get_progress_required()
                    })
            });

            if next_completed {
                let Some(found) = user_achievements
                    .iter()
                    .find(|user_achievement| {
                        user_achievement
                            .get_achievement_info()
                            .is_some_and(|info| info.get_name() == achievement_info.get_name())
                    })
                else {
                    continue;
                };

                let level = found
                    .get_achievement_info()
                    .map(|info| info.get_level())
                    .unwrap_or(0);

                let Some(achievement_type) =
                    AchievementType::get_by_name(achievement_info.get_name())
                else {
                    continue;
                };

                let Some(new_achievement) =
                    AchievementManager::get_instance()
                        .locate_achievement(&achievement_type, level + 1)
                else {
                    continue;
                };

                possible.push(new_achievement);
            } else {
                possible.push(achievement_info.clone());
            }
        }

        possible.sort_by(|a, b| {
            let key_a = format!("{}{}", a.get_name(), a.get_level());
            let key_b = format!("{}{}", b.get_name(), b.get_level());
            key_a.cmp(&key_b)
        });

        possible
    }

    /// Mirrors `processAchievements(Player, boolean)`.
    pub fn process_achievements(&self, player: &Player, is_login: bool) {
        if is_login {
            AchievementManager::get_instance()
                .try_progress(&AchievementType::Tags, player);
            AchievementManager::get_instance()
                .try_progress(&AchievementType::Login, player);
            AchievementManager::get_instance()
                .try_progress(&AchievementType::HappyHour, player);
        }

        AchievementManager::get_instance()
            .try_progress(&AchievementType::Hc, player);
        AchievementManager::get_instance()
            .try_progress(&AchievementType::RegistrationDuration, player);
        AchievementManager::get_instance()
            .try_progress(&AchievementType::Hc, player);
        AchievementManager::get_instance()
            .try_progress(&AchievementType::Guide, player);
        AchievementManager::get_instance()
            .try_progress(&AchievementType::Guide, player);
        AchievementManager::get_instance()
            .try_progress(&AchievementType::EmailVerification, player);

        let guide_manager = GuideManager::get_instance();
        guide_manager.try_progress(player);
        guide_manager.check_guiding_friends(player);

        ClubSubscription::check_badges(player);

        // Habbo Guide admins
        if player.get_guide_manager().is_guide() {
            if !player.get_badge_manager().has_badge("GLK") {
                let guide_group_id = GameConfiguration::get_instance().get_integer("guides.group.id");
                let group_member = player
                    .get_joined_group(guide_group_id)
                    .and_then(|group| group.get_member(player.get_details().get_id()));

                if let Some(group_member) = group_member {
                    let rank = group_member.get_member_rank();

                    if rank == Some(GroupMemberRank::Administrator)
                        || rank == Some(GroupMemberRank::Owner)
                    {
                        player.get_badge_manager().try_add_badge("GLK", None, 0);
                    }
                } else {
                    player
                        .get_guide_manager()
                        .set_guide(guide_manager.is_guide(player));
                }
            }
        }
    }
}
