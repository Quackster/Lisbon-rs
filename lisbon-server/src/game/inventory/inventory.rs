//! Mirrors `net.h4bbo.lisbon.game.inventory.Inventory`.
use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicI32, Ordering};

use parking_lot::Mutex;

use crate::dao::mysql::item_dao::ItemDao;
use crate::game::entity::entity::Entity;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::item::Item;
use crate::game::player::player::Player;
use crate::game::room::entities::room_entity::RoomEntity;
use crate::messages::outgoing::inventory::inventory::INVENTORY;
use crate::server::netty::streams::NettyResponse;
use crate::util::string_util::StringUtil;

/// Mirrors `MAX_ITEMS_PER_PAGE`.
const MAX_ITEMS_PER_PAGE: i32 = 9;

pub struct Inventory {
    items: Mutex<Vec<Item>>,
    paginated_items: Mutex<HashMap<i32, Vec<Item>>>,
    hand_strip_page_index: AtomicI32,
}

impl Default for Inventory {
    /// Mirrors the `Inventory(Player)` constructor (the Java `reload`
    // step runs from `Player::login`, where the `&Player` is available).
    fn default() -> Self {
        Self {
            items: Mutex::new(Vec::new()),
            paginated_items: Mutex::new(HashMap::new()),
            hand_strip_page_index: AtomicI32::new(0),
        }
    }
}

impl Inventory {
    /// Reload inventory. The `user_id` / `room_entity` arguments stand
    /// in for the Java `this.player` field access.
    pub fn reload(&self, user_id: i32, room_entity: Option<&RoomEntity>) {
        self.hand_strip_page_index.store(0, Ordering::SeqCst);
        *self.items.lock() = ItemDao::get_inventory(user_id);

        // Prevent possible virtual item dupes.
        if let Some(room_entity) = room_entity {
            if let Some(room) = room_entity.get_room() {
                let room_items = room.get_items();
                self.items.lock().retain(|item| {
                    !room_items
                        .iter()
                        .any(|room_item| room_item.get_id() == item.get_id())
                });
            }
        }

        self.refresh_pagination(room_entity);
    }

    /// Refreshes the pagination by making the most recently bought items
    /// appear first.
    fn refresh_pagination(&self, room_entity: Option<&RoomEntity>) {
        let mut order_id = 0;

        for item in self.items.lock().iter_mut() {
            if order_id != item.get_order_id() {
                item.set_order_id(order_id);
                item.save();
            }

            order_id += 1;
        }

        let mut temp_list = Vec::new();

        for item in self.items.lock().iter() {
            // Don't show items if they are hidden.
            if item.is_hidden() {
                continue;
            }

            // Don't show items if they are currently in the trade window.
            if let Some(room_entity) = room_entity {
                if room_entity.get_trade_partner().is_some()
                    && room_entity
                        .get_trade_items()
                        .iter()
                        .any(|trade_item| trade_item.get_id() == item.get_id())
                {
                    continue;
                }
            }

            temp_list.push(item.clone());
        }

        temp_list.sort_by_key(|item| item.get_order_id());
        *self.paginated_items.lock() = Self::paginate(&temp_list);
    }

    /// Mirrors `paginate` (the `StringUtil` equivalent).
    fn paginate(items: &[Item]) -> HashMap<i32, Vec<Item>> {
        let chunks = StringUtil::paginate(items, MAX_ITEMS_PER_PAGE as usize);

        chunks
            .into_iter()
            .map(|(page, items)| (page as i32, items))
            .collect()
    }

    /// Mirrors `getView(String)`.
    pub fn view(&self, player: &Player, strip_view: &str) {
        let room_entity = player.get_room_user();
        self.refresh_pagination(room_entity);
        self.change_view(strip_view);

        let casts = self.get_casts();
        player.send(&INVENTORY::new(self.clone(), casts));
    }

    /// Mirrors `getCasts()`.
    fn get_casts(&self) -> HashMap<i32, Item> {
        let mut casts = HashMap::new();
        let paginated = self.paginated_items.lock();
        let page_index = self.hand_strip_page_index.load(Ordering::SeqCst) as usize;

        if let Some(page) = paginated.get(&(page_index as i32)) {
            let mut strip_slot_id = page_index as i32 * MAX_ITEMS_PER_PAGE;

            for item in page {
                casts.insert(strip_slot_id, item.clone());
                strip_slot_id += 1;
            }
        }

        casts
    }

    /// Mirrors `changeView(String)`.
    fn change_view(&self, strip_view: &str) {
        let page_count = self.paginated_items.lock().len() as i32;

        if strip_view == "new" {
            self.hand_strip_page_index.store(0, Ordering::SeqCst);
        } else if strip_view == "next" {
            self.hand_strip_page_index.fetch_add(1, Ordering::SeqCst);
        } else if strip_view == "prev" {
            self.hand_strip_page_index.fetch_sub(1, Ordering::SeqCst);
        } else if strip_view == "last" {
            self.hand_strip_page_index.store(page_count - 1, Ordering::SeqCst);
        } else if strip_view == "update" {
            if self.hand_strip_page_index.load(Ordering::SeqCst) > page_count - 1 {
                self.hand_strip_page_index.store(page_count - 1, Ordering::SeqCst);
            }
        }

        let page_index = self.hand_strip_page_index.load(Ordering::SeqCst);
        if !self.paginated_items.lock().contains_key(&page_index) {
            self.hand_strip_page_index.store(0, Ordering::SeqCst);
        }
    }

    /// Mirrors the static `serialise(NettyResponse, Item, int)`.
    pub fn serialise(response: &mut NettyResponse, item: &Item, strip_slot_id: i32) {
        response.write_delimeter("SI", '\u{001e}');
        response.write_delimeter(item.get_id(), '\u{001e}');
        response.write_delimeter(strip_slot_id, '\u{001e}');

        if item.has_behaviour(ItemBehaviour::WallItem) {
            response.write_delimeter("I", '\u{001e}');
        } else {
            response.write_delimeter("S", '\u{001e}');
        }

        response.write_delimeter(item.get_id(), '\u{001e}');
        response.write_delimeter(item.get_definition().get_sprite(), '\u{001e}');

        if item.has_behaviour(ItemBehaviour::WallItem) {
            response.write_delimeter(item.get_custom_data(), '\u{001e}');
            response.write_delimeter("0", '\u{001e}');
        } else {
            response.write_delimeter(item.get_definition().get_length(), '\u{001e}');
            response.write_delimeter(item.get_definition().get_width(), '\u{001e}');
            response.write_delimeter(item.get_custom_data(), '\u{001e}');
            response.write_delimeter(item.get_definition().get_colour(), '\u{001e}');
            // The Java writes `isRecyclable() ? 1 : 1`.
            response.write_delimeter(1, '\u{001e}');
            response.write_delimeter(item.get_definition().get_sprite(), '\u{001e}');
        }

        response.write("/");
    }

    /// Mirrors `getItem(int)`.
    pub fn get_item(&self, item_id: i32) -> Option<Item> {
        self.items
            .lock()
            .iter()
            .find(|item| item.get_id() == item_id)
            .cloned()
    }

    /// Mirrors `getSoundsets()`.
    pub fn get_soundsets(&self) -> Vec<i32> {
        let mut hand_soundsets = Vec::new();

        for item in self.items.lock().iter() {
            if item.is_hidden() {
                continue;
            }

            if item.has_behaviour(ItemBehaviour::SoundMachineSampleSet) {
                // The Java throws on a malformed sprite.
                if let Some(part) = item.get_definition().get_sprite().split('_').nth(2) {
                    if let Ok(id) = part.parse() {
                        hand_soundsets.push(id);
                    }
                }
            }
        }

        hand_soundsets
    }

    /// Add the item to the start of items list.
    pub fn add_item(&self, item: &Item) {
        let mut items = self.items.lock();
        items.retain(|existing| existing.get_id() != item.get_id());
        items.insert(0, item.clone());
    }

    /// Get the list of inventory items.
    pub fn get_items(&self) -> Vec<Item> {
        self.items.lock().clone()
    }

    /// Mirrors `getItems().remove(item)` (the Java `List.remove(Object)`
    /// match, by item id).
    pub fn remove_item(&self, item: &Item) {
        self.items
            .lock()
            .retain(|existing| existing.get_id() != item.get_id());
    }
}

impl Clone for Inventory {
    fn clone(&self) -> Self {
        Self {
            items: Mutex::new(self.items.lock().clone()),
            paginated_items: Mutex::new(self.paginated_items.lock().clone()),
            hand_strip_page_index: AtomicI32::new(
                self.hand_strip_page_index.load(Ordering::SeqCst),
            ),
        }
    }
}

impl fmt::Debug for Inventory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Inventory").finish()
    }
}
