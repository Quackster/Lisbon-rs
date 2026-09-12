//! Mirrors `net.h4bbo.lisbon.game.commands.registered.PacketTestCommand`.

use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;

/// Mirrors `PacketTestCommand`.
pub struct PacketTestCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for PacketTestCommand {
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

    /// Mirrors `handleCommand(Entity, String, String[])`.
    fn handle_command(&self, entity: &mut dyn Entity, _message: &str, args: &[String]) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        let Some(player) = entity.as_player() else {
            return;
        };

        let mut packet: String = args.join(" ");

        for i in 0..14 {
            packet = packet.replace(
                &format!("{{{}}}", i),
                &char::from_u32(i).map(|c| c.to_string()).unwrap_or_default(),
            );
        }

        // Add ending packet suffix
        packet.push(char::from_u32(1).unwrap());

        player.send_object(&packet);
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "Tests a Habbo client-sided packet".to_string()
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
