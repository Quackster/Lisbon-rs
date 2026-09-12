//! Mirrors `net.h4bbo.lisbon.game.commands.clientside.ChooserCommand`.

use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;

/// Mirrors `ChooserCommand` (client-side, `handleCommand` is a no-op).
pub struct ChooserCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for ChooserCommand {
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
    fn handle_command(&self, _entity: &mut dyn Entity, _message: &str, _args: &[String]) {}

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "List users in current room (club membership required)".to_string()
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
