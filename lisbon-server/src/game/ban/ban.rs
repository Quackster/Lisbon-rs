//! Mirrors `net.h4bbo.lisbon.game.ban.Ban`.

use crate::dao::mysql::ban_dao::BanDao;
use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::ban::ban_type::BanType;
use crate::util::date_util::{DateUtil, LONG_DATE};

#[derive(Clone, Debug, serde::Serialize)]
pub struct Ban {
    ban_type: BanType,
    value: String,
    message: String,
    banned_util: i64,
    banned_at: i64,
    banned_by: i32,
}

impl Ban {
    /// Mirrors the 6-arg `Ban(BanType, String, String, long, long, int)` constructor.
    pub fn new(
        ban_type: BanType,
        value: String,
        message: String,
        banned_util: i64,
        banned_at: i64,
        banned_by: i32,
    ) -> Self {
        Self {
            ban_type,
            value,
            message,
            banned_util,
            banned_at,
            banned_by,
        }
    }

    /// Mirrors `getName()`.
    pub fn get_name(&self) -> String {
        if self.ban_type == BanType::MachineId {
            return BanDao::get_name(&self.value);
        }

        if self.ban_type == BanType::UserId {
            if let Ok(user_id) = self.value.parse::<i32>() {
                return PlayerDao::get_name(user_id).unwrap_or_default();
            }
        }

        String::new()
    }

    /// Mirrors `getBannedBy()`.
    pub fn get_banned_by(&self) -> String {
        if self.banned_by == -1 {
            return "Triggered spam filter".to_string();
        }

        if self.banned_by > 0 {
            return PlayerDao::get_name(self.banned_by).unwrap_or_default();
        }

        "Legacy Banned".to_string()
    }

    /// Mirrors `getBanType()`.
    pub fn get_ban_type(&self) -> BanType {
        self.ban_type
    }

    /// Mirrors `getValue()`.
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Mirrors `getMessage()`.
    pub fn get_message(&self) -> &str {
        &self.message
    }

    /// Mirrors `getBannedUtil()`.
    pub fn get_banned_util(&self) -> String {
        DateUtil::get_date(self.banned_util, LONG_DATE)
    }

    /// Mirrors `getBannedAt()`.
    pub fn get_banned_at(&self) -> String {
        DateUtil::get_date(self.banned_at, LONG_DATE)
    }
}
