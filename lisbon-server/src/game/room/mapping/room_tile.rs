//! Mirrors `net.h4bbo.lisbon.game.room.mapping.RoomTile`.
//!
//! Java holds a back-reference to its `Room`; the methods that need the
//! room here take it as an explicit parameter. The Java `highestItem`
//! reference is modelled as an id into the tile's item map (no
//! self-referential structs in Rust).

use std::collections::HashMap;

use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::infobus::infobus_manager::InfobusManager;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::interactors::interaction_type::InteractionType;
use crate::game::item::item::Item;
use crate::game::pathfinder::pathfinder::Pathfinder;
use crate::game::pathfinder::position::Position;
use crate::game::room::enums::status_type::StatusType;
use crate::game::room::handlers::walkways::walkways_manager::WalkwaysManager;
use crate::game::room::room::Room;

pub struct RoomTile {
    position: Position,
    entities: Vec<Box<dyn Entity + Send>>,
    non_blocking_entities: Vec<Box<dyn Entity + Send>>,
    items: HashMap<i32, Box<Item>>,
    tile_height: f64,
    default_height: f64,
    highest_item_id: Option<i32>,
}

impl RoomTile {
    /// Mirrors `ignorePoolTiles`.
    pub const IGNORE_POOL_TILES: [Position; 18] = [
        Position::const_new(20, 28, 0.0),
        Position::const_new(19, 28, 0.0),
        Position::const_new(17, 21, 0.0),
        Position::const_new(17, 20, 0.0),
        Position::const_new(31, 10, 0.0),
        Position::const_new(31, 9, 0.0),
        Position::const_new(19, 19, 0.0),
        Position::const_new(18, 19, 0.0),
        Position::const_new(11, 11, 0.0),
        Position::const_new(10, 11, 0.0),
        Position::const_new(21, 28, 0.0),
        Position::const_new(22, 28, 0.0),
        Position::const_new(16, 22, 0.0),
        Position::const_new(16, 23, 0.0),
        Position::const_new(30, 11, 0.0),
        Position::const_new(30, 12, 0.0),
        Position::const_new(12, 11, 0.0),
        Position::const_new(13, 12, 0.0),
    ];

    /// Mirrors the `RoomTile(Room, Position, double)` constructor (the
    /// back-reference is omitted, see module note).
    pub fn new(position: Position, tile_height: f64) -> Self {
        Self {
            position,
            entities: Vec::new(),
            non_blocking_entities: Vec::new(),
            items: HashMap::new(),
            tile_height,
            default_height: tile_height,
            highest_item_id: None,
        }
    }

    /// Gets if the tile is valid.
    pub fn is_valid_tile(
        room: &Room,
        entity: Option<&(dyn Entity + Send)>,
        position: &Position,
    ) -> bool {
        let mapping = room.get_mapping();
        let mapping = mapping.lock();
        let Some(tile) = mapping.get_tile(room, position.get_x(), position.get_y()) else {
            return false;
        };

        if let Some(entity) = entity {
            if let Some(model) = room.get_model() {
                if model.get_name() == "park_a"
                    && !InfobusManager::get_instance().is_door_open()
                {
                    if *position
                        == Position::new_xy(
                            InfobusManager::get_instance().get_door_x(),
                            InfobusManager::get_instance().get_door_y(),
                        )
                    {
                        return false;
                    }
                }
            }

            if let Some(item) = tile.get_highest_item() {
                if item.get_definition().get_sprite() == "poolExit"
                    && *item.get_position() == Position::new_xy(19, 19)
                {
                    return entity
                        .get_room_user()
                        .map(|room_user| room_user.contains_status(StatusType::Swim))
                        .unwrap_or(false);
                }

                // Allow pets to walk to their own pet bed.
                if entity.get_type() == EntityType::Pet {
                    if let Some(pet) = entity.as_pet() {
                        if pet.get_pet_details().get_item_id() == item.get_id() {
                            return true;
                        }
                    }
                }
            }
        }

        if !tile.get_entities().is_empty() {
            // Allow walk if you exist already in the tile.
            if let Some(highest_item) = tile.get_highest_item() {
                if highest_item.has_behaviour(ItemBehaviour::Teleporter) {
                    return true;
                }
            }

            match entity {
                Some(entity) => tile.contains_entity(entity),
                None => true,
            }
        } else if !tile.has_walkable_furni(entity) {
            if let Some(entity) = entity {
                if let Some(room_user) = entity.get_room_user() {
                    let entity_position = room_user.get_position();
                    return tile.get_highest_item().is_some_and(|highest_item| {
                        *highest_item.get_position() == entity_position
                    });
                }
            }

            false
        } else {
            true
        }
    }

    /// Gets if the tile is valid, but if there's chairs in the way it
    /// will not be valid.
    pub fn is_valid_diagonal_tile(
        room: &Room,
        entity: Option<&(dyn Entity + Send)>,
        position: &Position,
    ) -> bool {
        let mapping = room.get_mapping();
        let mapping = mapping.lock();
        let Some(tile) = mapping.get_tile(room, position.get_x(), position.get_y()) else {
            return false;
        };

        if !tile.get_entities().is_empty() {
            // Allow walk if you exist already in the tile.
            return match entity {
                Some(entity) => tile.contains_entity(entity),
                None => true,
            };
        }

        if let Some(highest_item) = tile.get_highest_item() {
            if highest_item.has_behaviour(ItemBehaviour::CanSitOnTop) {
                return false;
            }

            if !highest_item.is_walkable(entity) {
                return false;
            }
        }

        true
    }

    /// Get if the highest item has walkable furni, true if no furni is
    /// on the tile.
    pub fn has_walkable_furni(&self, entity: Option<&(dyn Entity + Send)>) -> bool {
        if let Some(highest_item) = self.get_highest_item() {
            return highest_item.is_walkable(entity);
        }

        true
    }

    /// Get the next available tile around this tile.
    pub fn get_next_available_position(
        &self,
        room: &Room,
        entity: Option<&(dyn Entity + Send)>,
    ) -> Option<Position> {
        let mut positions = Vec::new();

        for point in Pathfinder::DIAGONAL_MOVE_POINTS.iter() {
            let tmp = self.position.copy().add(point);

            if Self::is_valid_tile(room, entity, &tmp) {
                positions.push(tmp);
            }
        }

        // Java sorts with `Position::getDistanceSquared` as the comparator
        // (i.e. `a.getDistanceSquared(b)`).
        positions.sort_by(|a, b| a.get_distance_squared(b).cmp(&b.get_distance_squared(a)));

        positions.into_iter().next()
    }

    /// Sets the entity.
    pub fn add_entity(&mut self, room: &Room, entity: Box<dyn Entity + Send>) {
        // Don't add a user to the tile in a doorway.
        let current_position = Position::new_xy(self.position.get_x(), self.position.get_y());

        let model = room.get_model();

        if let Some(model) = model {
            if current_position == model.get_door_location() {
                return;
            }
        }

        // If the position is a destination in a walkway, don't add a user
        // to the tile.
        if room.is_public_room() {
            if WalkwaysManager::get_instance().get_destination(room, &self.position).is_some() {
                return;
            }

            let model_name = model.map(|model| model.get_name());

            if matches!(model_name, Some("pool_a") | Some("pool_b") | Some("md_a")) {
                for pos in Self::IGNORE_POOL_TILES.iter() {
                    if pos == &self.position {
                        return;
                    }
                }
            }

            if let Some(highest_item) = self.get_highest_item() {
                if highest_item.get_definition().get_interaction_type() == Some(InteractionType::WsJoinQueue) {
                    return;
                }
            }
        }

        if !room.is_game_arena() {
            if let Some(model) = model {
                let square_in_front = model.get_door_location().get_square_in_front();

                if current_position == square_in_front
                    || current_position == square_in_front.get_square_left()
                    || current_position == square_in_front.get_square_right()
                {
                    self.non_blocking_entities.push(entity);
                    return;
                }
            }
        }

        self.entities.push(entity);
    }

    /// Contains the entity (identity comparison, like Java's
    /// `Object`-based list containment).
    pub fn contains_entity(&self, entity: &(dyn Entity + Send)) -> bool {
        if self.entities.iter().any(|e| std::ptr::eq(Box::as_ref(e), entity)) {
            return true;
        }

        self.non_blocking_entities
            .iter()
            .any(|e| std::ptr::eq(Box::as_ref(e), entity))
    }

    /// Mirrors `getOtherEntities` (Java returns a copied list of
    /// references; trait objects can't be cloned here, so borrowed
    /// references are returned instead).
    pub fn get_other_entities(&self, entity: &(dyn Entity + Send)) -> Vec<&Box<dyn Entity + Send>> {
        let instance_id = entity
            .get_room_user()
            .map(|room_user| room_user.get_instance_id())
            .unwrap_or(-1);

        self.entities
            .iter()
            .filter(|e| {
                e.get_room_user()
                    .map(|room_user| room_user.get_instance_id())
                    .unwrap_or(-1)
                    != instance_id
            })
            .collect()
    }

    /// Removes the entity (identity removal).
    pub fn remove_entity(&mut self, entity: &(dyn Entity + Send)) {
        self.entities
            .retain(|e| !std::ptr::eq(Box::as_ref(e), entity));
        self.non_blocking_entities
            .retain(|e| !std::ptr::eq(Box::as_ref(e), entity));
    }

    /// Mirrors `addItem`.
    pub fn add_item(&mut self, item: Box<Item>) {
        let id = item.get_id();
        self.items.insert(id, item);

        if let Some(stored) = self.items.get(&id) {
            if stored.get_total_height() < self.tile_height {
                return;
            }
        }

        self.reset_highest_item();
    }

    /// Mirrors `removeItem`.
    pub fn remove_item(&mut self, item: &Item) {
        let id = item.get_id();
        self.items.remove(&id);

        if self.highest_item_id.is_none() || self.highest_item_id == Some(id) {
            self.reset_highest_item();
        }
    }

    /// Mirrors `resetHighestItem`.
    pub fn reset_highest_item(&mut self) {
        self.highest_item_id = None;
        self.tile_height = self.default_height;

        for item in self.items.values() {
            let height = item.get_total_height();

            if height < self.tile_height {
                continue;
            }

            self.highest_item_id = Some(item.get_id());
            self.tile_height = height;
        }
    }

    /// Mirrors `getPosition`.
    pub fn get_position(&self) -> &Position {
        &self.position
    }

    /// Mirrors `getTileHeight`.
    pub fn get_tile_height(&self) -> f64 {
        self.tile_height
    }

    /// Get the current height of the tile, but take away the offset of
    /// chairs and beds so users can sit on them properly.
    pub fn get_walking_height(&self) -> f64 {
        let mut height = self.tile_height;

        if let Some(highest_item) = self.get_highest_item() {
            if highest_item.has_behaviour(ItemBehaviour::CanSitOnTop)
                || highest_item.has_behaviour(ItemBehaviour::CanLayOnTop)
            {
                height -= highest_item.get_definition().get_positive_top_height();
            }
        }

        height
    }

    /// Is the next tile lower than our current tile in walking height.
    pub fn is_height_drop(&self, other_tile: &RoomTile) -> bool {
        self.get_walking_height() > other_tile.get_walking_height()
    }

    /// Is the next tile higher than our current tile in walking height.
    pub fn is_height_upwards(&self, other_tile: &RoomTile) -> bool {
        self.get_walking_height() < other_tile.get_walking_height()
    }

    /// Get the highest item in this tile.
    pub fn get_highest_item(&self) -> Option<&Item> {
        self.highest_item_id.and_then(|id| self.items.get(&id).map(|item| item.as_ref()))
    }

    /// Set the highest item in this tile (tracked by id).
    pub fn set_highest_item(&mut self, item: &Item) {
        self.highest_item_id = Some(item.get_id());
    }

    /// Mirrors `getEntities`.
    pub fn get_entities(&self) -> &[Box<dyn Entity + Send>] {
        &self.entities
    }

    /// Mirrors `getNonBlockingEntities`.
    pub fn get_non_blocking_entities(&self) -> &[Box<dyn Entity + Send>] {
        &self.non_blocking_entities
    }

    /// Get all entities, including ones on non-blocking tiles, used for
    /// furni interactions (borrowed references, see `get_other_entities`).
    pub fn get_entire_entities(&self) -> Vec<&Box<dyn Entity + Send>> {
        let mut entity_list: Vec<&Box<dyn Entity + Send>> = Vec::new();
        entity_list.extend(self.entities.iter());
        entity_list.extend(self.non_blocking_entities.iter());
        entity_list
    }

    /// Get the list of items on this tile, sorted by height (Java
    /// returns references; this returns borrows).
    pub fn get_items(&self) -> Vec<&Item> {
        let mut items: Vec<&Item> = self.items.values().map(|item| item.as_ref()).collect();
        items.sort_by(|a, b| {
            a.get_position()
                .get_z()
                .partial_cmp(&b.get_position().get_z())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        items
    }

    /// Mirrors the `ItemRollingAnalysis` loop that marks every item on
    /// the front roller's tile as roll-blocked.
    pub fn set_all_items_roll_blocked(&mut self) {
        for item in self.items.values_mut() {
            item.set_current_roll_blocked(true);
        }
    }

    /// Mirrors `getDefaultHeight`.
    pub fn get_default_height(&self) -> f64 {
        self.default_height
    }

    /// Mirrors `getItemsAbove`.
    pub fn get_items_above(&self, item: &Item) -> Vec<&Item> {
        let mut items = self.get_items();
        let id = item.get_id();
        let z = item.get_position().get_z();
        items.retain(|i| i.get_id() != id && i.get_position().get_z() >= z);
        items
    }
}
