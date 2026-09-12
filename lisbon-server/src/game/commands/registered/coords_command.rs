//! Mirrors `net.h4bbo.lisbon.game.commands.registered.CoordsCommand`.

use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::util::string_util::StringUtil;

/// Mirrors `CoordsCommand`.
pub struct CoordsCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for CoordsCommand {
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

        let position = room_user.get_position();
        player.send(&ALERT::new(&format!(
            "Your coordinates:<br>X: {}<br>Y: {}<br>Z: {}<br><br>Head rotation: {}<br>Body rotation: {}",
            position.get_x(),
            position.get_y(),
            StringUtil::format(position.get_z()),
            position.get_head_rotation(),
            position.get_body_rotation()
        )));
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "Shows the coordinates in the room".to_string()
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
