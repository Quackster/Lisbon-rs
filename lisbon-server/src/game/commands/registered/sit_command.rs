//! Mirrors `net.h4bbo.lisbon.game.commands.registered.SitCommand`.

use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::room::enums::status_type::StatusType;
use crate::util::string_util::StringUtil;

/// Mirrors `SitCommand`.
pub struct SitCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for SitCommand {
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
        let Some(room) = room_user.get_room() else {
            return;
        };

        if room_user.contains_status(StatusType::Sit) {
            return;
        }

        if room_user.contains_status(StatusType::Swim) {
            return;
        }

        let mut height = 0.5;

        if room.is_public_room()
            && room
                .get_model()
                .map(|model| model.get_name().starts_with("pool_"))
                .unwrap_or(false)
        {
            height = 0.0;
        }

        let position = room_user.get_position();
        let rotation = position.get_rotation() / 2 * 2;
        let item = room_user.get_current_item();

        if let Some(item) = &item {
            if item.has_behaviour(ItemBehaviour::CanSitOnTop)
                || item.has_behaviour(ItemBehaviour::CanLayOnTop)
            {
                return;
            }

            if !item.has_behaviour(ItemBehaviour::Roller) {
                height += item.get_definition().get_top_height();
            }
        }

        let mut position = position;
        position.set_rotation(rotation);
        room_user.set_position(position);

        room_user.set_status(StatusType::Sit, &StringUtil::format(height).to_string());
        room_user.remove_status(StatusType::Dance);
        room_user.set_needs_update(true);
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "Parks your arse on the floor".to_string()
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
