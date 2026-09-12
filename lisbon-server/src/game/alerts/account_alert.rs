//! Mirrors `net.h4bbo.lisbon.game.alerts.AccountAlert`.

use crate::game::alerts::alert_type::AlertType;

#[derive(Clone, Debug, serde::Serialize)]
pub struct AccountAlert {
    id: i32,
    user_id: i32,
    alert_type: AlertType,
    message: String,
    is_disabled: bool,
    created_at: i64,
}

impl AccountAlert {
    /// Mirrors the 6-arg `AccountAlert(int, int, AlertType, String, boolean, long)` constructor.
    pub fn new(
        id: i32,
        user_id: i32,
        alert_type: AlertType,
        message: String,
        is_disabled: bool,
        created_at: i64,
    ) -> Self {
        Self {
            id,
            user_id,
            alert_type,
            message,
            is_disabled,
            created_at,
        }
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `getUserId()`.
    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    /// Mirrors `getAlertType()`.
    pub fn get_alert_type(&self) -> AlertType {
        self.alert_type
    }

    /// Mirrors `getMessage()`.
    pub fn get_message(&self) -> &str {
        &self.message
    }

    /// Mirrors `isDisabled()`.
    pub fn is_disabled(&self) -> bool {
        self.is_disabled
    }

    /// Mirrors `getCreatedAt()`.
    pub fn get_created_at(&self) -> i64 {
        self.created_at
    }
}
