//! Mirrors `net.h4bbo.lisbon.dao.mysql.AlertsDao`.

use std::collections::HashMap;

use sqlx::mysql::MySqlRow;

use crate::dao::storage::{RowGetters, Storage};
use crate::game::alerts::account_alert::AccountAlert;
use crate::game::alerts::alert_type::AlertType;
use crate::game::messenger::messenger_user::MessengerUser;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

/// `AlertType` has no `name()` in Rust; the Java enum constant names are stored.
fn alert_type_name(alert_type: AlertType) -> &'static str {
    match alert_type {
        AlertType::HcExpired => "HC_EXPIRED",
        AlertType::Present => "PRESENT",
        AlertType::TutorScore => "TUTOR_SCORE",
        AlertType::CreditDonation => "CREDIT_DONATION",
    }
}

fn alert_type_from_name(name: &str) -> Option<AlertType> {
    match name.to_uppercase().as_str() {
        "HC_EXPIRED" => Some(AlertType::HcExpired),
        "PRESENT" => Some(AlertType::Present),
        "TUTOR_SCORE" => Some(AlertType::TutorScore),
        "CREDIT_DONATION" => Some(AlertType::CreditDonation),
        _ => None,
    }
}

pub struct AlertsDao;

impl AlertsDao {
    /// Mirrors `getAlerts(int)`.
    pub fn get_alerts(user_id: i32) -> Vec<AccountAlert> {
        let mut alerts = Vec::new();

        for row in Storage::get_storage()
            .query_all(&format!(
                "SELECT * FROM cms_alerts WHERE user_id = {user_id} ORDER BY created_at DESC"
            ))
        {
            if let Some(alert) = Self::fill(&row) {
                alerts.push(alert);
            }
        }

        alerts
    }

    /// Mirrors `createAlert(int, AlertType, String)`.
    pub fn create_alert(user_id: i32, alert_type: AlertType, message: &str) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO cms_alerts (user_id, alert_type, message) VALUES ({user_id}, '{}', '{}')",
            alert_type_name(alert_type),
            escape(message)
        ));
    }

    /// Mirrors `deleteAlerts(int, AlertType)`.
    pub fn delete_alerts(user_id: i32, alert_type: AlertType) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM cms_alerts WHERE user_id = {user_id} AND alert_type = '{}'",
            alert_type_name(alert_type)
        ));
    }

    /// Mirrors `disableAlerts(int, AlertType)`.
    pub fn disable_alerts(user_id: i32, alert_type: AlertType) {
        Storage::get_storage().execute(&format!(
            "UPDATE cms_alerts SET is_disabled = 1 WHERE user_id = {user_id} AND alert_type = '{}'",
            alert_type_name(alert_type)
        ));
    }

    /// Mirrors `getOnlineFriends(int)`.
    pub fn get_online_friends(user_id: i32) -> HashMap<i32, MessengerUser> {
        let mut friends = HashMap::new();

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT id,username,figure,motto,last_online,sex,allow_stalking,is_online,category_id,online_status_visible FROM messenger_friends INNER JOIN users ON messenger_friends.from_id = users.id WHERE to_id = {user_id} AND is_online = 1"
            ),
        ) {
            if let (Some(result_user_id), Some(username), Some(figure), Some(console_motto), Some(sex), Some(last_online), Some(allow_stalking), Some(category_id), Some(is_online), Some(online_status_visible)) = (
                row.i32("id"),
                row.str("username"),
                row.str("figure"),
                row.str("motto"),
                row.str("sex"),
                row.i64("last_online"),
                row.bool("allow_stalking"),
                row.i32("category_id"),
                row.bool("is_online"),
                row.bool("online_status_visible"),
            ) {
                friends.insert(
                    result_user_id,
                    MessengerUser::new(
                        result_user_id,
                        &username,
                        &figure,
                        &sex,
                        &console_motto,
                        last_online,
                        allow_stalking,
                        category_id,
                        is_online,
                        online_status_visible,
                    ),
                );
            }
        }

        friends
    }

    /// Mirrors `countRequests(int)`.
    pub fn count_requests(user_id: i32) -> i32 {
        let mut count = 0;

        for row in Storage::get_storage()
            .query_all(
                &format!(
                    "SELECT COUNT(*) AS request_count FROM messenger_requests WHERE to_id = {user_id}"
                ),
            )
        {
            if let Some(value) = row.i32("request_count") {
                count = value;
            }
        }

        count
    }

    /// Mirrors `deleteAlert(int, int)`.
    pub fn delete_alert(user_id: i32, id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM cms_alerts WHERE user_id = {user_id} AND id = {id}"
        ));
    }

    /// Mirrors the private `fill(ResultSet)`.
    fn fill(row: &MySqlRow) -> Option<AccountAlert> {
        let id = row.i32("id")?;
        let user_id = row.i32("user_id")?;
        let alert_type_str = row.str("alert_type")?;
        let alert_type = alert_type_from_name(&alert_type_str)?;
        let message = row.str("message")?;
        let is_disabled = row.bool("is_disabled")?;
        let created_at = row.i64("created_at")?;

        Some(AccountAlert::new(
            id,
            user_id,
            alert_type,
            message,
            is_disabled,
            created_at,
        ))
    }
}
