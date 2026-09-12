//! Mirrors `net.h4bbo.lisbon.game.item.Item`.

use crate::game::entity::entity::Entity;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::base::item_definition::ItemDefinition;
use crate::game::item::roller::rolling_data::RollingData;
use crate::game::pathfinder::affected_tile::AffectedTile;
use crate::game::pathfinder::position::Position;
use crate::game::room::mapping::room_tile::RoomTile;
use crate::game::room::mapping::room_tile_state::RoomTileState;
use crate::game::room::room::Room;
use crate::game::room::room_manager::RoomManager;
use crate::util::config::game_configuration::GameConfiguration;
use crate::messages::outgoing::rooms::items::show_program::SHOWPROGRAM;
use crate::messages::outgoing::rooms::items::stuff_data_update::STUFFDATAUPDATE;

#[derive(Debug)]
pub struct Item {
    id: i32,
    order_id: i32,
    owner_id: i32,
    room_id: i32,
    teleporter_id: i32,
    definition: Option<std::sync::Arc<parking_lot::Mutex<ItemDefinition>>>,

    definition_id: i32,
    item_below: Option<i32>,
    wall_position: String,
    custom_data: String,
    current_program: String,
    current_program_value: String,
    requires_update: bool,
    is_current_roll_blocked: bool,
    rolling_data: Option<RollingData>,
    is_hidden: bool,
    teleport_to: Option<Position>,
    swim_to: Option<Position>,
    position: Position,
}

impl Clone for Item {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            order_id: self.order_id,
            owner_id: self.owner_id,
            room_id: self.room_id,
            teleporter_id: self.teleporter_id,
            definition: self
                .definition
                .as_ref()
                .map(|definition| {
                    std::sync::Arc::new(parking_lot::Mutex::new((*definition.lock()).clone()))
                }),

            definition_id: self.definition_id,
            item_below: self.item_below,
            wall_position: self.wall_position.clone(),
            custom_data: self.custom_data.clone(),
            current_program: self.current_program.clone(),
            current_program_value: self.current_program_value.clone(),
            requires_update: self.requires_update,
            is_current_roll_blocked: self.is_current_roll_blocked,
            rolling_data: self.rolling_data.clone(),
            is_hidden: self.is_hidden,
            teleport_to: self.teleport_to.clone(),
            swim_to: self.swim_to.clone(),
            position: self.position.clone(),
        }
    }
}

impl Item {
    /// Mirrors the `DEFAULT_ROOMDIMMER_CUSTOM_DATA` constant.
    pub const DEFAULT_ROOMDIMMER_CUSTOM_DATA: &'static str = "1,1,1,#000000,255";

    /// Mirrors the `PRESENT_DELIMETER` constant.
    pub const PRESENT_DELIMETER: &'static str = "|";

    /// Mirrors the no-arg constructor.
    pub fn new() -> Self {
        Self {
            id: 0,
            order_id: 0,
            owner_id: 0,
            room_id: 0,
            teleporter_id: 0,
            definition: Some(std::sync::Arc::new(parking_lot::Mutex::new(
                ItemDefinition::new(),
            ))),

            definition_id: 0,
            item_below: None,
            wall_position: String::new(),
            custom_data: String::new(),
            current_program: String::new(),
            current_program_value: String::new(),
            requires_update: false,
            is_current_roll_blocked: false,
            rolling_data: None,
            is_hidden: false,
            teleport_to: None,
            swim_to: None,
            position: Position::default(),
        }
    }

    /// Mirrors `getId`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `setId`.
    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    /// Mirrors `getOrderId`.
    pub fn get_order_id(&self) -> i32 {
        self.order_id
    }

    /// Mirrors `setOrderId`.
    pub fn set_order_id(&mut self, order_id: i32) {
        self.order_id = order_id;
    }

    /// Mirrors `getOwnerId`.
    pub fn get_owner_id(&self) -> i32 {
        self.owner_id
    }

    /// Mirrors `setOwnerId`.
    pub fn set_owner_id(&mut self, owner_id: i32) {
        self.owner_id = owner_id;
    }

    /// Mirrors `getRoomId`.
    pub fn get_room_id(&self) -> i32 {
        self.room_id
    }

    /// Mirrors `setRoomId`.
    pub fn set_room_id(&mut self, room_id: i32) {
        self.room_id = room_id;
    }

    /// Mirrors `getTeleporterId`.
    pub fn get_teleporter_id(&self) -> i32 {
        self.teleporter_id
    }

    /// Mirrors `setTeleporterId`.
    pub fn set_teleporter_id(&mut self, teleporter_id: i32) {
        self.teleporter_id = teleporter_id;
    }

    /// Mirrors `getPosition`.
    pub fn get_position(&self) -> &Position {
        &self.position
    }

    /// Mirrors `getPosition` (mutable access for the map methods).
    pub fn get_position_mut(&mut self) -> &mut Position {
        &mut self.position
    }

    /// Mirrors `setPosition`.
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    /// Mirrors `getWallPosition`.
    pub fn get_wall_position(&self) -> &str {
        &self.wall_position
    }

    /// Mirrors `setWallPosition`.
    pub fn set_wall_position(&mut self, wall_position: &str) {
        self.wall_position = wall_position.to_string();
    }

    /// Mirrors `hasBehaviour`.
    pub fn has_behaviour(&self, behaviour: ItemBehaviour) -> bool {
        self.get_definition().has_behaviour(behaviour)
    }

    /// Mirrors `isWalkable`.
    pub fn is_walkable(&self, entity: Option<&(dyn Entity + Send)>) -> bool {
        if self.get_definition().get_sprite() == "poolLift" {
            return self.get_current_program_value() == "open";
        }

        if self.get_definition().get_sprite() == "poolBooth" {
            return self.get_current_program_value() == "open";
        }

        if self.has_behaviour(ItemBehaviour::CanSitOnTop) {
            return true;
        }

        if self.has_behaviour(ItemBehaviour::CanLayOnTop) {
            return true;
        }

        if self.has_behaviour(ItemBehaviour::CanStandOnTop) {
            return true;
        }

        if self.has_behaviour(ItemBehaviour::Teleporter) {
            if let Some(entity) = entity {
                if let Some(player) = entity.as_player() {
                    // The Java NPEs when there is no room user.
                    if let Some(room_user) = player.get_room_user() {
                        if room_user.get_authenticate_teleporter_id() == self.id {
                            return true;
                        }

                        if room_user.get_pending_teleporter_id() == self.id {
                            return true;
                        }
                    }
                }
            }

            return self.custom_data.eq_ignore_ascii_case("TRUE") || self.custom_data == "1";
        }

        if self.has_behaviour(ItemBehaviour::Gate) {
            return self.is_gate_open();
        }

        // Allow walking from it if stuck inside.
        if let Some(entity) = entity {
            // The Java NPEs when the room is gone.
            if let Some(room) = self.get_room() {
                // The shared tile handle is taken so the room lock is
                // not held past the `tile.lock()` below.
                let tile = {
                    let room = room.lock();
                    room.get_mapping().lock().get_tile_handle_for(
                        &room,
                        self.position.get_x(),
                        self.position.get_y(),
                    )
                };

                if let Some(tile) = tile {
                    return tile
                        .lock()
                        .get_entities()
                        .iter()
                        .any(|item_entity| std::ptr::eq(
                            item_entity.as_ref() as *const (dyn Entity + Send),
                            entity as *const (dyn Entity + Send),
                        ));
                }
            }
        }

        false
    }

    /// Mirrors `isGateOpen`.
    pub fn is_gate_open(&self) -> bool {
        if self.has_behaviour(ItemBehaviour::Gate) {
            return self.custom_data == "O";
        }

        false
    }

    /// Check if the move is valid before moving an item. Will prevent
    /// long furniture from being on top of rollers, will prevent
    /// placing rollers on top of other rollers. Will prevent items
    /// being placed on closed tile states.
    pub fn is_valid_move(
        &self,
        item: &Item,
        room: &Room,
        entity: Option<&(dyn Entity + Send)>,
        x: i32,
        y: i32,
        rotation: i32,
    ) -> bool {
        // The Java NPEs when the tile is missing.
        let Some(_) = room.get_mapping().lock().get_tile(room, x, y) else {
            return false;
        };

        let rolling_data = item.get_rolling_data();
        let is_rotation = item.get_position().get_rotation() != rotation
            && (Position::new_xy(x, y) == *item.get_position()
                || (rolling_data.is_some()
                    && Position::new_xy(x, y)
                        == rolling_data.unwrap().get_next_position())
                || (rolling_data.is_some()
                    && Position::new_xy(x, y)
                        == rolling_data.unwrap().get_from_position()));

        if is_rotation {
            if item.get_rolling_data().is_some() {
                return false; // Don't allow rotating items when they're rolling
            }

            if item.get_definition().get_width() <= 1
                && item.get_definition().get_width() <= 1
            {
                return true;
            }
        }

        for position in AffectedTile::get_affected_tiles_ext(self, x, y, rotation) {
            // The tile data is copied out so the tile lock is not held
            // across `is_walkable` (which locks the room).
            let (walking_height, entity_count, highest_item, tile_items) = {
                // The Java NPEs when the tile is missing.
                let mapping = room.get_mapping();
                let mapping = mapping.lock();
                let tile = match mapping.get_tile(room, position.get_x(), position.get_y()) {
                    Some(tile) => tile,
                    None => return false,
                };

                (
                    tile.get_walking_height(),
                    tile.get_entities().len(),
                    tile.get_highest_item().cloned(),
                    tile.get_items().into_iter().cloned().collect::<Vec<Item>>(),
                )
            };

            // The Java NPEs when the room has no model.
            if let Some(model) = room.get_model() {
                if model.get_tile_state(position.get_x(), position.get_y())
                    == RoomTileState::Closed
                {
                    return false;
                }
            }

            if walking_height + item.get_definition().get_positive_top_height()
                > GameConfiguration::get_instance().get_integer("stack.height.limit") as f64
            {
                return false;
            }

            if entity_count > 0 && !item.is_walkable(entity) {
                return false;
            }

            if let Some(highest_item) = highest_item {
                if highest_item.get_id() != item.get_id()
                    && !self.can_place_on_top(item, &highest_item)
                {
                    return false;
                }
            }

            for tile_item in tile_items {
                if tile_item.get_id() == item.get_id() {
                    continue;
                }

                if !self.can_place_on_top(item, &tile_item) {
                    return false;
                }

                if tile_item.has_behaviour(ItemBehaviour::Roller) {
                    if self.has_behaviour(ItemBehaviour::Roller) {
                        return false; // Can't place rollers on top of rollers
                    }

                    if (self.get_definition().get_length() > 1
                        || self.get_definition().get_width() > 1)
                        && (self.has_behaviour(ItemBehaviour::CanSitOnTop)
                            || self.has_behaviour(ItemBehaviour::CanLayOnTop))
                    {
                        return false; // Chair or bed is too big to place on rollers.
                    }
                }
            }
        }

        true
    }

    /// Mirrors `canPlaceOnTop`.
    fn can_place_on_top(&self, item: &Item, tile_item: &Item) -> bool {
        // Don't allow putting rollers on top of stackable objects
        if item.has_behaviour(ItemBehaviour::Roller)
            && tile_item.has_behaviour(ItemBehaviour::CanStackOnTop)
            && !tile_item.has_behaviour(ItemBehaviour::PlaceRollerOnTop)
        {
            if tile_item.get_definition().get_top_height() >= 0.1 {
                return false;
            }
        }

        // If the item is rolling, we can place on the square
        if tile_item.get_rolling_data().is_some() {
            return true;
        }

        // Can't place items on solid objects
        if tile_item.has_behaviour(ItemBehaviour::Solid)
            && !tile_item.has_behaviour(ItemBehaviour::CanStackOnTop)
        {
            return false;
        }

        // Can't place gates on solid rollers
        if tile_item.has_behaviour(ItemBehaviour::Roller)
            && item.has_behaviour(ItemBehaviour::Gate)
        {
            return false;
        }

        // Can't place items on sittable items
        if tile_item.has_behaviour(ItemBehaviour::CanSitOnTop) {
            return false;
        }

        // Can't place item on layable items
        if tile_item.has_behaviour(ItemBehaviour::CanLayOnTop) {
            return false;
        }

        true
    }

    /// Mirrors `getTotalHeight`.
    pub fn get_total_height(&self) -> f64 {
        let definition = self.get_definition();

        if definition.get_top_height() < 0.0 {
            self.position.get_z() + crate::game::item::base::item_definition::DEFAULT_TOP_HEIGHT
        } else {
            self.position.get_z() + definition.get_top_height()
        }
    }

    /// Mirrors `serialise(NettyResponse)`.
    pub fn serialise(&self, response: &mut crate::server::netty::streams::NettyResponse) {
        let definition = self.get_definition();

        if definition.has_behaviour(ItemBehaviour::PublicSpaceObject) {
            response.write_delimeter(self.custom_data.as_str(), ' ');
            response.write_string(definition.get_sprite());
            response.write_delimeter(self.position.get_x(), ' ');
            response.write_delimeter(self.position.get_y(), ' ');
            response.write_delimeter(self.position.get_z() as i32, ' ');
            response.write(self.position.get_rotation());

            if self.has_behaviour(ItemBehaviour::ExtraParameter) {
                response.write(" 2");
            }

            response.write('\u{000D}');
        } else if self.has_behaviour(ItemBehaviour::WallItem) {
            response.write_delimeter(self.id, '\u{0009}');
            response.write_delimeter(definition.get_sprite(), '\u{0009}');
            response.write_delimeter(" ", '\u{0009}');
            response.write_delimeter(self.wall_position.as_str(), '\u{0009}');

            if !self.custom_data.is_empty() {
                if self.has_behaviour(ItemBehaviour::PostIt) {
                    // Only show post-it colour.
                    response.write(
                        self.custom_data
                            .chars()
                            .take(6)
                            .collect::<String>()
                            .as_str(),
                    );
                } else {
                    response.write(self.custom_data.as_str());
                }
            }

            response.write('\u{000D}');
        } else {
            response.write_string(self.id);
            response.write_string(definition.get_sprite());
            response.write_int(self.position.get_x());
            response.write_int(self.position.get_y());
            response.write_int(definition.get_length());
            response.write_int(definition.get_width());
            response.write_int(self.position.get_rotation());
            response.write_string(crate::util::string_util::StringUtil::format(
                self.position.get_z(),
            ));
            response.write_string(definition.get_colour());
            response.write_string("");
            response.write_int(if self.has_behaviour(ItemBehaviour::Roller) {
                2
            } else {
                0
            });

            if self.has_behaviour(ItemBehaviour::Present) {
                let present_data: Vec<&str> = self.custom_data.split(Item::PRESENT_DELIMETER).collect();
                if present_data.len() >= 3 {
                    response.write_string(format!("!{}", present_data[2]));
                } else {
                    response.write_string("");
                }
            } else {
                response.write_string(self.custom_data.as_str());
            }
        }
    }

    /// Mirrors `showProgram(String)`.
    pub fn show_program(&mut self, value: Option<&str>) {
        if let Some(value) = value {
            self.current_program_value = value.to_string();
        }

        if let Some(room) = self.get_room() {
            let room = room.lock();
            room.send(&SHOWPROGRAM::new(vec![
                self.current_program.clone(),
                self.current_program_value.clone(),
            ]));
        }
    }

    /// Mirrors `updateStatus`.
    pub fn update_status(&self) {
        if let Some(room) = self.get_room() {
            let room = room.lock();

            if self.has_behaviour(ItemBehaviour::Teleporter)
                || self.has_behaviour(ItemBehaviour::Gate)
            {
                room.get_mapping().lock().regenerate_collision_map(&room);
            }

            room.send(&STUFFDATAUPDATE::new(Box::new(self.clone())));
        }
    }

    /// Mirrors `save`.
    pub fn save(&self) {
        crate::game::game_scheduler::GameScheduler::get_instance().queue_save_item(self);
    }

    /// Mirrors `delete`.
    pub fn delete(&self) {
        crate::game::game_scheduler::GameScheduler::get_instance().queue_delete_item(self.get_id());
    }

    /// Mirrors `getDefinition` (the Java fallback to `ItemManager` is
    /// ported with the full class).
    /// Mirrors `getDefinition` (the Java lazy `ItemManager` fallback is
    /// resolved eagerly in `set_definition_id`; a `None` here mirrors the
    /// Java `null`, so the port panics like the Java NPE).
    pub fn get_definition(&self) -> parking_lot::MutexGuard<'_, ItemDefinition> {
        self.definition
            .as_ref()
            .expect("item definition resolvable through the item manager")
            .lock()
    }

    /// Mirrors `getDefinition` (mutable form; the Java getter hands out
    /// the item's own `ItemDefinition`, which the `Talk` / `Ufos`
    /// commands mutate).
    pub fn get_definition_mut(&mut self) -> parking_lot::MutexGuard<'_, ItemDefinition> {
        self.definition
            .get_or_insert_with(|| {
                std::sync::Arc::new(parking_lot::Mutex::new(ItemDefinition::new()))
            })
            .lock()
    }

    /// Mirrors `setDefinitionId` (the Java lazy `ItemManager` fallback is
    /// resolved eagerly here, since the getter is `&self`).
    pub fn set_definition_id(&mut self, definition_id: i32) {
        self.definition_id = definition_id;
        self.definition = crate::game::item::item_manager::ItemManager::get_instance()
            .get_definition(definition_id)
            .map(|definition| {
                std::sync::Arc::new(parking_lot::Mutex::new(definition))
            });
    }

    /// Mirrors `getDefinitionId`.
    pub fn get_definition_id(&self) -> i32 {
        self.definition_id
    }

    /// Mirrors `getTile` (the shared handle form, matching
    /// `RoomEntity::getTile`).
    pub fn get_tile(&self) -> Option<std::sync::Arc<parking_lot::Mutex<RoomTile>>> {
        let room = self.get_room()?;
        let room = room.lock();

        room.get_mapping()
            .lock()
            .get_tile_handle_for(&room, self.position.get_x(), self.position.get_y())
    }

    /// Mirrors `updateEntities`.
    pub fn update_entities(&self, old_position: Option<&Position>) {
        if self.has_behaviour(ItemBehaviour::WallItem) {
            return;
        }

        let room_arc = match self.get_room() {
            Some(room_arc) => room_arc,
            None => return,
        };
        let room = room_arc.lock();

        let mut tiles_to_update: Vec<Position> = Vec::new();

        if let Some(old_position) = old_position {
            for position in AffectedTile::get_affected_tiles_ext(
                self,
                old_position.get_x(),
                old_position.get_y(),
                old_position.get_rotation(),
            ) {
                if room
                    .get_mapping()
                    .lock()
                    .get_tile(&room, position.get_x(), position.get_y())
                    .is_some()
                {
                    tiles_to_update.push(position);
                }
            }
        }

        for position in AffectedTile::get_affected_tiles(self) {
            if room
                .get_mapping()
                .lock()
                .get_tile(&room, position.get_x(), position.get_y())
                .is_some()
            {
                tiles_to_update.push(position);
            }
        }

        // Reset people teleporting
        if self.has_behaviour(ItemBehaviour::Teleporter) {
            for entity in room.get_entities() {
                if entity.get_type() != crate::game::entity::entity_type::EntityType::Player {
                    continue;
                }

                let Some(room_user) = entity.get_room_user() else {
                    continue;
                };

                if room_user
                    .get_position()
                    .touches(&self.position)
                    || room_user.get_authenticate_teleporter_id() == self.get_id()
                {
                    room_user.set_authenticate_teleporter_id(-1);
                    room_user.set_walking_allowed(true);
                }
            }
        }

        let mapping = room.get_mapping();
        let mapping = mapping.lock();

        for position in tiles_to_update {
            let Some(tile) = mapping.get_tile(&room, position.get_x(), position.get_y()) else {
                continue;
            };

            for entity in tile.get_entities() {
                if let Some(room_user) = entity.get_room_user() {
                    room_user.invoke_item(old_position, true);
                }
            }
        }
    }

    /// Mirrors `getCustomData`.
    pub fn get_custom_data(&self) -> &str {
        &self.custom_data
    }

    /// Mirrors `setCustomData`.
    pub fn set_custom_data(&mut self, custom_data: &str) {
        self.custom_data = custom_data.to_string();
    }

    /// Mirrors `getCurrentProgram`.
    pub fn get_current_program(&self) -> &str {
        &self.current_program
    }

    /// Mirrors `setCurrentProgram`.
    pub fn set_current_program(&mut self, current_program: &str) {
        self.current_program = current_program.to_string();
    }

    /// Mirrors `getCurrentProgramValue`.
    pub fn get_current_program_value(&self) -> &str {
        &self.current_program_value
    }

    /// Mirrors `getRoom`.
    pub fn get_room(&self) -> Option<std::sync::Arc<parking_lot::Mutex<crate::game::room::room::Room>>> {
        RoomManager::get_instance().get_room_by_id(self.room_id)
    }

    /// Mirrors `getItemBelow` (Java stores the reference; the Rust `Item`
    /// keeps an id).
    pub fn get_item_below(&self) -> Option<i32> {
        self.item_below
    }

    /// Mirrors `setItemBelow`.
    pub fn set_item_below(&mut self, item_below: Option<i32>) {
        self.item_below = item_below;
    }

    /// Mirrors `getItemAbove` (Java computes the next item in the tile's
    /// list; the Rust `Item` returns the id).
    pub fn get_item_above(&self) -> Option<i32> {
        let room = self.get_room()?;
        let room = room.lock();

        let tile = room
            .get_mapping()
            .lock()
            .get_tile_handle_for(&room, self.position.get_x(), self.position.get_y())?;
        let tile = tile.lock();

        let items = tile.get_items();
        let position = items.iter().position(|item| item.get_id() == self.get_id())?;

        items.get(position + 1).map(|item| item.get_id())
    }

    /// Mirrors `getRequiresUpdate`.
    pub fn get_requires_update(&self) -> bool {
        self.requires_update
    }

    /// Mirrors `setRequiresUpdate`.
    pub fn set_requires_update(&mut self, requires_update: bool) {
        self.requires_update = requires_update;
    }

    /// Mirrors `isCurrentRollBlocked`.
    pub fn is_current_roll_blocked(&self) -> bool {
        self.is_current_roll_blocked
    }

    /// Mirrors `setCurrentRollBlocked`.
    pub fn set_current_roll_blocked(&mut self, current_roll_blocked: bool) {
        self.is_current_roll_blocked = current_roll_blocked;
    }

    /// Mirrors `getRollingData`.
    pub fn get_rolling_data(&self) -> Option<&RollingData> {
        self.rolling_data.as_ref()
    }

    /// Mirrors `setRollingData`.
    pub fn set_rolling_data(&mut self, rolling_data: Option<RollingData>) {
        self.rolling_data = rolling_data;
    }

    /// Mirrors `isHidden`.
    pub fn is_hidden(&self) -> bool {
        self.is_hidden
    }

    /// Mirrors `setHidden`.
    pub fn set_hidden(&mut self, hidden: bool) {
        self.is_hidden = hidden;
    }

    /// Mirrors `getTeleportTo`.
    pub fn get_teleport_to(&self) -> Option<&Position> {
        self.teleport_to.as_ref()
    }

    /// Mirrors `setTeleportTo`.
    pub fn set_teleport_to(&mut self, teleport_to: Option<Position>) {
        self.teleport_to = teleport_to;
    }

    /// Mirrors `getSwimTo`.
    pub fn get_swim_to(&self) -> Option<&Position> {
        self.swim_to.as_ref()
    }

    /// Mirrors `setSwimTo`.
    pub fn set_swim_to(&mut self, swim_to: Option<Position>) {
        self.swim_to = swim_to;
    }
}

impl Default for Item {
    fn default() -> Self {
        Self::new()
    }
}
