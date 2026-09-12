//! Mirrors `net.h4bbo.lisbon.dao.mysql.ModerationDao`.

use crate::dao::storage::Storage;
use crate::game::moderation::moderation_action_type::ModerationActionType;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct ModerationDao;

impl ModerationDao {
    /// Mirrors `addLog(ModerationActionType, int, int, String, String)`.
    pub fn add_log(
        action_type: ModerationActionType,
        user_id: i32,
        target_id: i32,
        message: &str,
        extra_notes: &str,
    ) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO housekeeping_audit_log (action, user_id, target_id, message, extra_notes) VALUES ('{}', {target_id}, {user_id}, '{}', '{}')",
            action_type.name(),
            escape(message),
            escape(extra_notes)
        ));
    }
}
