//! Mirrors `net.h4bbo.lisbon.game.moderation.ModerationActionType`.

use std::sync::Arc;

use crate::game::moderation::actions::moderator_alert_user_action::ModeratorAlertUserAction;
use crate::game::moderation::actions::moderator_ban_user_action::ModeratorBanUserAction;
use crate::game::moderation::actions::moderator_kick_user_action::ModeratorKickUserAction;
use crate::game::moderation::actions::moderator_room_alert_action::ModeratorRoomAlertAction;
use crate::game::moderation::actions::moderator_room_kick_action::ModeratorRoomKickAction;
use crate::game::moderation::moderation_action::ModerationAction;

/// Mirrors the `ModerationActionType` enum (the per-variant
/// `ModerationAction` instances the Java enum constants hold eagerly are
/// constructed on demand here).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ModerationActionType {
    AlertUser,
    KickUser,
    BanUser,
    RoomAlert,
    RoomKick,
}

impl ModerationActionType {
    /// Mirrors `getTargetType`.
    pub fn get_target_type(&self) -> i32 {
        match self {
            Self::AlertUser | Self::KickUser | Self::BanUser => 0,
            Self::RoomAlert | Self::RoomKick => 1,
        }
    }

    /// Mirrors `getActionType`.
    pub fn get_action_type(&self) -> i32 {
        match self {
            Self::AlertUser | Self::RoomAlert => 0,
            Self::KickUser | Self::RoomKick => 1,
            Self::BanUser => 2,
        }
    }

    /// Mirrors `getModerationAction`.
    pub fn get_moderation_action(&self) -> Arc<dyn ModerationAction + Send> {
        match self {
            Self::AlertUser => Arc::new(ModeratorAlertUserAction),
            Self::KickUser => Arc::new(ModeratorKickUserAction),
            Self::BanUser => Arc::new(ModeratorBanUserAction),
            Self::RoomAlert => Arc::new(ModeratorRoomAlertAction),
            Self::RoomKick => Arc::new(ModeratorRoomKickAction),
        }
    }

    /// Mirrors the enum `name()` (lower-cased by `ModerationDao.addLog`).
    pub fn name(&self) -> &'static str {
        match self {
            Self::AlertUser => "alert_user",
            Self::KickUser => "kick_user",
            Self::BanUser => "ban_user",
            Self::RoomAlert => "room_alert",
            Self::RoomKick => "room_kick",
        }
    }
}
