//! Mirrors `net.h4bbo.lisbon.game.commands.registered.ShutdownCommand`.

use std::time::Duration;

use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::outgoing::rooms::user::chat_message::{ChatMessageType, CHAT_MESSAGE};
use crate::util::config::game_configuration::GameConfiguration;

/// Mirrors `ShutdownCommand`.
pub struct ShutdownCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for ShutdownCommand {
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
    fn add_arguments(&mut self) {}

    /// Mirrors `handleCommand(Entity, String, String[])`.
    fn handle_command(&self, entity: &mut dyn Entity, _message: &str, args: &[String]) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        let Some(player) = entity.as_player() else {
            return;
        };

        let instance_id = player
            .get_room_user()
            .map(crate::game::room::entities::room_entity::RoomEntity::get_instance_id)
            .unwrap_or(0);

        // Abort maintenance shutdown if provided argument is either cancel, off or stop (case insensitive)
        if !args.is_empty() {
            if args[0].eq_ignore_ascii_case("cancel")
                || args[0].eq_ignore_ascii_case("off")
                || args[0].eq_ignore_ascii_case("stop")
            {
                PlayerManager::get_instance().cancel_maintenance();
                player.send(&CHAT_MESSAGE::new(
                    ChatMessageType::Whisper,
                    instance_id,
                    "Cancelled shutdown",
                ));
                return;
            }
        }

        // Try parsing minutes argument, use default if failed
        let default_minutes = GameConfiguration::get_instance().get_long("shutdown.minutes");
        let minutes = match (
            !args.is_empty(),
            args.first().and_then(|arg| arg.parse::<i64>().ok()),
        ) {
            (true, Some(minutes)) => minutes,
            _ => {
                if args.len() > 0 {
                    player.send(&CHAT_MESSAGE::new(
                        ChatMessageType::Whisper,
                        instance_id,
                        &format!(
                            "Failed to parse minutes provided to shutdown command, defaulting to {} minute(s)",
                            default_minutes
                        ),
                    ));
                }
                default_minutes
            }
        };

        // Enqueue maintenance shutdown
        PlayerManager::get_instance()
            .plan_maintenance(Duration::from_secs(minutes.saturating_mul(60) as u64));

        // Let callee know Kepler is shutting down in X minutes
        player.send(&CHAT_MESSAGE::new(
            ChatMessageType::Whisper,
            instance_id,
            &format!("Shutting down in {} minute(s)", minutes),
        ));
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "<minutes> - Shutdown Kepler".to_string()
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
