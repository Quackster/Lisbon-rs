//! Mirrors `net.h4bbo.lisbon.game.commands.registered.ChangeMottoCommand`.

use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::messages::outgoing::rooms::user::figure_change::FIGURE_CHANGE;
use crate::util::string_util::StringUtil;

/// Mirrors `ChangeMottoCommand`.
pub struct ChangeMottoCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for ChangeMottoCommand {
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
        if entity.get_type() != EntityType::Player {
            return;
        }

        let Some(player) = entity.as_player_mut() else {
            return;
        };

        if player
            .get_room_user()
            .map(crate::game::room::entities::room_entity::RoomEntity::get_room)
            .flatten()
            .is_none()
        {
            return;
        }

        // Filter out possible packet injection attacks
        let motto = if !args.is_empty() {
            StringUtil::filter_input(&args.join(" "), true)
        } else {
            String::new()
        };

        // Update motto
        player.get_details_mut().set_motto(&motto);
        PlayerDao::save_motto(player.get_details().get_id(), &motto);

        let Some(room_user) = player.get_room_user() else {
            return;
        };

        // Notify room of changed motto
        if let Some(room) = room_user.get_room() {
            room.send(&FIGURE_CHANGE::new(
                room_user.get_instance_id(),
                player.get_details(),
            ));
        }
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "Change your motto".to_string()
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
