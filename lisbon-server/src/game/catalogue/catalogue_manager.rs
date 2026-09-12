//! Mirrors `net.h4bbo.lisbon.game.catalogue.CatalogueManager`.

use std::sync::Arc;

use lazy_static::lazy_static;
use parking_lot::RwLock;

use crate::dao::mysql::catalogue_dao::CatalogueDao;
use crate::dao::mysql::currency_dao::CurrencyDao;
use crate::dao::mysql::item_dao::ItemDao;
use crate::dao::mysql::pet_dao::PetDao;
use crate::dao::mysql::teleporter_dao::TeleporterDao;
use crate::game::catalogue::catalogue_item::CatalogueItem;
use crate::game::catalogue::catalogue_page::CataloguePage;
use crate::game::catalogue::catalogue_package::CataloguePackage;
use crate::game::catalogue::collectables::collectables_manager::CollectablesManager;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::base::item_definition::ItemDefinition;
use crate::game::item::interactors::interaction_type::InteractionType;
use crate::game::item::item::Item;
use crate::game::item::item_manager::ItemManager;
use crate::game::pets::pet_manager::PetManager;
use crate::game::player::player_details::PlayerDetails;
use crate::game::player::player_manager::PlayerManager;
use crate::game::player::player_rank::PlayerRank;
use crate::messages::outgoing::user::currencies::film::FILM;
use crate::util::date_util::DateUtil;
use crate::util::string_util::StringUtil;

lazy_static! {
    static ref INSTANCE: RwLock<Option<Arc<CatalogueManager>>> = RwLock::new(None);
}

pub struct CatalogueManager {
    catalogue_page_list: Vec<CataloguePage>,
    catalogue_item_list: parking_lot::Mutex<Vec<CatalogueItem>>,
    catalogue_package_list: Vec<CataloguePackage>,
}

impl CatalogueManager {
    fn new() -> Self {
        let catalogue_page_list = CatalogueDao::get_pages();
        let catalogue_package_list = CatalogueDao::get_packages();
        let catalogue_item_list = CatalogueDao::get_items();

        let mut manager = Self {
            catalogue_page_list,
            catalogue_item_list: parking_lot::Mutex::new(catalogue_item_list),
            catalogue_package_list,
        };
        manager.load_packages();
        manager
    }

    /// Mirrors `purchase(PlayerDetails, CatalogueItem, String, String, long)`.
    pub fn purchase(
        &self,
        player_details: &PlayerDetails,
        item: CatalogueItem,
        extra_data: Option<&str>,
        override_name: Option<&str>,
        timestamp: i64,
    ) -> Vec<Item> {
        let mut items_bought = Vec::new();

        if !item.is_package() {
            if let Some(definition) = item.get_definition() {
                if let Some(new_item) = self.purchase_single(
                    player_details,
                    definition,
                    extra_data,
                    item.get_item_special_id(),
                    override_name,
                    timestamp,
                ) {
                    items_bought.push(new_item);
                }
            }
        } else {
            for catalogue_package in item.get_packages() {
                for _ in 0..catalogue_package.get_amount() {
                    if let Some(definition) = catalogue_package.get_definition() {
                        if let Some(new_item) = self.purchase_single(
                            player_details,
                            definition,
                            None,
                            catalogue_package.get_special_sprite_id(),
                            override_name,
                            timestamp,
                        ) {
                            items_bought.push(new_item);
                        }
                    }
                }
            }
        }

        items_bought
    }

    /// Mirrors `purchase(PlayerDetails, ItemDefinition, String, int, String, long)`.
    pub fn purchase_single(
        &self,
        player_details: &PlayerDetails,
        def: ItemDefinition,
        extra_data: Option<&str>,
        special_sprite_id: i32,
        override_name: Option<&str>,
        _timestamp: i64,
    ) -> Option<Item> {
        let player_arc = PlayerManager::get_instance()
            .get_player_by_id(player_details.get_id())?;
        let player = player_arc.lock();

        if def.get_sprite() == "film" {
            CurrencyDao::increase_film(player_details, 5);
            player.send(&FILM::new(player_details));
            return None;
        }

        let mut custom_data = String::new();

        if let Some(extra_data) = extra_data {
            if def.has_behaviour(ItemBehaviour::Decoration) {
                custom_data = extra_data.to_string();
            } else if special_sprite_id > 0 {
                custom_data = special_sprite_id.to_string();
            }

            if def.has_behaviour(ItemBehaviour::PostIt) {
                custom_data = "20".to_string();
            }

            if def.has_behaviour(ItemBehaviour::PrizeTrophy) {
                custom_data += override_name.unwrap_or_else(|| player_details.get_name());
                custom_data += "\t";

                custom_data += &DateUtil::get_short_date();
                custom_data += "\t";

                custom_data += &StringUtil::filter_input(extra_data, true);
            }

            if def.has_behaviour(ItemBehaviour::RoomDimmer) {
                custom_data = Item::DEFAULT_ROOMDIMMER_CUSTOM_DATA.to_string();
            }
        }

        let mut item = Item::new();
        item.set_owner_id(player_details.get_id());
        item.set_definition_id(def.get_id());

        if def.get_interaction_type() == Some(InteractionType::PetNest) {
            if extra_data.is_some() {
                if let Some(nest) = ItemManager::get_instance().get_definition_by_sprite("nest") {
                    item.set_definition_id(nest.get_id());
                }
            }
        }

        item.set_custom_data(&custom_data);

        ItemDao::new_item(&mut item);
        if let Some(inventory) = player.get_inventory() {
            inventory.add_item(&item);
        }

        if def.get_sprite() == "camera" {
            CurrencyDao::increase_film(player_details, 2);
            player.send(&FILM::new(player_details));
        }

        if def.has_behaviour(ItemBehaviour::Teleporter) {
            let mut linked_teleporter_item = Item::new();
            linked_teleporter_item.set_owner_id(player_details.get_id());
            linked_teleporter_item.set_definition_id(def.get_id());
            linked_teleporter_item.set_custom_data(&custom_data);

            ItemDao::new_item(&mut linked_teleporter_item);
            if let Some(inventory) = player.get_inventory() {
                inventory.add_item(&linked_teleporter_item);
            }

            let item_id = item.get_id();
            let linked_id = linked_teleporter_item.get_id();
            linked_teleporter_item.set_teleporter_id(item_id);
            item.set_teleporter_id(linked_id);

            TeleporterDao::add_pair(linked_id, item_id);
            TeleporterDao::add_pair(item_id, linked_id);
        }

        if def.get_interaction_type() == Some(InteractionType::PetNest) {
            if let Some(extra_data) = extra_data {
                let pet_data: Vec<&str> = extra_data.split('\u{2}').collect();

                if pet_data.len() >= 3 {
                    let name = StringUtil::filter_input(pet_data[0], true);
                    let type_ = def.get_sprite().replace("pets", "");
                    let race = pet_data[1].parse::<i32>().unwrap_or(0);
                    let color = StringUtil::filter_input(pet_data[2], true);

                    if PetManager::get_instance()
                        .is_valid_name(player_details.get_name(), &name)
                    {
                        PetDao::create_pet(item.get_id() as i64, &name, &type_, race, &color);
                    }
                }
            }
        }

        Some(item)
    }

    /// Mirrors `loadPackages()`.
    fn load_packages(&mut self) {
        for catalogue_item in self.catalogue_item_list.lock().iter_mut() {
            if !catalogue_item.is_package() {
                continue;
            }

            for catalogue_package in &self.catalogue_package_list {
                if catalogue_item.get_sale_code() == catalogue_package.get_sale_code() {
                    catalogue_item.add_package(catalogue_package.clone());
                }
            }
        }
    }

    /// Mirrors `getCataloguePage(String)`.
    pub fn get_catalogue_page(&self, page_index: &str) -> Option<CataloguePage> {
        for catalogue_page in &self.catalogue_page_list {
            if catalogue_page.get_name_index() == page_index {
                return Some(catalogue_page.clone());
            }
        }

        None
    }

    /// Mirrors `getCatalogueItem(String)`.
    pub fn get_catalogue_item(&self, sale_code: &str) -> Option<CatalogueItem> {
        for catalogue_item in self.catalogue_item_list.lock().iter() {
            if catalogue_item.is_hidden() {
                continue;
            }

            if catalogue_item.get_sale_code() == sale_code {
                return Some(catalogue_item.clone());
            }
        }

        None
    }

    /// Mirrors the `item.setPrice(newPrice)` mutation of the shared
    /// catalogue item in `SetItemPriceCommand`.
    pub fn set_item_price(&self, sale_code: &str, price: i32) {
        for catalogue_item in self.catalogue_item_list.lock().iter_mut() {
            if catalogue_item.get_sale_code() == sale_code {
                catalogue_item.set_price(price);
            }
        }
    }

    /// Mirrors `getCataloguePageItems(int, boolean)`.
    pub fn get_catalogue_page_items(&self, page_id: i32, raw_items: bool) -> Vec<CatalogueItem> {
        let mut items = Vec::new();

        if !raw_items {
            if let Some(collectable_data) =
                CollectablesManager::get_instance().get_collectable_data_by_page(page_id)
            {
                if let Some(collectable_item) = collectable_data.get_active_item() {
                    return vec![collectable_item];
                }
            }
        }

        for catalogue_item in self.catalogue_item_list.lock().iter() {
            if catalogue_item.is_hidden() {
                continue;
            }

            if catalogue_item.has_page(page_id) {
                items.push(catalogue_item.clone());
            }
        }

        items
    }

    /// Mirrors `getCataloguePages()`.
    pub fn get_catalogue_pages(&self) -> Vec<CataloguePage> {
        self.catalogue_page_list.clone()
    }

    /// Mirrors `getPagesForRank(PlayerRank, boolean)`.
    pub fn get_pages_for_rank(
        &self,
        minimum_rank: PlayerRank,
        has_club: bool,
    ) -> Vec<CataloguePage> {
        let mut catalogue_pages_for_rank = Vec::new();

        for page in &self.catalogue_page_list {
            if !page.is_index_visible() {
                continue;
            }

            if page.is_club_only() && !has_club {
                continue;
            }

            if minimum_rank.rank_id() >= page.get_min_role().rank_id() {
                catalogue_pages_for_rank.push(page.clone());
            }
        }

        catalogue_pages_for_rank
    }

    /// Mirrors `getCatalogueItems()`.
    pub fn get_catalogue_items(&self) -> Vec<CatalogueItem> {
        self.catalogue_item_list.lock().clone()
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> Arc<CatalogueManager> {
        if let Some(existing) = INSTANCE.read().as_ref() {
            return existing.clone();
        }
        let instance = Arc::new(Self::new());
        INSTANCE.write().replace(instance.clone());
        instance
    }

    /// Mirrors `reset()`.
    pub fn reset() {
        INSTANCE.write().take();
        Self::get_instance();
    }
}
