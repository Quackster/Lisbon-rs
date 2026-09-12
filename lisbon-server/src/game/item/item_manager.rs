//! Mirrors `net.h4bbo.lisbon.game.item.ItemManager`.
use std::collections::HashMap;
use std::sync::Arc;

use lazy_static::lazy_static;
use parking_lot::{Mutex, RwLock};
use rand::Rng;

use crate::dao::mysql::item_dao::ItemDao;
use crate::dao::mysql::song_machine_dao::SongMachineDao;
use crate::game::catalogue::catalogue_manager::CatalogueManager;
use crate::game::item::base::item_definition::ItemDefinition;
use crate::game::item::item::Item;
use crate::game::player::player_manager::PlayerManager;
use crate::game::room::room_manager::RoomManager;
use crate::log::Log;
use crate::util::date_util::DateUtil;

lazy_static! {
    static ref INSTANCE: RwLock<Option<Arc<ItemManager>>> = RwLock::new(None);
}

pub struct ItemManager {
    item_definition_map: HashMap<i32, ItemDefinition>,
}

impl ItemManager {
    fn new() -> Self {
        Self {
            item_definition_map: ItemDao::get_item_definitions(),
        }
    }

    /// Mirrors `createGift(int, String, String, String, String)`.
    pub fn create_gift(
        &self,
        owner_id: i32,
        received_from: &str,
        sale_code: &str,
        present_label: &str,
        extra_data: &str,
    ) -> Item {
        let present_id = rand::thread_rng().gen_range(0..7);
        let mut sprite = "present_gen".to_string();

        if present_id > 0 {
            sprite += &present_id.to_string();
        }

        let Some(item_def) = self.get_definition_by_sprite(&sprite) else {
            Log::get_error_logger()
                .error(&format!("createGift: no item definition for sprite {}", sprite));
            return Item::new();
        };

        let Some(catalogue_item) = CatalogueManager::get_instance().get_catalogue_item(sale_code)
        else {
            Log::get_error_logger()
                .error(&format!("createGift: no catalogue item for sale code {}", sale_code));
            return Item::new();
        };

        let mut item = Item::new();
        item.set_definition_id(item_def.get_id());
        item.set_owner_id(owner_id);
        item.set_custom_data(&format!(
            "{}{}{}{}{}{}{}{}{}",
            catalogue_item.get_id(),
            Item::PRESENT_DELIMETER,
            received_from,
            Item::PRESENT_DELIMETER,
            present_label.replace(Item::PRESENT_DELIMETER, ""),
            Item::PRESENT_DELIMETER,
            extra_data.replace(Item::PRESENT_DELIMETER, ""),
            Item::PRESENT_DELIMETER,
            DateUtil::get_current_time_seconds(),
        ));

        ItemDao::new_item(&mut item);

        item
    }

    /// Mirrors `getJukeboxTracks(int)`.
    pub fn get_jukebox_tracks(&self, item_id: i32) -> Vec<crate::game::song::song::Song> {
        let mut saved_tracks = Vec::new();

        for (slot_id, song_id) in SongMachineDao::get_tracks(item_id).iter() {
            if let Some(mut song) = SongMachineDao::get_song(*song_id) {
                song.set_slot_id(*slot_id);
                saved_tracks.push(song);
            }
        }

        saved_tracks
    }

    /// Mirrors `performItemSaving(BlockingQueue<Item>)`.
    pub fn perform_item_saving(&self, item_saving_queue: &Mutex<Vec<Item>>) {
        if item_saving_queue.lock().is_empty() {
            return;
        }

        let item_list = std::mem::take(&mut *item_saving_queue.lock());

        ItemDao::update_items(&item_list);
    }

    /// Mirrors `performItemDeletion(BlockingQueue<Integer>)`.
    pub fn perform_item_deletion(&self, item_deletion_queue: &Mutex<Vec<i32>>) {
        let item_list = {
            if item_deletion_queue.lock().is_empty() {
                return;
            }
            std::mem::take(&mut *item_deletion_queue.lock())
        };

        if !item_list.is_empty() {
            ItemDao::delete_items(&item_list);
        }
    }

    /// Mirrors `getDefinition(int)`.
    pub fn get_definition(&self, definition_id: i32) -> Option<ItemDefinition> {
        self.item_definition_map.get(&definition_id).cloned()
    }

    /// Mirrors `getDefinitionBySprite(String)`.
    pub fn get_definition_by_sprite(&self, sprite_name: &str) -> Option<ItemDefinition> {
        for definition in self.item_definition_map.values() {
            if definition.get_sprite() == sprite_name {
                return Some(definition.clone());
            }
        }

        None
    }

    /// Mirrors `resolveItem(int)`.
    pub fn resolve_item_by_id(&self, item_id: i32) -> Option<Item> {
        let database_item = ItemDao::get_item(item_id)?;

        self.resolve_item(&database_item)
    }

    /// Mirrors `resolveItem(Item)`.
    pub fn resolve_item(&self, database_item: &Item) -> Option<Item> {
        if RoomManager::get_instance().get_room_by_id(database_item.get_room_id()).is_some() {
            let Some(room) = database_item.get_room() else {
                return None;
            };
            let room = room.lock();

            return room
                .get_item_manager()
                .get_by_id(database_item.get_id())
                .map(|item| *item);
        }

        if let Some(player) = PlayerManager::get_instance().get_player_by_id(database_item.get_owner_id())
        {
            let player = player.lock();

            return player
                .get_inventory()
                .and_then(|inventory| inventory.get_item(database_item.get_id()));
        }

        None
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> Arc<ItemManager> {
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
