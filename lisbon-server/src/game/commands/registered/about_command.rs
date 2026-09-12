//! Mirrors `net.h4bbo.lisbon.game.commands.registered.AboutCommand`.

use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::messages::outgoing::alert::alert::ALERT;

/// Mirrors `AboutCommand`.
pub struct AboutCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for AboutCommand {
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

        player.send(&ALERT::new(
            "Project Lisbon - Habbo Hotel v26 emulation<br><br>Max version supported: r26_20080915_0408_7984_61ccb5f8b8797a3aba62c1fa2ca80169<br><br>Originally based off Kepler<br><br>Made by Quackster from RaGEZONE",
        ));
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        " Information about the software powering this retro".to_string()
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
