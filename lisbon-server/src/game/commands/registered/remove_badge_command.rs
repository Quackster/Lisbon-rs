//! Mirrors `net.h4bbo.lisbon.game.commands.registered.RemoveBadgeCommand`.

use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::badges::badge_manager::BadgeManager;
use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::messages::outgoing::rooms::user::chat_message::{ChatMessageType, CHAT_MESSAGE};

/// Mirrors `RemoveBadgeCommand`.
pub struct RemoveBadgeCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for RemoveBadgeCommand {
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
        self.permissions.push(Fuseright::ModeratorAccess);
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

        let Some(target_user) = PlayerDao::get_details_by_name(&args[0]) else {
            player.send(&CHAT_MESSAGE::new(
                ChatMessageType::Whisper,
                room_user.get_instance_id(),
                &format!("Could not find user: {}", args[0]),
            ));
            return;
        };

        if args.len() == 1 {
            player.send(&CHAT_MESSAGE::new(
                ChatMessageType::Whisper,
                room_user.get_instance_id(),
                "Badge code not provided",
            ));
            return;
        }

        let badge = &args[1];

        if badge.starts_with("GL")
            || badge.starts_with("ACH_")
            || badge.eq_ignore_ascii_case("Z64")
        {
            return;
        }

        let badge_manager = BadgeManager::new(target_user.get_id());

        // Check if user already owns badge
        if !badge_manager.has_badge(badge) {
            player.send(&CHAT_MESSAGE::new(
                ChatMessageType::Whisper,
                room_user.get_instance_id(),
                &format!(
                    "User {} does not have this badge.",
                    target_user.get_name()
                ),
            ));
            return;
        }

        // Remove badge
        badge_manager.remove_badge(badge);
        player.send(&CHAT_MESSAGE::new(
            ChatMessageType::Whisper,
            room_user.get_instance_id(),
            &format!(
                "Badge {} removed from user {}",
                badge,
                target_user.get_name()
            ),
        ));
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "Remove badge from user".to_string()
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
