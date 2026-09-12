//! Mirrors `net.h4bbo.lisbon.dao.mysql.PlayerStatisticsDao`.

use std::collections::HashMap;

use crate::dao::storage::{RowGetters, Storage};
use crate::game::player::statistics::player_statistic::PlayerStatistic;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct PlayerStatisticsDao;

impl PlayerStatisticsDao {
    /// Mirrors `updateStatistic(int, PlayerStatistic, String)`.
    pub fn update_statistic(user_id: i32, statistic: PlayerStatistic, value: &str) {
        let set_clause = if statistic.is_date_time() {
            // Java `setTimestamp(new Timestamp(value * 1000L))`.
            format!("FROM_UNIXTIME({value})")
        } else {
            format!("'{}'", escape(value))
        };

        Storage::get_storage().execute(&format!(
            "UPDATE users_statistics SET {} = {} WHERE user_id = {user_id}",
            statistic.column(),
            set_clause
        ));
    }

    /// Mirrors `incrementStatistic(int, PlayerStatistic, long)`.
    pub fn increment_statistic(user_id: i32, statistic: PlayerStatistic, value: i64) {
        Storage::get_storage().execute(&format!(
            "UPDATE users_statistics SET {} = {} + {value} WHERE user_id = {user_id}",
            statistic.column(),
            statistic.column()
        ));
    }

    /// Mirrors `updateStatistics(int, Map)`.
    pub fn update_statistics(user_id: i32, statistics: &HashMap<PlayerStatistic, String>) {
        for (statistic, value) in statistics {
            Self::update_statistic(user_id, *statistic, value);
        }
    }

    /// Mirrors `incrementStatistics(int, Map)`.
    pub fn increment_statistics(user_id: i32, statistics: &HashMap<PlayerStatistic, i64>) {
        for (statistic, value) in statistics {
            Self::increment_statistic(user_id, *statistic, *value);
        }
    }

    /// Mirrors `getStatisticLong(int, PlayerStatistic)`.
    pub fn get_statistic_long(user_id: i32, player_statistic: PlayerStatistic) -> i64 {
        let mut setting = 0;

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT {} FROM users_statistics WHERE user_id = {user_id}",
                player_statistic.column()
            ),
        ) {
            if let Some(value) = row.i32(player_statistic.column()) {
                setting = value as i64;
            }
        }

        setting
    }

    /// Mirrors `getStatisticString(int, PlayerStatistic)`.
    pub fn get_statistic_string(user_id: i32, player_statistic: PlayerStatistic) -> Option<String> {
        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT {} FROM users_statistics WHERE user_id = {user_id}",
                player_statistic.column()
            ),
        ) {
            if let Some(value) = row.str(player_statistic.column()) {
                return Some(value);
            }
        }

        None
    }

    /// Mirrors `getStatistics(int)`.
    pub fn get_statistics(user_id: i32) -> HashMap<PlayerStatistic, Option<String>> {
        let mut values: HashMap<PlayerStatistic, Option<String>> = HashMap::new();

        for row in Storage::get_storage().query_all(
            &format!("SELECT * FROM users_statistics WHERE user_id = {user_id}"),
        ) {
            for player_statistic in [
                PlayerStatistic::DaysLoggedInRow,
                PlayerStatistic::GuestbookUnreadMessages,
                PlayerStatistic::OnlineTime,
                PlayerStatistic::BattleballPointsAllTime,
                PlayerStatistic::SnowstormPointsAllTime,
                PlayerStatistic::WobbleSquabblePointsAllTime,
                PlayerStatistic::BattleballMonthlyScores,
                PlayerStatistic::SnowstormMonthlyScores,
                PlayerStatistic::WobbleSquabbleMonthlyScores,
                PlayerStatistic::XpEarnedMonth,
                PlayerStatistic::XpAllTime,
                PlayerStatistic::BattleballGamesWon,
                PlayerStatistic::SnowstormGamesWon,
                PlayerStatistic::WobbleSquabbleGamesWon,
                PlayerStatistic::GuidedBy,
                PlayerStatistic::HasTutorial,
                PlayerStatistic::IsGuidable,
                PlayerStatistic::PlayersGuided,
                PlayerStatistic::NewbieRoomLayout,
                PlayerStatistic::NewbieGift,
                PlayerStatistic::NewbieGiftTime,
                PlayerStatistic::GiftsDue,
                PlayerStatistic::ClubGiftDue,
                PlayerStatistic::ClubMemberTime,
                PlayerStatistic::ClubMemberTimeUpdated,
                PlayerStatistic::ActivationCode,
                PlayerStatistic::ForgotPasswordCode,
                PlayerStatistic::ForgotRecoveryRequestedTime,
                PlayerStatistic::MuteExpiresAt,
            ] {
                if player_statistic.is_date_time() {
                    values.insert(
                        player_statistic,
                        row.i64(player_statistic.column()).map(|v| v.to_string()).or_else(|| {
                            row.str(player_statistic.column())
                        }),
                    );
                } else {
                    values.insert(player_statistic, row.str(player_statistic.column()));
                }
            }
        }

        values
    }

    /// Mirrors `newStatistics(int, String)`.
    pub fn new_statistics(user_id: i32, activation_code: &str) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO users_statistics (user_id, activation_code) VALUES ({user_id}, '{}')",
            escape(activation_code)
        ));
    }
}
