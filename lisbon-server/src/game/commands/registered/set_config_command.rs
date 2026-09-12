//! Mirrors `net.h4bbo.lisbon.game.commands.registered.SetConfigCommand`.

use crate::dao::mysql::settings_dao::SettingsDao;
use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::util::config::game_configuration::GameConfiguration;
use crate::util::config::writer::game_config_writer::GameConfigWriter;

/// Mirrors `SetConfigCommand`.
pub struct SetConfigCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for SetConfigCommand {
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
        self.arguments.push("setting".to_string());
        self.arguments.push("value".to_string());
    }

    /// Mirrors `handleCommand(Entity, String, String[])`.
    fn handle_command(&self, entity: &mut dyn Entity, _message: &str, args: &[String]) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        let Some(player) = entity.as_player() else {
            return;
        };

        let setting = &args[0];
        let value = &args[1];

        let old_value;
        {
            let config_instance = GameConfiguration::get_instance();
            let config = config_instance.get_config();

            if !config.contains_key(setting) {
                player.send(&ALERT::new(&format!(
                    "The setting \"{}\" doesn't exist!",
                    setting
                )));
                return;
            }

            old_value = config
                .get(setting)
                .map(String::as_str)
                .unwrap_or_default()
                .to_string();
        }

        SettingsDao::update_setting(setting, value);
        GameConfiguration::reset(GameConfigWriter::new());

        player.send(&ALERT::new(&format!(
            "The setting \"{}\" value has been updated from \"{}\" to \"{}\"",
            setting, old_value, value
        )));
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "In-game housekeeping for the catalogue item prices.".to_string()
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
