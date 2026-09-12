//! Mirrors `net.h4bbo.lisbon.game.commands.registered.HotelAlertCommand`.

use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::util::string_util::StringUtil;

/// Mirrors `HotelAlertCommand`.
pub struct HotelAlertCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for HotelAlertCommand {
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

        let alert = StringUtil::filter_input(&args.join(" "), true);

        // Send all players an alert
        PlayerManager::get_instance().send_all(&ALERT::new(&alert));
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "Sends an alert hotel-wide".to_string()
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
