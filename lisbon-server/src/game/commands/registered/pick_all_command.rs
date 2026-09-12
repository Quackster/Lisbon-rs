//! Mirrors `net.h4bbo.lisbon.game.commands.registered.PickAllCommand`.

use crate::dao::mysql::item_dao::ItemDao;
use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::item::base::item_behaviour::ItemBehaviour;

/// Mirrors `PickAllCommand`.
pub struct PickAllCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for PickAllCommand {
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

        if !room.is_owner(player.get_details().get_id())
            && !player.has_fuse(&Fuseright::AnyRoomController)
        {
            return;
        }

        let mut items_to_pickup = Vec::new();

        for item in room.get_items() {
            if item.has_behaviour(ItemBehaviour::PublicSpaceObject) {
                continue;
            }

            if item.has_behaviour(ItemBehaviour::PostIt) {
                continue;
            }

            items_to_pickup.push(item);
        }

        for item in &mut items_to_pickup {
            item.set_owner_id(player.get_details().get_id());

            room.get_mapping().lock().pickup_item(&room, player, item);

            if let Some(inventory) = player.get_inventory() {
                inventory.add_item(item);
            }
        }

        ItemDao::update_items(&items_to_pickup);
        if let Some(inventory) = player.get_inventory() {
            inventory.view(player, "new");
        }
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "Allows the owner to pick up all furniture in a room".to_string()
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
