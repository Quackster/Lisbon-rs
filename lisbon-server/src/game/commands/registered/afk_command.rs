//! Mirrors `net.h4bbo.lisbon.game.commands.registered.AfkCommand`.

use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::room::enums::status_type::StatusType;
use crate::messages::outgoing::rooms::user::user_statuses::USER_STATUSES;

/// Mirrors `AfkCommand`.
pub struct AfkCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for AfkCommand {
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
        self.permissions.push(Fuseright::Default);
    }

    /// Mirrors `handleCommand(Entity, String, String[])`.
    fn handle_command(&self, entity: &mut dyn Entity, _message: &str, _args: &[String]) {
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

        if room_user.is_walking() {
            return;
        }

        if !room_user.contains_status(StatusType::AvatarSleep) {
            room_user.remove_drinks();
            room_user.set_status(StatusType::AvatarSleep, "");
            room_user.set_needs_update(true);

            // Send immediate update to client
            let refs: Vec<&(dyn Entity + Send)> = vec![player];
            player.send(&USER_STATUSES::new(refs));
        }
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "Put your eyes to sleep".to_string()
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
