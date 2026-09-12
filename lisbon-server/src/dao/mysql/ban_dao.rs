//! Mirrors `net.h4bbo.lisbon.dao.mysql.BanDao`.

use crate::dao::storage::{RowGetters, Storage};
use crate::game::ban::ban::Ban;
use crate::game::ban::ban_type::BanType;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

fn ban_type_name(ban_type: BanType) -> &'static str {
    match ban_type {
        BanType::UserId => "USER_ID",
        BanType::MachineId => "MACHINE_ID",
        BanType::IpAddress => "IP_ADDRESS",
    }
}

fn ban_type_from_name(name: &str) -> Option<BanType> {
    match name.to_uppercase().as_str() {
        "USER_ID" => Some(BanType::UserId),
        "MACHINE_ID" => Some(BanType::MachineId),
        "IP_ADDRESS" => Some(BanType::IpAddress),
        _ => None,
    }
}

pub struct BanDao;

impl BanDao {
    /// Mirrors `hasBan(BanType, String)`.
    pub fn has_ban(ban_type: BanType, value: &str) -> Option<(String, i64)> {
        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT * FROM users_bans WHERE banned_value = '{}' AND ban_type = '{}' AND banned_until > CURRENT_TIMESTAMP() AND is_active = 1 ORDER BY banned_until DESC LIMIT 1",
                escape(value),
                ban_type_name(ban_type)
            ),
        ) {
            if let (Some(message), Some(banned_until)) = (row.str("message"), row.i64("banned_until")) {
                return Some((message, banned_until));
            }
        }

        None
    }

    /// Mirrors `getName(String)`.
    pub fn get_name(machine_id: &str) -> String {
        Storage::get_storage()
            .get_string(
                &format!("SELECT username FROM users WHERE machine_id = '{}'", escape(machine_id)),
                "username",
            )
            .unwrap_or_default()
    }

    /// Mirrors `addBan(BanType, String, long, String, int)`.
    pub fn add_ban(ban_type: BanType, value: &str, banned_until: i64, message: &str, banned_by: i32) {
        if value.trim().is_empty() {
            return;
        }

        Storage::get_storage().execute(&format!(
            "INSERT INTO users_bans (banned_value, ban_type, banned_until, message, banned_by) VALUES ('{}', '{}', FROM_UNIXTIME({banned_until}), '{}', {banned_by})",
            escape(value),
            ban_type_name(ban_type),
            escape(message)
        ));
    }

    /// Mirrors `removeBan(BanType, String)`.
    pub fn remove_ban(ban_type: BanType, value: &str) {
        Storage::get_storage().execute(&format!(
            "UPDATE users_bans SET is_active = 0 WHERE banned_value = '{}' AND ban_type = '{}'",
            escape(value),
            ban_type_name(ban_type)
        ));
    }

    /// Mirrors `getActiveBans(int, String)`.
    pub fn get_active_bans(page: i32, sort_by: &str) -> Vec<Ban> {
        let mut ban_list = Vec::new();

        let rows = 25;
        let next_offset = page * rows;

        if next_offset >= 0 {
            for row in Storage::get_storage().query_all(
                &format!(
                    "SELECT * FROM users_bans WHERE is_active = 1 ORDER BY {sort_by} DESC LIMIT {rows} OFFSET {next_offset}"
                ),
            ) {
                if let (Some(ban_type), Some(banned_value), Some(message), Some(banned_until), Some(banned_at), Some(banned_by)) = (
                    row.str("ban_type"),
                    row.str("banned_value"),
                    row.str("message"),
                    row.i64("banned_until"),
                    row.i64("banned_at"),
                    row.i32("banned_by"),
                ) {
                    if let Some(ban_type) = ban_type_from_name(&ban_type) {
                        ban_list.push(Ban::new(
                            ban_type,
                            banned_value,
                            message,
                            banned_until,
                            banned_at,
                            banned_by,
                        ));
                    }
                }
            }
        }

        ban_list
    }
}
