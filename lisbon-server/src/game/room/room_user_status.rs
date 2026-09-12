//! Mirrors `net.h4bbo.lisbon.game.room.RoomUserStatus`.

use crate::game::room::enums::status_type::StatusType;

#[derive(Clone, Debug)]
pub struct RoomUserStatus {
    key: StatusType,
    value: String,
    action: Option<StatusType>,

    sec_action_switch: i32,
    sec_switch_lifetime: i32,
    lifetime_countdown: i32,

    action_countdown: i32,
    action_switch_countdown: i32,
}

impl RoomUserStatus {
    /// Mirrors the 2-arg `RoomUserStatus` constructor.
    pub fn new(status: StatusType, value: &str) -> Self {
        Self {
            key: status,
            value: value.to_string(),
            action: None,
            sec_action_switch: -1,
            sec_switch_lifetime: -1,
            lifetime_countdown: -1,
            action_countdown: -1,
            action_switch_countdown: -1,
        }
    }

    /// Mirrors the 7-arg `RoomUserStatus` constructor.
    pub fn new_with_action(
        status: StatusType,
        value: &str,
        sec_lifetime: i32,
        action: Option<StatusType>,
        sec_action_switch: i32,
        sec_switch_lifetime: i32,
    ) -> Self {
        Self {
            key: status,
            value: value.to_string(),
            action,
            sec_action_switch,
            sec_switch_lifetime,
            lifetime_countdown: sec_lifetime,
            action_countdown: sec_action_switch,
            action_switch_countdown: -1,
        }
    }

    /// Mirrors `swapKeyAction` (timed statuses, e.g. drinking). Java would
    /// null out `key` when `action` is unset; here the swap is a no-op in
    /// that case.
    pub fn swap_key_action(&mut self) {
        if self.action.is_some() {
            std::mem::swap(&mut self.key, self.action.as_mut().unwrap());
        }
    }

    /// Mirrors `getKey`.
    pub fn get_key(&self) -> StatusType {
        self.key
    }

    /// Getter for the `action` field (the Java class exposes no accessor).
    pub fn get_action(&self) -> Option<StatusType> {
        self.action
    }

    /// Mirrors `getValue`.
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Mirrors `setValue`.
    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }

    /// Mirrors `getSecActionSwitch`.
    pub fn get_sec_action_switch(&self) -> i32 {
        self.sec_action_switch
    }

    /// Mirrors `getSecSwitchLifetime`.
    pub fn get_sec_switch_lifetime(&self) -> i32 {
        self.sec_switch_lifetime
    }

    /// Mirrors `getLifetimeCountdown`.
    pub fn get_lifetime_countdown(&self) -> i32 {
        self.lifetime_countdown
    }

    /// Mirrors `setLifetimeCountdown`.
    pub fn set_lifetime_countdown(&mut self, lifetime_countdown: i32) {
        self.lifetime_countdown = lifetime_countdown;
    }

    /// Mirrors `getActionCountdown`.
    pub fn get_action_countdown(&self) -> i32 {
        self.action_countdown
    }

    /// Mirrors `setActionCountdown`.
    pub fn set_action_countdown(&mut self, action_countdown: i32) {
        self.action_countdown = action_countdown;
    }

    /// Mirrors `getActionSwitchCountdown`.
    pub fn get_action_switch_countdown(&self) -> i32 {
        self.action_switch_countdown
    }

    /// Mirrors `setActionSwitchCountdown`.
    pub fn set_action_switch_countdown(&mut self, action_switch_countdown: i32) {
        self.action_switch_countdown = action_switch_countdown;
    }
}
