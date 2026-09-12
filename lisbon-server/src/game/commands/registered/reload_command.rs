//! Mirrors `net.h4bbo.lisbon.game.commands.registered.ReloadCommand`.

use crate::game::achievements::achievement_manager::AchievementManager;
use crate::game::ads::ad_manager::AdManager;
use crate::game::catalogue::catalogue_manager::CatalogueManager;
use crate::game::catalogue::collectables::collectables_manager::CollectablesManager;
use crate::game::commands::command::Command;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::events::events_manager::EventsManager;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::games::game_manager::GameManager;
use crate::game::games::snowstorm::snowstorm_maps_manager::SnowStormMapsManager;
use crate::game::item::item_manager::ItemManager;
use crate::game::room::models::room_model_manager::RoomModelManager;
use crate::game::texts::texts_manager::TextsManager;
use crate::game::wordfilter::wordfilter_manager::WordfilterManager;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::util::config::game_configuration::GameConfiguration;
use crate::util::config::writer::game_config_writer::GameConfigWriter;

/// Mirrors `ReloadCommand`.
pub struct ReloadCommand {
    permissions: Vec<Fuseright>,
    arguments: Vec<String>,
}

impl Command for ReloadCommand {
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
        self.arguments.push("component".to_string());
    }

    /// Mirrors `handleCommand(Entity, String, String[])`.
    fn handle_command(&self, entity: &mut dyn Entity, _message: &str, args: &[String]) {
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

        let component = &args[0];
        let component = component.to_lowercase();
        let mut component_name: Option<&str> = None;

        if component == "catalogue" || component == "shop" || component == "items" {
            ItemManager::reset();
            CatalogueManager::reset();

            // Regenerate collision map with proper height differences
            // (if they changed).
            if let Some(room) = room_user.get_room() {
                room.get_mapping().lock().regenerate_collision_map(&room);
            }
            component_name = Some("Catalogue and item definitions");
        }

        if component == "wordfilter" {
            WordfilterManager::reset();
            component_name = Some("Wordfilter");
        }

        if component == "ads" {
            AdManager::reset();
            component_name = Some("Advertisements");
        }

        if component == "texts" {
            TextsManager::reset();
            component_name = Some("Texts");
        }

        if component == "games" {
            GameManager::reset();
            component_name = Some("Games");
        }

        if component == "events" {
            EventsManager::reset();
            component_name = Some("Events");
        }

        if component == "gamemaps" {
            SnowStormMapsManager::reset();
            component_name = Some("game maps");
        }

        if component == "achievements" || component == "ach" {
            AchievementManager::reset();
            component_name = Some("Achievements");
        }

        if component == "models" {
            RoomModelManager::reset();
            component_name = Some("Room models");
        }

        if component == "collectables" {
            CollectablesManager::reset();
            component_name = Some("Collectables");
        }

        if component == "settings" || component == "config" {
            GameConfiguration::reset(GameConfigWriter::new());
            component_name = Some("Game settings");
        }

        if let Some(component_name) = component_name {
            player.send(&ALERT::new(&format!("{} have been reloaded.", component_name)));
        } else {
            player.send(&ALERT::new(
                "You did not specify which component to reload!<br>You may reload either the catalogue/shop/items, advertisements, events, commands,<br>navigator, collectables, models, texts, plugins, wordfitler, games, badgebuy,<br>rewards, versions or settings.",
            ));
        }
    }

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String {
        "Refresh the settings/items/texts".to_string()
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
