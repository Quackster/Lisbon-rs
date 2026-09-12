//! Mirrors `net.h4bbo.lisbon.game.pathfinder.Pathfinder`.
use std::collections::{HashMap, VecDeque};

use crate::game::entity::entity::Entity;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::item::Item;
use crate::game::item::interactors::types::pool_interactor::PoolInteractor;
use crate::game::pathfinder::affected_tile::AffectedTile;
use crate::game::pathfinder::pathfinder_node::PathfinderNode;
use crate::game::pathfinder::position::Position;
use crate::game::room::mapping::room_tile::RoomTile;
use crate::game::room::room::Room;

pub struct Pathfinder;

impl Pathfinder {
    /// Mirrors `MAX_DROP_HEIGHT`.
    pub const MAX_DROP_HEIGHT: f64 = 3.0;
    /// Mirrors `MAX_LIFT_HEIGHT`.
    pub const MAX_LIFT_HEIGHT: f64 = 1.5;

    /// Mirrors `DIAGONAL_MOVE_POINTS`.
    pub const DIAGONAL_MOVE_POINTS: [Position; 8] = [
        Position::const_new(0, -1, 0.0),
        Position::const_new(0, 1, 0.0),
        Position::const_new(1, 0, 0.0),
        Position::const_new(-1, 0, 0.0),
        Position::const_new(1, -1, 0.0),
        Position::const_new(-1, 1, 0.0),
        Position::const_new(1, 1, 0.0),
        Position::const_new(-1, -1, 0.0),
    ];

    /// Mirrors `MOVE_POINTS`.
    pub const MOVE_POINTS: [Position; 4] = [
        Position::const_new(0, -1, 0.0),
        Position::const_new(1, 0, 0.0),
        Position::const_new(0, 1, 0.0),
        Position::const_new(-1, 0, 0.0),
    ];

    /// Mirrors `DIAGONAL_MOVE_POINTS2`.
    pub const DIAGONAL_MOVE_POINTS2: [Position; 8] = [
        Position::const_new(1, 0, 0.0),
        Position::const_new(1, 1, 0.0),
        Position::const_new(0, 1, 0.0),
        Position::const_new(-1, 1, 0.0),
        Position::const_new(-1, 0, 0.0),
        Position::const_new(-1, -1, 0.0),
        Position::const_new(0, -1, 0.0),
        Position::const_new(1, -1, 0.0),
    ];

    /// Mirrors `isValidStep`.
    pub fn is_valid_step(
        room: &Room,
        entity: &(dyn Entity + Send),
        current: &Position,
        tmp: &Position,
        is_final_move: bool,
    ) -> bool {
        let Some(model) = room.get_model() else {
            return false;
        };

        if !RoomTile::is_valid_tile(
            room,
            Some(entity),
            &Position::new_xy(current.get_x(), current.get_y()),
        ) {
            return false;
        }

        if !RoomTile::is_valid_tile(room, Some(entity), tmp) {
            return false;
        }

        let mapping = room.get_mapping();
        let mapping = mapping.lock();
        let (Some(from_tile), Some(to_tile)) = (
            mapping.get_tile(room, current.get_x(), current.get_y()),
            mapping.get_tile(room, tmp.get_x(), tmp.get_y()),
        ) else {
            return false;
        };

        let old_height = from_tile.get_walking_height();
        let new_height = to_tile.get_walking_height();

        let from_item = from_tile.get_highest_item();
        let to_item = to_tile.get_highest_item();

        // boolean hasPool = room.getModel().getName().startsWith("pool_") || room.getModel().getName().equals("md_a");
        // boolean isPrivateRoom =  !room.isPublicRoom();

        let sprite = |item: Option<&Item>| item.map(|i| i.get_definition().get_sprite().to_string());
        let from_sprite = sprite(from_item);
        let to_sprite = sprite(to_item);

        let from_item_height_exempt = from_item.is_some()
            && (from_item.unwrap().has_behaviour(ItemBehaviour::Teleporter)
                || from_sprite.as_deref() == Some("wsJoinQueue")
                || from_sprite.as_deref() == Some("wsQueueTile")
                // No height check when going between pool triggers
                || (from_sprite.as_deref() == Some("poolEnter")
                    && to_sprite.as_deref() == Some("poolExit"))
                || (from_sprite.as_deref() == Some("poolExit")
                    && to_sprite.as_deref() == Some("poolEnter"))
                || from_sprite.as_deref() == Some("poolLift")
                || (from_sprite.as_deref() == Some("queue_tile2")
                    && room.get_data().get_model() == "pool_b"));

        let to_item_height_exempt = to_item.is_some()
            && (to_item.unwrap().has_behaviour(ItemBehaviour::Teleporter)
                || to_sprite.as_deref() == Some("wsJoinQueue")
                || to_sprite.as_deref() == Some("wsQueueTile")
                // No height check when going between pool triggers
                || (to_sprite.as_deref() == Some("poolEnter")
                    && from_sprite.as_deref() == Some("poolExit"))
                || (to_sprite.as_deref() == Some("poolExit")
                    && from_sprite.as_deref() == Some("poolEnter"))
                || to_sprite.as_deref() == Some("poolLift")
                || (to_sprite.as_deref() == Some("queue_tile2")
                    && room.get_data().get_model() == "pool_b"));

        // Pathfinder makes the path from reversed, so we compare the drop reversed (To tile height against From tile height)
        if to_tile.is_height_upwards(&from_tile) && !from_item_height_exempt && !to_item_height_exempt
        {
            if (new_height - old_height).abs() > Self::MAX_LIFT_HEIGHT {
                return false;
            }
        }

        if to_tile.is_height_drop(&from_tile)
            && !from_item_height_exempt
            && !to_item_height_exempt
        {
            if (old_height - new_height).abs() > Self::MAX_DROP_HEIGHT {
                return false;
            }
        }

        if from_tile.is_height_upwards(&to_tile)
            && !from_item_height_exempt
            && !to_item_height_exempt
        {
            if (new_height - old_height).abs() > Self::MAX_LIFT_HEIGHT {
                return false;
            }
        }

        if from_tile.is_height_drop(&to_tile) && !from_item_height_exempt && !to_item_height_exempt {
            if (old_height - new_height).abs() > Self::MAX_DROP_HEIGHT {
                return false;
            }
        }

        if !PoolInteractor::get_tile_status(room, entity, current, tmp, is_final_move) {
            return false;
        }

        // Don't enable diagonal checking for the Sun Terrace
        // Don't allow diagonal for pool triggers
        let can_walk_diagonal = !model.get_name().starts_with("sun_terrace")
            && from_sprite.as_deref() != Some("poolExit")
            && from_sprite.as_deref() != Some("poolEnter")
            && to_sprite.as_deref() != Some("poolExit")
            && to_sprite.as_deref() != Some("poolEnter");

        // Can't walk diagonal between two non-walkable tiles
        if can_walk_diagonal
            && current.get_x() != tmp.get_x()
            && current.get_y() != tmp.get_y()
        {
            let first_valid_tile = RoomTile::is_valid_diagonal_tile(
                room,
                Some(entity),
                &Position::new_xy(tmp.get_x(), current.get_y()),
            );
            let second_valid_tile = RoomTile::is_valid_diagonal_tile(
                room,
                Some(entity),
                &Position::new_xy(current.get_x(), tmp.get_y()),
            );

            if !first_valid_tile && !second_valid_tile {
                return false;
            }
        }

        // Avoid walking into furniture unless it's their last location
        if current != &model.get_door_location() {
            if let Some(to_item) = to_item {
                if is_final_move {
                    // Allow walking if item is walkable or trapped inside
                    let affected = AffectedTile::get_affected_tiles(to_item);
                    let trapped = affected.iter().any(|x| {
                        mapping
                            .get_tile(room, x.get_x(), x.get_y())
                            .map(|tile| tile.contains_entity(entity))
                            .unwrap_or(false)
                    });
                    return to_item.is_walkable(Some(entity)) || trapped;
                } else {
                    return to_item.has_behaviour(ItemBehaviour::CanStandOnTop)
                        || to_item.is_gate_open();
                }
            }
        }

        true
    }

    /// Mirrors `makePath`.
    ///
    /// Port note: the Java `LinkedList` of next-node references is
    /// represented by a next-position map (Rust nodes cannot reference
    /// each other while the search map owns them).
    pub fn make_path(
        entity: &(dyn Entity + Send),
        start: &Position,
        end: &Position,
    ) -> Vec<Position> {
        let mut squares = Vec::new();
        let Some((result, map, next_map)) = Self::make_path_reversed(entity, start, end) else {
            return squares;
        };

        let mut current = Some(result);
        while let Some(key) = current {
            let Some(next) = next_map.get(&key).copied() else {
                break;
            };
            squares.push(map[&next].get_position().copy());
            current = Some(next);
        }

        squares
    }

    /// Mirrors `makePathReversed`.
    fn make_path_reversed(
        entity: &(dyn Entity + Send),
        start: &Position,
        end: &Position,
    ) -> Option<(
        (i32, i32),
        HashMap<(i32, i32), PathfinderNode>,
        HashMap<(i32, i32), (i32, i32)>,
    )> {
        let Some(room) = entity
            .get_room_user()
            .and_then(|ru| ru.get_room())
        else {
            return None;
        };
        let model = room.get_model()?;

        let mut open_list: VecDeque<(i32, i32)> = VecDeque::new();
        let mut map: HashMap<(i32, i32), PathfinderNode> = HashMap::new();
        let mut next_map: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

        let mut current = PathfinderNode::new(start.copy());
        current.set_cost(0);

        map.insert(
            (current.get_position().get_x(), current.get_position().get_y()),
            current,
        );
        open_list.push_back((start.get_x(), start.get_y()));

        let finish = (end.get_x(), end.get_y());

        while let Some(current_key) = open_list.pop_front() {
            let mut current_node = map.get_mut(&current_key).unwrap();
            current_node.set_in_closed(true);
            let current_pos = current_node.get_position().copy();
            let current_cost = current_node.get_cost();
            drop(current_node);

            for point in Self::DIAGONAL_MOVE_POINTS.iter() {
                let tmp = current_pos.add(point);

                let is_final_move = tmp.get_x() == end.get_x() && tmp.get_y() == end.get_y();

                if !Self::is_valid_step(&room, entity, &current_pos, &tmp, is_final_move) {
                    continue;
                }

                let tmp_key = (tmp.get_x(), tmp.get_y());

                if !map.contains_key(&tmp_key) {
                    map.insert(tmp_key, PathfinderNode::new(tmp.copy()));
                }

                let node = map.get_mut(&tmp_key).unwrap();
                if node.is_in_closed() {
                    continue;
                }

                let mut diff = 0;
                if current_pos.get_x() != node.get_position().get_x() {
                    diff += 2; // Reminder: It was 1 up until 29/08/2018
                }

                if current_pos.get_y() != node.get_position().get_y() {
                    diff += 2; // Reminder: It was 1 up until 29/08/2018
                }

                let cost = current_cost
                    + diff
                    + node.get_position().get_distance_squared(end);

                let is_finish = node.get_position().get_x() == finish.0
                    && node.get_position().get_y() == finish.1;
                let is_open = node.is_in_open();
                drop(node);

                if cost < map[&tmp_key].get_cost() {
                    map.get_mut(&tmp_key).unwrap().set_cost(cost);
                    next_map.insert(tmp_key, current_key);
                }

                if !is_open {
                    if is_finish {
                        next_map.insert(tmp_key, current_key);
                        return Some((tmp_key, map, next_map));
                    }
                    map.get_mut(&tmp_key).unwrap().set_in_open(true);
                    open_list.push_back(tmp_key);
                }
            }
        }

        None
    }
}
