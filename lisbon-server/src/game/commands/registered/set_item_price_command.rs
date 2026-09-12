//! Mirrors `net.h4bbo.lisbon.game.commands.registered.SetItemPriceCommand`.

use crate::dao::mysql::catalogue_dao::CatalogueDao;
use crate::game::catalogue::catalogue_manager::CatalogueManager;
use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::messages::outgoing::rooms::user::chat_message::{ChatMessageType, CHAT_MESSAGE};

/// Mirrors `SetItemPriceCommand`.
pub struct SetItemPriceCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for SetItemPriceCommand {
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
        self.arguments.push("sale code".to_string());
        self.arguments.push("price".to_string());
    }

    /// Mirrors `handleCommand(Entity, String, String[])`.
    fn handle_command(&self, entity: &mut dyn Entity, _message: &str, args: &[String]) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        let Some(player) = entity.as_player() else {
            return;
        };

        let sale_code = &args[0];

        let Some(mut item) = CatalogueManager::get_instance().get_catalogue_item(sale_code) else {
            player.send(&ALERT::new("That sale code doesn't exist!"));
            return;
        };

        if !args[1].chars().all(|c| c.is_ascii_digit()) {
            player.send(&ALERT::new("You did not enter a number!"));
            return;
        }

        let new_price = args[1].parse::<i32>().unwrap_or(0);

        if item.get_price() == new_price {
            player.send(&ALERT::new(
                "You entered the same price that the catalogue item costs!",
            ));
            return;
        }

        CatalogueDao::set_price(item.get_sale_code(), new_price);

        let word = "increased";

        let word = if item.get_price() > new_price {
            "decreased"
        } else {
            word
        };

        let instance_id = player
            .get_room_user()
            .map(crate::game::room::entities::room_entity::RoomEntity::get_instance_id)
            .unwrap_or(0);

        player.send(&CHAT_MESSAGE::new(
            ChatMessageType::Whisper,
            instance_id,
            &format!(
                "The {} has successfully {} from {} to {}",
                item.get_name(),
                word,
                item.get_price(),
                new_price
            ),
        ));
        CatalogueManager::get_instance()
            .set_item_price(item.get_sale_code(), new_price);
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
