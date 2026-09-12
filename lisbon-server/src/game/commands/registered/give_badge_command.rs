//! Mirrors `net.h4bbo.lisbon.game.commands.registered.GiveBadgeCommand`.

use crate::dao::mysql::badge_dao::BadgeDao;
use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::badges::badge_manager::BadgeManager;
use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::messages::outgoing::rooms::user::chat_message::{ChatMessageType, CHAT_MESSAGE};
use crate::messages::outgoing::rooms::user::figure_change::FIGURE_CHANGE;

/// Mirrors `GiveBadgeCommand`.
pub struct GiveBadgeCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for GiveBadgeCommand {
    /// Mirrors the no-arg constructor.
    fn new() -> Self {
        let mut this = Self {
            permissions: Vec::new(),
            arguments: Vec::new(),
        };
        this.add_permissions();
        this.add_arguments();
        this
    }

    /// Mirrors `addPermissions()`.
    fn add_permissions(&mut self) {
        self.permissions.push(Fuseright::AdministratorAccess);
    }

    /// Mirrors `addArguments()`.
    fn add_arguments(&mut self) {
        self.arguments.push("user".to_string());
        self.arguments.push("badge".to_string());
    }

    /// Mirrors `handleCommand(Entity, String, String[])`.
    fn handle_command(&self, entity: &mut dyn Entity, _message: &str, args: &[String]) {
        // :givebadge Alex NL1

        // should refuse to give badges that belong to ranks
        if entity.get_type() != EntityType::Player {
            return;
        }

        let Some(player) = entity.as_player() else {
            return;
        };

        let Some(room_user) = player.get_room_user() else {
            return;
        };
        if room_user.get_room().is_none() {
            return;
        }

        if args.len() == 1 {
            player.send(&CHAT_MESSAGE::new(
                ChatMessageType::Whisper,
                room_user.get_instance_id(),
                "Badge code not provided",
            ));
            return;
        }

        let Some(target_user_details) = PlayerDao::get_details_by_name(&args[0]) else {
            player.send(&ALERT::new(&format!("Could not find user: {}", args[0])));
            return;
        };

        let badge = &args[1];

        if badge.starts_with("GL")
            || badge.starts_with("ACH_")
            || badge.eq_ignore_ascii_case("Z64")
        {
            return;
        }

        let badge_manager = BadgeManager::new(target_user_details.get_id());

        let Some(target_user) = PlayerManager::get_instance().get_player_by_name(&args[0]) else {
            // Check if user already owns badge
            if badge_manager.has_badge(badge) {
                player.send(&CHAT_MESSAGE::new(
                    ChatMessageType::Whisper,
                    room_user.get_instance_id(),
                    &format!(
                        "User {} already owns this badge.",
                        target_user_details.get_name()
                    ),
                ));
                return;
            }

            let rank_badges = BadgeDao::get_rank_badges();

            // Check if badge code is a rank badge
            if rank_badges.contains(badge) {
                player.send(&CHAT_MESSAGE::new(
                    ChatMessageType::Whisper,
                    room_user.get_instance_id(),
                    &format!(
                        "This badge belongs to a certain rank. If you would like to give {} this badge, increase their rank.",
                        target_user_details.get_name()
                    ),
                ));
                return;
            }

            // Add badge
            badge_manager.try_add_badge(badge, None, 0);

            player.send(&CHAT_MESSAGE::new(
                ChatMessageType::Whisper,
                room_user.get_instance_id(),
                &format!(
                    "Badge {} added to user {}",
                    badge, target_user_details.get_name()
                ),
            ));
            return;
        };

        let target_user = target_user.lock();

        // Check if user already owns badge
        if target_user.get_badge_manager().has_badge(badge) {
            player.send(&CHAT_MESSAGE::new(
                ChatMessageType::Whisper,
                room_user.get_instance_id(),
                &format!(
                    "User {} already owns this badge.",
                    target_user_details.get_name()
                ),
            ));
            return;
        }

        let rank_badges = BadgeDao::get_rank_badges();

        // Check if badge code is a rank badge
        if rank_badges.contains(badge) {
            player.send(&CHAT_MESSAGE::new(
                ChatMessageType::Whisper,
                room_user.get_instance_id(),
                &format!(
                    "This badge belongs to a certain rank. If you would like to give {} this badge, increase their rank.",
                    target_user_details.get_name()
                ),
            ));
            return;
        }

        // Add badge
        target_user.get_badge_manager().try_add_badge(badge, None, 0);
        target_user.get_badge_manager().refresh_badges(&target_user);

        // Let other room users know something changed if targetUser is inside a room
        if let Some(target_room_user) = target_user.get_room_user() {
            if let Some(_target_room) = target_room_user.get_room() {
                _target_room.send(&FIGURE_CHANGE::new(
                    target_room_user.get_instance_id(),
                    target_user.get_details(),
                ));
            }
        }

        player.send(&CHAT_MESSAGE::new(
            ChatMessageType::Whisper,
            room_user.get_instance_id(),
            &format!(
                "Badge {} added to user {}",
                badge, target_user_details.get_name()
            ),
        ));
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "Add badge to user".to_string()
    }

    /// Mirrors `getPermissions()`.
    fn get_permissions(&self) -> Vec<Fuseright> {
        self.permissions.clone()
    }

    /// Mirrors `getArguments()`.
    fn get_arguments(&self) -> Vec<String> {
        self.arguments.clone()
    }
}
