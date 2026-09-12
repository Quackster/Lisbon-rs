//! Mirrors `net.h4bbo.lisbon.game.player.statistics.PlayerStatisticManager`.

use std::collections::HashMap;

use parking_lot::Mutex;

use crate::dao::mysql::player_statistics_dao::PlayerStatisticsDao;
use crate::game::player::statistics::player_statistic::PlayerStatistic;

pub struct PlayerStatisticManager {
    user_id: i32,
    values: Mutex<HashMap<PlayerStatistic, String>>,
}

impl PlayerStatisticManager {
    /// Mirrors the `PlayerStatisticManager(int, Map)` constructor (the
    /// `Option<String>` values from `PlayerStatisticsDao::get_statistics`
    /// drop their `None` entries, mirroring JDBC `null` values).
    pub fn new(
        user_id: i32,
        values: HashMap<PlayerStatistic, Option<String>>,
    ) -> Self {
        Self {
            user_id,
            values: Mutex::new(
                values
                    .into_iter()
                    .filter_map(|(statistic, value)| value.map(|value| (statistic, value)))
                    .collect(),
            ),
        }
    }

    /// Mirrors `reload()`.
    pub fn reload(&self) {
        *self.values.lock() = PlayerStatisticsDao::get_statistics(self.user_id)
            .into_iter()
            .filter_map(|(statistic, value)| value.map(|value| (statistic, value)))
            .collect();
    }

    /// Mirrors `getIntValue(PlayerStatistic)`.
    pub fn get_int_value(&self, player_statistic: PlayerStatistic) -> i32 {
        self.values
            .lock()
            .get(&player_statistic)
            .and_then(|value| value.parse().ok())
            .unwrap_or(0)
    }

    /// Mirrors `getLongValue(PlayerStatistic)`.
    pub fn get_long_value(&self, player_statistic: PlayerStatistic) -> i64 {
        self.values
            .lock()
            .get(&player_statistic)
            .and_then(|value| value.parse().ok())
            .unwrap_or(0)
    }

    /// Mirrors `setLongValue(PlayerStatistic, long)`.
    pub fn set_long_value(&self, statistic: PlayerStatistic, value: i64) {
        let mut values = self.values.lock();

        if !values.contains_key(&statistic) {
            return;
        }

        let value = value.to_string();
        values.insert(statistic, value.clone());
        PlayerStatisticsDao::update_statistic(self.user_id, statistic, &value);
    }

    /// Mirrors `setValue(PlayerStatistic, String)`.
    pub fn set_value(&self, statistic: PlayerStatistic, value: &str) {
        let mut values = self.values.lock();

        if !values.contains_key(&statistic) {
            return;
        }

        values.insert(statistic, value.to_string());
        PlayerStatisticsDao::update_statistic(self.user_id, statistic, value);
    }

    /// Mirrors `incrementValue(PlayerStatistic, int)`.
    pub fn increment_value(&self, statistic: PlayerStatistic, value: i32) {
        let mut values = self.values.lock();

        if !values.contains_key(&statistic) {
            return;
        }

        let current = values
            .get(&statistic)
            .and_then(|value| value.parse::<i32>().ok())
            .unwrap_or(0);
        let value = (current + value).to_string();
        values.insert(statistic, value.clone());
        PlayerStatisticsDao::update_statistic(self.user_id, statistic, &value);
    }

    /// Mirrors `setValues(int, Map<PlayerStatistic, String>)`.
    pub fn set_values(
        &self,
        user_id: i32,
        values: &HashMap<PlayerStatistic, String>,
    ) {
        let mut current = self.values.lock();

        for (statistic, value) in values {
            if current.contains_key(statistic) {
                current.insert(*statistic, value.clone());
            }
        }

        PlayerStatisticsDao::update_statistics(user_id, values);
    }

    /// Mirrors `getValue(PlayerStatistic)`.
    pub fn get_value(&self, statistic: PlayerStatistic) -> Option<String> {
        self.values.lock().get(&statistic).cloned()
    }
}
