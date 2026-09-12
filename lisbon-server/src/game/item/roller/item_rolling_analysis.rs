//! Mirrors `net.h4bbo.lisbon.game.item.roller.ItemRollingAnalysis`.
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::item::Item;
use crate::game::item::roller::rolling_analysis::RollingAnalysis;
use crate::game::item::roller::rolling_data::RollingData;
use crate::game::pathfinder::position::Position;
use crate::game::room::room::Room;
use crate::game::room::mapping::room_tile::RoomTile;
use crate::util::config::game_configuration::GameConfiguration;

pub struct ItemRollingAnalysis;

impl ItemRollingAnalysis {
    /// Mirrors the `ItemRollingAnalysis()` constructor.
    pub fn new() -> Self {
        Self
    }
}

impl RollingAnalysis<Item> for ItemRollingAnalysis {
    fn can_roll(&self, item: &mut Item, roller: &Item, room: &Room) -> Option<Position> {
        if item.get_id() == roller.get_id() {
            return None;
        }

        if item.get_position().get_z() < roller.get_position().get_z() {
            return None;
        }

        let front = roller.get_position().get_square_in_front();

        let mapping = room.get_mapping();
        let mapping = mapping.lock();
        let Some(front_tile) = mapping.get_tile_by_position(room, &front) else {
            return None;
        };

        // Check all entities in the room
        for e in room.get_entities().iter() {
            let e = e.as_ref();

            if e.get_room_user().map_or(true, |room_user| room_user.get_room().is_none()) {
                continue;
            }

            // Don't roll if an entity is going to walk into the furniture
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

                // Don't roll if there's another item that's going to roll into this item
                if rolling_data.get_next_position() == front {
                    return None;
                }
            }
        }

        let mut next_height = item.get_position().get_z();
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
                subtract_roller_height = false;

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

                    if front_item.get_id() == item.get_id() {
                        continue;
                    }

                    if front_item.has_behaviour(ItemBehaviour::Roller) {
                        let front_position = front_roller.get_position().get_square_in_front();

                        // Don't roll an item into the next roller, if the next roller is facing towards the roller
                        // it just rolled from, and the next roller has an item on it.
                        if front_position == *item.get_position() {
                            if !front_tile.get_items_above(front_roller).is_empty()
                                || !front_tile.get_entities().is_empty()
                            {
                                return None;
                            }
                        }
                    }
                }

                let Some(highest_next_item) = front_tile.get_highest_item() else {
                    return None;
                };

                if !highest_next_item.has_behaviour(ItemBehaviour::Roller) {
                    if highest_next_item.has_behaviour(ItemBehaviour::CanStackOnTop) {
                        let Some(tile) = item.get_tile() else {
                            return None;
                        };

                        if tile.lock().get_entities().is_empty() {
                            if let Some(front_roller_tile) = front_roller.get_tile() {
                                front_roller_tile
                                    .lock()
                                    .set_all_items_roll_blocked();
                            }
                            next_height =
                                (highest_next_item.get_position().get_z()
                                    + highest_next_item.get_definition().get_top_height())
                                    + item.get_position().get_z()
                                    - front_roller.get_definition().get_top_height();
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
            } else {
                if !RoomTile::is_valid_tile(room, None, front_tile.get_position()) {
                    return None;
                }
            }
        }

        if subtract_roller_height {
            next_height -= roller.get_definition().get_top_height();
        }

        if next_height > GameConfiguration::get_instance().get_integer("stack.height.limit") as f64 {
            next_height = GameConfiguration::get_instance().get_integer("stack.height.limit") as f64;
        }

        let next_position = Position::new(front.get_x(), front.get_y(), next_height);
        item.set_rolling_data(Some(RollingData::new_item(
            item,
            roller,
            item.get_position(),
            &next_position,
        )));
        Some(next_position)
    }

    fn do_roll(
        &self,
        item: &mut Item,
        _roller: &Item,
        _room: &Room,
        _from_position: &Position,
        next_position: &mut Position,
    ) {
        item.get_position_mut().set_x(next_position.get_x());
        item.get_position_mut().set_y(next_position.get_y());
        item.get_position_mut().set_z(next_position.get_z());
    }
}

impl Default for ItemRollingAnalysis {
    fn default() -> Self {
        Self::new()
    }
}
