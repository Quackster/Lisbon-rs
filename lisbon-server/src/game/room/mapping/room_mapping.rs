//! Mirrors `net.h4bbo.lisbon.game.room.mapping.RoomMapping`.
//!
//! The Java `room` back-reference is omitted; the room is passed to the
//! methods that need it. Java's 2-D `RoomTile[][]` grid and `tileList`
//! share the same tile objects; here both are `Arc`/`Mutex`-wrapped
//! handles to the same `RoomTile`.

use parking_lot::{Mutex, MutexGuard};
use std::sync::Arc;

use crate::game::entity::entity::Entity;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::interactors::interaction_type::InteractionType;
use crate::game::item::interactors::types::pet_food_interactor::PetFoodInteractor;
use crate::game::item::interactors::types::pet_nest_interactor::PetNestInteractor;
use crate::game::item::interactors::types::pet_toy_interactor::PetToyInteractor;
use crate::game::item::interactors::types::pet_water_bowl_interactor::PetWaterBowlInteractor;
use crate::game::item::item::Item;
use crate::game::pathfinder::affected_tile::AffectedTile;
use crate::game::pathfinder::position::Position;
use crate::game::player::player::Player;
use crate::game::room::mapping::room_tile::RoomTile;
use crate::game::room::mapping::room_tile_state::RoomTileState;
use crate::game::room::models::room_model_manager::RoomModelManager;
use crate::game::room::public_rooms::pool_handler::PoolHandler;
use crate::game::room::room::Room;
use crate::messages::outgoing::rooms::items::move_flooritem::MOVE_FLOORITEM;
use crate::messages::outgoing::rooms::items::place_flooritem::PLACE_FLOORITEM;
use crate::messages::outgoing::rooms::items::place_wallitem::PLACE_WALLITEM;
use crate::messages::outgoing::rooms::items::remove_flooritem::REMOVE_FLOORITEM;
use crate::messages::outgoing::rooms::items::remove_wallitem::REMOVE_WALLITEM;
use crate::util::config::game_configuration::GameConfiguration;

#[derive(Clone)]
pub struct RoomMapping {
    room_model: Option<crate::game::room::models::room_model::RoomModel>,
    room_map: Vec<Vec<Option<Arc<Mutex<RoomTile>>>>>,
    tile_list: Vec<Arc<Mutex<RoomTile>>>,
}

impl RoomMapping {
    /// Mirrors the `TELEPORTER_CLOSE` constant.
    pub const TELEPORTER_CLOSE: &'static str = "FALSE";

    /// Mirrors the `TELEPORTER_OPEN` constant.
    pub const TELEPORTER_OPEN: &'static str = "TRUE";

    /// Mirrors the `FORTUNE_OFF` constant.
    pub const FORTUNE_OFF: i32 = 8;

    /// Mirrors the `FORTUNE_NO_STATE` constant.
    pub const FORTUNE_NO_STATE: i32 = -1;

    /// Mirrors the `RoomMapping(Room)` constructor (the back-reference is
    /// omitted, see module note).
    pub fn new() -> Self {
        Self {
            room_model: None,
            room_map: Vec::new(),
            tile_list: Vec::new(),
        }
    }

    /// Regenerate the entire collision map used for furniture and entity
    /// detection.
    pub fn regenerate_collision_map(&mut self, room: &Room) {
        // Java builds the grid twice (copy-paste duplicate); the second
        // build wins, so the port performs it once.
        self.tile_list.clear();
        self.room_model = match room.get_model() {
            Some(model) => Some(model.clone()),
            None => Some(
                RoomModelManager::get_instance()
                    .get_model(room.get_data().get_model())
                    .unwrap(),
            ),
        };

        let model = self.room_model.as_ref().unwrap();
        let size_x = model.get_map_size_x() as usize;
        let size_y = model.get_map_size_y() as usize;

        let mut new_map: Vec<Vec<Option<Arc<Mutex<RoomTile>>>>> =
            vec![vec![None; size_y]; size_x];

        for x in 0..size_x {
            for y in 0..size_y {
                let room_tile = Arc::new(Mutex::new(RoomTile::new(
                    Position::new_xy(x as i32, y as i32),
                    model.get_tile_height(x as i32, y as i32),
                )));
                self.tile_list.push(room_tile.clone());
                new_map[x][y] = Some(room_tile);
            }
        }
        self.room_map = new_map;

        // The Java code wraps the item/entity passes in try-catch
        // (logging and continuing); the stubbed dependencies panic instead.
        room.get_item_manager().set_sound_machine(None);
        room.get_item_manager().set_moodlight(None);

        if !room.is_game_arena() {
            let mut items: Vec<Item> = room.get_items();
            items.sort_by(|a, b| {
                a.get_position()
                    .get_z()
                    .partial_cmp(&b.get_position().get_z())
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            for item in &items {
                if item.has_behaviour(ItemBehaviour::WallItem) {
                    continue;
                }

                if item.get_tile().is_none() {
                    continue;
                }

                for position in AffectedTile::get_affected_tiles(item) {
                    let Some(mut affected_tile) =
                        self.get_tile(room, position.get_x(), position.get_y())
                    else {
                        continue;
                    };

                    affected_tile.add_item(item.clone().into());

                    if item.has_behaviour(ItemBehaviour::PublicSpaceObject) {
                        PoolHandler::setup_redirections(room, item);
                    }
                }
            }
        }

        for entity in room.get_entities() {
            let Some(room_user) = entity.get_room_user() else {
                continue;
            };
            let position = room_user.get_position();
            if let Some(mut tile) = self.get_tile(room, position.get_x(), position.get_y()) {
                tile.add_entity(room, entity);
            }
        }

        self.refresh_room_items(room);
    }

    /// Mirrors `refreshRoomItems`.
    pub fn refresh_room_items(&self, room: &Room) {
        room.get_item_manager().set_sound_machine(None);
        room.get_item_manager().set_moodlight(None);

        for item in room.get_item_manager().get_floor_items() {
            if item.has_behaviour(ItemBehaviour::WallItem) {
                continue;
            }

            let _ = item.get_tile();

            // Method to set only one jukebox per room.
            if room.get_item_manager().get_sound_machine().is_none() {
                if item.has_behaviour(ItemBehaviour::Jukebox)
                    || item.has_behaviour(ItemBehaviour::SoundMachine)
                {
                    room.get_item_manager().set_sound_machine(Some(&item));
                }
            }
        }

        // Method to set only one moodlight per room.
        for item in room.get_item_manager().get_wall_items() {
            if item.has_behaviour(ItemBehaviour::RoomDimmer) {
                room.get_item_manager().set_moodlight(Some(&item));
                break;
            }
        }
    }

    /// Add a specific item to the room map.
    pub fn add_item(&self, room: &Room, player: &Player, item: &mut Item) {

        item.set_room_id(room.get_id());
        item.set_owner_id(room.get_data().get_owner_id());
        item.set_rolling_data(None);

        if item.has_behaviour(ItemBehaviour::Dice) {
            // For some reason the client expects the HC dice to have a
            // default of 1 while the normal dice a default of 0 (off).
            if item.get_definition().get_sprite() == "hcdice" {
                // Client expects default of 1 for HC dices.
                item.set_custom_data("1");
            } else if item.get_definition().get_sprite() == "edice" {
                // Client expects default of 0 (off) for 'normal'/'oldskool' dices.
                item.set_custom_data("0");
            } else {
                // Handle custom furniture dices (TODO: define behaviour
                // differences between HC dice and 'oldskool' dices).
                item.set_custom_data("1");
            }
        }

        room.push_item(item.clone().into());

        if item.has_behaviour(ItemBehaviour::WallItem) {
            room.send(&PLACE_WALLITEM::new(item.clone().into()));

            if item.has_behaviour(ItemBehaviour::RoomDimmer) {
                room.get_item_manager().set_moodlight(Some(item));
            }
        } else {
            self.handle_item_adjustment(room, item, false);

            for position in AffectedTile::get_affected_tiles(item) {
                if let Some(mut affected_tile) =
                    self.get_tile(room, position.get_x(), position.get_y())
                {
                    affected_tile.add_item(item.clone().into());
                }
            }

            room.send(&PLACE_FLOORITEM::new(item.clone().into()));
        }

        if item.has_behaviour(ItemBehaviour::Teleporter) {
            item.set_custom_data(Self::TELEPORTER_CLOSE);
        }

        item.update_entities(None);
        item.save();

        let has_wall = item.get_definition().has_behaviour(ItemBehaviour::WallItem);
        if !has_wall {
            let interaction_type = item.get_definition().get_interaction_type();
            if let Some(interaction_type) = interaction_type {
                if let Some(box_trigger) = interaction_type.get_trigger() {
                    if let Some(interactor) = box_trigger.downcast_ref::<PetNestInteractor>() {
                        interactor.on_item_placed(player, room, item);
                    } else if let Some(interactor) =
                        box_trigger.downcast_ref::<PetFoodInteractor>()
                    {
                        interactor.on_item_placed(player, room, item);
                    } else if let Some(interactor) =
                        box_trigger.downcast_ref::<PetWaterBowlInteractor>()
                    {
                        interactor.on_item_placed(player, room, item);
                    }
                }
            }
        }

        self.refresh_room_items(room);
    }

    /// Move an item, will regenerate the map if the item is a floor
    /// item.
    pub fn move_item(
        &self,
        room: &Room,
        player: &Player,
        item: &mut Item,
        new_position: &Position,
        old_position: &Position,
    ) {

        let is_rotation =
            *item.get_position() == Position::new_xy(new_position.get_x(), new_position.get_y())
                && item.get_position().get_rotation() != new_position.get_rotation();

        for position in AffectedTile::get_affected_tiles(item) {
            if let Some(mut affected_tile) =
                self.get_tile(room, position.get_x(), position.get_y())
            {
                affected_tile.remove_item(item);
            }
        }

        item.get_position_mut().set_x(new_position.get_x());
        item.get_position_mut().set_y(new_position.get_y());
        item.get_position_mut().set_rotation(new_position.get_rotation());
        item.set_room_id(room.get_id());
        item.set_rolling_data(None);
        Self::reset_extra_data(item, false);

        if !item.has_behaviour(ItemBehaviour::WallItem) {
            self.handle_item_adjustment(room, item, is_rotation);

            for position in AffectedTile::get_affected_tiles(item) {
                if let Some(mut affected_tile) =
                    self.get_tile(room, position.get_x(), position.get_y())
                {
                    affected_tile.add_item(item.clone().into());
                }
            }

            room.send(&MOVE_FLOORITEM::new(item.clone().into()));
        }

        if item.has_behaviour(ItemBehaviour::Teleporter) {
            item.set_custom_data(Self::TELEPORTER_CLOSE);
        }

        item.update_entities(Some(old_position));
        item.save();

        if let Some(interaction_type) = item.get_definition().get_interaction_type() {
            if let Some(box_trigger) = interaction_type.get_trigger() {
                if let Some(interactor) = box_trigger.downcast_ref::<PetFoodInteractor>() {
                    interactor.on_item_moved(
                        player,
                        room,
                        item,
                        is_rotation,
                        old_position,
                        None,
                        None,
                    );
                } else if let Some(interactor) =
                    box_trigger.downcast_ref::<PetWaterBowlInteractor>()
                {
                    interactor.on_item_moved(
                        player,
                        room,
                        item,
                        is_rotation,
                        old_position,
                        None,
                        None,
                    );
                } else if let Some(interactor) = box_trigger.downcast_ref::<PetToyInteractor>() {
                    interactor.on_item_moved(
                        player,
                        room,
                        item,
                        is_rotation,
                        old_position,
                        None,
                        None,
                    );
                }
            }
        }

        self.refresh_room_items(room);
    }

    /// Mirrors `pickupItem`.
    pub fn pickup_item(&self, room: &Room, player: &Player, item: &mut Item) {
        if let Some(interaction_type) = item.get_definition().get_interaction_type() {
            if let Some(box_trigger) = interaction_type.get_trigger() {
                if let Some(interactor) = box_trigger.downcast_ref::<PetNestInteractor>() {
                    interactor.on_item_pickup(player, room, item);
                } else if let Some(interactor) = box_trigger.downcast_ref::<PetFoodInteractor>() {
                    interactor.on_item_pickup(player, room, item);
                } else if let Some(interactor) =
                    box_trigger.downcast_ref::<PetWaterBowlInteractor>()
                {
                    interactor.on_item_pickup(player, room, item);
                } else if let Some(interactor) =
                    box_trigger.downcast_ref::<PetToyInteractor>()
                {
                    interactor.on_item_pickup(player, room, item);
                }
            }
        }

        self.remove_item(room, item);
    }

    /// Remove an item from the room.
    pub fn remove_item(&self, room: &Room, item: &mut Item) {
        room.remove_item(item);

        if item.has_behaviour(ItemBehaviour::WallItem) {
            room.send(&REMOVE_WALLITEM::new(item.clone().into()));
        } else {
            for position in AffectedTile::get_affected_tiles(item) {
                if let Some(mut affected_tile) =
                    self.get_tile(room, position.get_x(), position.get_y())
                {
                    affected_tile.remove_item(item);
                }
            }

            room.send(&REMOVE_FLOORITEM::new(item.clone().into()));
        }

        if item.has_behaviour(ItemBehaviour::RoomDimmer) {
            if item.get_custom_data().is_empty() {
                item.set_custom_data(Item::DEFAULT_ROOMDIMMER_CUSTOM_DATA);
            }

            if item.get_custom_data().chars().next() == Some('2') {
                // Roomdimmer is enabled, turn it off.
                item.set_custom_data(&format!(
                    "1{}",
                    item.get_custom_data().get(1..).unwrap_or("")
                ));
            }

            room.get_item_manager().set_moodlight(None);
        }

        Self::reset_extra_data(item, false);
        item.update_entities(None);

        item.get_position_mut().set_x(0);
        item.get_position_mut().set_y(0);
        item.get_position_mut().set_z(0.0);
        item.get_position_mut().set_rotation(0);
        item.set_room_id(0);
        item.set_rolling_data(None);
        item.save();

        self.refresh_room_items(room);
    }

    /// Mirrors `resetExtraData`.
    pub fn reset_extra_data(item: &mut Item, room_load: bool) -> bool {
        if item.has_behaviour(ItemBehaviour::Dice) {
            item.set_requires_update(false);

            // For some reason the client expects the HC dice to have a
            // default of 1 while the normal dice a default of 0 (off).
            let sprite = item.get_definition().get_sprite().to_string();
            match sprite.as_str() {
                // Client expects default of 1 for HC dices.
                "edicehc" => {
                    if room_load {
                        if item.get_custom_data() != "0" {
                            item.set_custom_data("0");
                            return true;
                        }
                    } else if item.get_custom_data() != "1" {
                        item.set_custom_data("1");
                        return true;
                    }
                }
                // Client expects default of 0 (off) for 'normal'/'oldskool' dices.
                "edice" => {
                    if item.get_custom_data() != "0" {
                        item.set_custom_data("0");
                        return true;
                    }
                }
                _ => {
                    // Handle custom furniture dices (TODO: define behaviour
                    // differences between HC dice and 'oldskool' dices).
                    if item.get_custom_data() != "1" {
                        item.set_custom_data("1");
                        return true;
                    }
                }
            }
        }

        if item.get_definition().get_interaction_type() == Some(InteractionType::Lert) {
            if item.get_custom_data() != "0" {
                item.set_custom_data("0");
                return true;
            }
        }

        if item.get_definition().get_interaction_type() == Some(InteractionType::Fortune) {
            if item.get_custom_data() != Self::FORTUNE_OFF.to_string() {
                item.set_custom_data(&Self::FORTUNE_OFF.to_string());
                return true;
            }

            item.set_requires_update(false);
        }

        if item.has_behaviour(ItemBehaviour::Teleporter) {
            if item.get_custom_data() != Self::TELEPORTER_CLOSE {
                item.set_custom_data(Self::TELEPORTER_CLOSE);
                return true;
            }
        }

        if item.is_current_roll_blocked() {
            item.set_current_roll_blocked(false);
        }

        false
    }

    /// Handle item adjustment.
    fn handle_item_adjustment(&self, room: &Room, item: &mut Item, is_rotation: bool) {
        let tile = match self.get_tile(room, item.get_position().get_x(), item.get_position().get_y()) {
            Some(tile) => tile,
            None => return,
        };

        if !is_rotation {
            let mut tile_height = tile.get_tile_height();
            let highest_item = tile.get_highest_item();

            if let Some(highest) = highest_item {
                if highest.get_id() == item.get_id() {
                    tile_height -= highest.get_total_height();

                    let default_height = room
                        .get_model()
                        .map(|model| {
                            model.get_tile_height(
                                item.get_position().get_x(),
                                item.get_position().get_y(),
                            )
                        })
                        .unwrap_or(0.0);

                    if tile_height < default_height {
                        tile_height = default_height;
                    }
                }
            }

            item.get_position_mut().set_z(tile_height);

            if let Some(highest) = highest_item {
                if let Some(rolling_data) = highest.get_rolling_data() {
                    let roller = rolling_data.get_roller();

                    item.get_position_mut().set_z(
                        roller.get_position().get_z()
                            + roller.get_definition().get_positive_top_height(),
                    );
                }
            }
        }

        let limit = GameConfiguration::get_instance().get_integer("stack.height.limit") as f64;
        if item.get_position().get_z() > limit {
            item.get_position_mut().set_z(limit);
        }
    }

    /// Get the tile by the given position.
    pub fn get_tile_by_position(
        &self,
        room: &Room,
        position: &Position,
    ) -> Option<MutexGuard<'_, RoomTile>> {
        self.get_tile(room, position.get_x(), position.get_y())
    }

    /// Get the tile by the specified coordinates.
    pub fn get_tile(
        &self,
        room: &Room,
        x: i32,
        y: i32,
    ) -> Option<MutexGuard<'_, RoomTile>> {
        if x < 0 || y < 0 {
            return None;
        }

        if let Some(model) = room.get_model() {
            if x >= model.get_map_size_x() || y >= model.get_map_size_y() {
                return None;
            }
        } else if let Some(model) =
            RoomModelManager::get_instance().get_model(room.get_data().get_model())
        {
            if x >= model.get_map_size_x() || y >= model.get_map_size_y() {
                return None;
            }
        }

        if let Some(room_model) = self.room_model.as_ref() {
            if x >= room_model.get_map_size_x() || y >= room_model.get_map_size_y() {
                return None;
            }

            if room_model.get_tile_state(x, y) == RoomTileState::Closed {
                return None;
            }
        }

        self.room_map.get(x as usize)?
            .get(y as usize)?
            .as_ref()
            .map(|tile| tile.lock())
    }

    /// Mirrors direct access to the `roomMap` grid cell (the Java grid is
    /// private; callers that need the tile past the room lock use this
    /// shared handle instead of `getTile`).
    pub fn get_tile_handle(&self, x: i32, y: i32) -> Option<Arc<Mutex<RoomTile>>> {
        self.room_map.get(x as usize)?.get(y as usize)?.clone()
    }

    /// Mirrors `getTile` but returns the shared handle instead of a guard,
    /// so it can be handed across locks (`RoomEntity.getTile`).
    pub fn get_tile_handle_for(
        &self,
        room: &Room,
        x: i32,
        y: i32,
    ) -> Option<Arc<Mutex<RoomTile>>> {
        if x < 0 || y < 0 {
            return None;
        }

        if let Some(model) = room.get_model() {
            if x >= model.get_map_size_x() || y >= model.get_map_size_y() {
                return None;
            }
        } else if let Some(model) =
            RoomModelManager::get_instance().get_model(room.get_data().get_model())
        {
            if x >= model.get_map_size_x() || y >= model.get_map_size_y() {
                return None;
            }
        }

        if let Some(room_model) = self.room_model.as_ref() {
            if x >= room_model.get_map_size_x() || y >= room_model.get_map_size_y() {
                return None;
            }

            if room_model.get_tile_state(x, y) == RoomTileState::Closed {
                return None;
            }
        }

        self.room_map.get(x as usize)?.get(y as usize)?.clone()
    }

    /// Mirrors `getRandomWalkableBound`.
    pub fn get_random_walkable_bound(
        &self,
        room: &Room,
        entity: Option<&(dyn Entity + Send)>,
        allow_door_bound: bool,
    ) -> Option<Position> {
        let mut attempts = 0;
        let max_attempts = 10;

        while attempts < max_attempts {
            attempts += 1;

            let Some(model) = room.get_model() else {
                // Java would NPE on `room.getModel()`.
                return None;
            };
            let position = Position::new_xy(model.get_random_bound(0), model.get_random_bound(1));

            if !allow_door_bound && position == model.get_door_location() {
                continue;
            }

            if RoomTile::is_valid_tile(room, entity, &position) {
                return Some(position);
            }
        }

        None
    }
}

impl Default for RoomMapping {
    fn default() -> Self {
        Self::new()
    }
}
