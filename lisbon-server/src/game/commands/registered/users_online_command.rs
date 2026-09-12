//! Mirrors `net.h4bbo.lisbon.game.commands.registered.UsersOnlineCommand`.

use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::util::string_util::StringUtil;

/// Mirrors `UsersOnlineCommand`.
pub struct UsersOnlineCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for UsersOnlineCommand {
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

        let Some(session) = entity.as_player() else {
            return;
        };

        const MAX_PLAYERS_PER_LINE: usize = 5;

        let players = PlayerManager::get_instance().get_players();
        let names = players
            .iter()
            .map(|p| p.lock().get_details().get_name().to_string())
            .collect::<Vec<String>>();
        let paginated_players = StringUtil::paginate(&names, MAX_PLAYERS_PER_LINE);

        let mut sb = String::new();
        sb.push_str(&format!("Users online: {}<br>", names.len()));
        sb.push_str(&format!(
            "Daily player peak count: {}<br>",
            PlayerManager::get_instance().get_daily_player_peak()
        ));
        sb.push_str("List of users online: <br><br>");

        for player_list in paginated_players.values() {
            let length = player_list.len();
            for (i, name) in player_list.iter().enumerate() {
                sb.push_str(name);

                if i < length - 1 {
                    sb.push_str(", ");
                }
            }

            sb.push_str("<br>");
        }

        session.send(&ALERT::new(&sb));
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "Get the list of players currently online".to_string()
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
