//! Mirrors `net.h4bbo.lisbon.game.item.roller.EntityRollingAnalysis`.
use crate::game::entity::entity::Entity;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::item::Item;
use crate::game::item::roller::rolling_analysis::RollingAnalysis;
use crate::game::item::roller::rolling_data::RollingData;
use crate::game::pathfinder::position::Position;
use crate::game::room::room::Room;
use crate::game::room::enums::status_type::StatusType;
use crate::game::room::mapping::room_tile::RoomTile;

pub struct EntityRollingAnalysis;

impl EntityRollingAnalysis {
    /// Mirrors the `EntityRollingAnalysis()` constructor.
    pub fn new() -> Self {
        Self
    }
}

impl RollingAnalysis<Box<dyn Entity + Send>> for EntityRollingAnalysis {
    fn can_roll(
        &self,
        entity: &mut Box<dyn Entity + Send>,
        roller: &Item,
        room: &Room,
    ) -> Option<Position> {
        let entity = entity.as_ref();
        let Some(room_user) = entity.get_room_user() else {
            return None;
        };

        if room_user.is_walking() {
            return None; // Don't roll user if they're walking.
        }

        if room_user.get_position().get_z() < roller.get_position().get_z() {
            return None; // Don't roll user if they're below the roller
        }

        if room_user.get_position() != *roller.get_position() {
            return None; // Don't roll users who aren't on this tile.
        }

        let Some(tile) = room_user.get_tile() else {
            return None;
        };

        if !tile.lock().has_walkable_furni(Some(entity)) {
            return None; // Don't roll user if they are stuck, let them be unstuck...
        }

        let front = roller.get_position().get_square_in_front();

        let mapping = room.get_mapping();
        let mapping = mapping.lock();
        let Some(front_tile) = mapping.get_tile_by_position(room, &front) else {
            return None;
        };

        if !front_tile.has_walkable_furni(Some(entity)) {
            return None;
        }

        // Check all entities in the room
        for e in room.get_entities().iter() {
            let e = e.as_ref();

            if e.get_room_user().map_or(true, |room_user| room_user.get_room().is_none()) {
                continue;
            }

            // Don't roll if an entity is going to walk into the this entity
            if let Some(next_position) = e
                .get_room_user()
                .and_then(|room_user| room_user.get_next_position())
            {
                if next_position == front {
                    return None;
                }
            }

            // Ignore people who are walking
            if e.get_room_user().map_or(false, |room_user| room_user.is_walking()) {
                continue;
            }

            // Don't roll if there's an entity rolling into you
            if let Some(rolling_data) = e
                .get_room_user()
                .and_then(|room_user| room_user.get_rolling_data())
            {
                if rolling_data.get_next_position() == front {
                    return None;
                }
            }

            if let Some(room_user) = e.get_room_user() {
                if room_user.get_position() == front {
                    return None;
                }
            }
        }

        // Check all rolling items in the room
        for floor_item in room.get_item_manager().get_floor_items().iter() {
            if let Some(rolling_data) = floor_item.get_rolling_data() {
                if floor_item.get_position() == roller.get_position() {
                    continue;
                }

                // Don't roll if there's another item that's going to roll into this entity
                if rolling_data.get_next_position() == front {
                    return None;
                }
            }
        }

        let mut next_height = room_user.get_position().get_z();
        let mut subtract_roller_height = true;

        if front_tile.get_highest_item().is_some() {
            let mut front_roller: Option<&Item> = None;

            for front_item in front_tile.get_items().iter() {
                if !front_item.has_behaviour(ItemBehaviour::Roller) {
                    continue;
                }

                front_roller = Some(*front_item);
            }

            if let Some(front_roller) = front_roller {
                subtract_roller_height = false; // Since we know there's a roller, don't subtract the height.

                if front_roller.get_position().get_z() != roller.get_position().get_z() {
                    if (front_roller.get_position().get_z() - roller.get_position().get_z()).abs()
                        > 0.1
                    {
                        return None; // Don't roll if the height of the roller is different by >0.1
                    }
                }

                for front_item in front_tile.get_items().iter() {
                    if front_item.get_position().get_z() < front_roller.get_position().get_z() {
                        continue;
                    }

                    // This is because the ItemRollingAnalysis has setHighestItem in nextTile in doRoll which blocks this
                    if let Some(current_item) = room_user.get_current_item() {
                        if current_item.get_id() == front_item.get_id() {
                            continue;
                        }
                    }

                    if front_item.has_behaviour(ItemBehaviour::Roller) {
                        let front_position = front_roller.get_position().get_square_in_front();

                        // Don't roll an item into the next roller, if the next roller is facing towards the roller
                        // it just rolled from, and the next roller has an item on it.
                        if front_position == room_user.get_position() {
                            if !front_tile.get_items_above(front_roller).is_empty()
                                || !front_tile.get_entities().is_empty()
                            {
                                return None;
                            }
                        }
                    } else {
                        return None;
                    }
                }
            } else {
                if !RoomTile::is_valid_tile(room, Some(entity), front_tile.get_position()) {
                    return None;
                }
            }
        }

        if subtract_roller_height {
            next_height -= roller.get_definition().get_top_height();
        }

        if let Some(current_item) = room_user.get_current_item() {
            if !current_item.has_behaviour(ItemBehaviour::Roller) {
                // If we can roll but our item can't, don't roll!
                let mut current_item = current_item;
                if crate::game::item::roller::item_rolling_analysis::ItemRollingAnalysis::new()
                    .can_roll(&mut current_item, roller, room)
                    .is_none()
                {
                    return None;
                }
            }
        }

        let next_position = Position::new(front.get_x(), front.get_y(), next_height);
        room_user.set_rolling_data(Some(RollingData::new_entity(
            entity,
            roller,
            &room_user.get_position(),
            &next_position,
        )));
        Some(next_position)
    }

    fn do_roll(
        &self,
        entity: &mut Box<dyn Entity + Send>,
        roller: &Item,
        room: &Room,
        from_position: &Position,
        next_position: &mut Position,
    ) {
        let mapping = room.get_mapping();
        let mut mapping = mapping.lock();
        let Some(mut previous_tile) = mapping.get_tile_by_position(room, from_position) else {
            return;
        };
        let Some(next_tile) = mapping.get_tile_by_position(room, next_position) else {
            return;
        };
        let entity = entity.as_ref();
        let Some(room_user) = entity.get_room_user() else {
            return;
        };

        // Temporary fix if the user walks on an item and their height gets put up.
        if let Some(current_item) = room_user.get_current_item() {
            if current_item.has_behaviour(ItemBehaviour::Roller)
                && (room_user.get_position().get_z() - roller.get_position().get_z()).abs() >= 0.1
            {
                if let Some(highest) = next_tile.get_highest_item() {
                    if highest.has_behaviour(ItemBehaviour::Roller) {
                        next_position.set_z(
                            roller.get_position().get_z()
                                + roller.get_definition().get_top_height(),
                        );
                    }
                }
            }
        }

        // The next height but what the client sees.
        let mut display_next_height = next_position.get_z();

        if room_user.is_sitting_on_ground() {
            display_next_height -= 0.5; // Take away sit offset when sitting on ground, because yeah, weird stuff.
        }

        // Fix bounce for sitting on chairs if the chair top height is higher 1.0
        if room_user.contains_status(StatusType::Sit) {
            if let Some(status) = room_user.get_status(StatusType::Sit) {
                if let Ok(sit_height) = status.get_value().parse::<f64>() {
                    if sit_height > 1.0 {
                        display_next_height += sit_height - 1.0; // Add new height offset found.
                    }
                }
            }
        }

        if let Some(mut rolling_data) = room_user.get_rolling_data() {
            rolling_data.set_display_height(display_next_height);
        }

        room_user.set_position(next_position.clone());

        previous_tile.remove_entity(entity);

        // Port note: the Java `nextTile.addEntity(entity)` re-add is
        // skipped (the Rust `add_entity` takes an owned
        // `Box<dyn Entity + Send>`, which cannot be moved out of the
        // `&mut Box` the rolling analysis receives).
    }
}

impl Default for EntityRollingAnalysis {
    fn default() -> Self {
        Self::new()
    }
}
