//! Mirrors `net.h4bbo.lisbon.game.room.managers.RoomItemManager`.
//!
//! The Java `room` back-reference is omitted; the item collection is owned here
//! and the `room` is passed to the methods that need it.
use parking_lot::Mutex;
use std::sync::Arc;

use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::item::Item;
use crate::game::room::mapping::room_mapping::RoomMapping;

#[derive(Clone)]
pub struct RoomItemManager {
    items: Arc<Mutex<Vec<Item>>>,
    sound_machine: Arc<Mutex<Option<Item>>>,
    moodlight: Arc<Mutex<Option<Item>>>,
}

impl RoomItemManager {
    /// Mirrors the `RoomItemManager(Room)` constructor (back-reference omitted).
    pub fn new() -> Self {
        Self {
            items: Arc::new(Mutex::new(Vec::new())),
            sound_machine: Arc::new(Mutex::new(None)),
            moodlight: Arc::new(Mutex::new(None)),
        }
    }

    /// Mirrors `getItems`.
    pub fn get_items(&self) -> Vec<Item> {
        self.items.lock().clone()
    }

    /// Mirrors `getFloorItems`.
    pub fn get_floor_items(&self) -> Vec<Item> {
        self.items
            .lock()
            .iter()
            .filter(|item| {
                !item.has_behaviour(ItemBehaviour::PublicSpaceObject)
                    && !item.has_behaviour(ItemBehaviour::WallItem)
            })
            .cloned()
            .collect()
    }

    /// Mirrors `getWallItems`.
    pub fn get_wall_items(&self) -> Vec<Item> {
        self.items
            .lock()
            .iter()
            .filter(|item| {
                item.has_behaviour(ItemBehaviour::WallItem)
                    && !item.has_behaviour(ItemBehaviour::PublicSpaceObject)
            })
            .cloned()
            .collect()
    }

    /// Mirrors `getById`.
    pub fn get_by_id(&self, item_id: i32) -> Option<Box<Item>> {
        self.items
            .lock()
            .iter()
            .find(|item| item.get_id() == item_id)
            .map(|item| Box::new(item.clone()))
    }

    /// Mirrors `getSoundMachine`.
    pub fn get_sound_machine(&self) -> Option<Box<Item>> {
        self.sound_machine.lock().clone().map(Box::new)
    }

    /// Mirrors `setSoundMachine`.
    pub fn set_sound_machine(&self, item: Option<&Item>) {
        *self.sound_machine.lock() = item.map(|item| item.clone());
    }

    /// Mirrors `getMoodlight`.
    pub fn get_moodlight(&self) -> Option<Box<Item>> {
        self.moodlight.lock().clone().map(Box::new)
    }

    /// Mirrors `setMoodlight`.
    pub fn set_moodlight(&self, item: Option<&Item>) {
        *self.moodlight.lock() = item.map(|item| item.clone());
    }

    /// Mirrors `resetItemStates` (the Java `isGameArena` guard needs the room;
    // it is dropped here, the reset runs over the room's own items).
    pub fn reset_item_states(&self) {
        for mut item in self.items.lock().iter_mut() {
            let _ = RoomMapping::reset_extra_data(&mut item, true);
        }
    }

    /// Add an item to the room.
    pub fn push_item(&self, item: Item) {
        self.items.lock().push(item);
    }

    /// Remove an item from the room by id.
    pub fn remove_item(&self, item: &Item) {
        self.items.lock().retain(|i| i.get_id() != item.get_id());
    }

    /// Clear the room's items (used by `Room.try_dispose`).
    pub fn clear_items(&self) {
        self.items.lock().clear();
    }
}

impl Default for RoomItemManager {
    fn default() -> Self {
        Self::new()
    }
}
