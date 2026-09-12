//! Mirrors `net.h4bbo.lisbon.game.commands.registered.HelpCommand`.

use crate::game::commands::command::Command;
use crate::game::commands::command_manager::CommandManager;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::util::string_util::StringUtil;

/// Mirrors `HelpCommand`.
pub struct HelpCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for HelpCommand {
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
    fn handle_command(&self, entity: &mut dyn Entity, _message: &str, args: &[String]) {
        let commands = CommandManager::get_instance().get_commands();
        let commands = StringUtil::paginate(&commands, 10);

        let mut page_id = 1;

        if !args.is_empty() && args[0].chars().all(|c| c.is_ascii_digit()) {
            page_id = args[0].parse().unwrap_or(1);
        }

        if page_id < 1 || !commands.contains_key(&(page_id - 1)) {
            page_id = 1;
        }

        let command_list = commands.get(&(page_id - 1)).cloned().unwrap_or_default();

        let mut about = String::new();
        about.push_str("Commands ('<' and '>' are optional parameters):<br>");
        about.push_str("<br>");

        for (command_alias, command) in &command_list {
            if !CommandManager::get_instance().has_command_permission(entity, command.as_ref()) {
                continue;
            }

            about.push(':');
            about.push_str(&command_alias.join("/"));

            let command_arguments = command.get_arguments();

            if !command_arguments.is_empty() {
                if command_arguments.len() > 1 {
                    about.push_str(&format!(" [{}]", command_arguments.join("] [")));
                } else {
                    about.push_str(&format!(" [{}]", command_arguments[0]));
                }
            }

            about.push_str(" - ");
            about.push_str(&command.get_description());
            about.push_str("<br>");
        }

        about.push_str("<br>");
        about.push_str("Page ");
        about.push_str(&page_id.to_string());
        about.push_str(" out of ");
        about.push_str(&commands.len().to_string());

        if let Some(player) = entity.as_player() {
            player.send(&ALERT::new(&about));
        }
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "<page> - List available commands".to_string()
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
