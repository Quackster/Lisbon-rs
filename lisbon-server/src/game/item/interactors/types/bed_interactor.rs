//! Mirrors `net.h4bbo.lisbon.game.item.interactors.types.BedInteractor`.
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::item::item::Item;
use crate::game::pathfinder::position::Position;
use crate::game::room::entities::room_entity::RoomEntity;
use crate::game::room::enums::status_type::StatusType;
use crate::game::room::mapping::room_tile::RoomTile;
use crate::game::triggers::generic_trigger::GenericTrigger;
use crate::util::string_util::StringUtil;

pub struct BedInteractor {
    #[allow(dead_code)]
    trigger: GenericTrigger,
}

impl BedInteractor {
    /// Mirrors the `BedInteractor()` constructor.
    pub fn new() -> Self {
        Self {
            #[allow(dead_code)]
    trigger: GenericTrigger,
        }
    }

    /// Mirrors `onEntityStop(Entity, RoomEntity, Item, boolean)`.
    pub fn on_entity_stop(
        &self,
        entity: &(dyn Entity + Send),
        room_entity: &RoomEntity,
        item: &Item,
        _is_rotation: bool,
    ) {
        let Some(room_user) = entity.get_room_user() else {
            return;
        };

        let mut destination = room_user.get_position();

        if !Self::is_valid_pillow_tile(item, &destination) {
            destination = Self::convert_to_pillow(&destination, item);
        }

        if Self::is_valid_pillow_tile(item, &destination) {
            let Some(room) = room_entity.get_room() else {
                return;
            };

            if !RoomTile::is_valid_tile(&room, Some(entity), &destination) {
                return;
            }

            room_user.warp(&destination, false, false);

            room_entity.remove_drinks();
            room_entity.remove_status(StatusType::Dance);

            let mut position = room_entity.get_position();
            position.set_rotation(item.get_position().get_rotation());
            room_entity.set_position(position);

            let mut top_height = item.get_definition().get_top_height();

            if entity.get_type() == EntityType::Pet {
                top_height =
                    0.5 + item.get_tile().map_or(0.0, |tile| tile.lock().get_walking_height());
            }

            let mut position = room_entity.get_position();
            position.set_rotation(item.get_position().get_rotation());
            room_entity.set_position(position);
            room_entity.set_status(StatusType::Lay, &format!("{}", StringUtil::format(top_height)));
        }

        room_entity.set_needs_update(true);
    }

    /// Mirrors `convertToPillow(Position, Item)`.
    pub fn convert_to_pillow(position: &Position, item: &Item) -> Position {
        let mut destination = position.clone();

        if !Self::is_valid_pillow_tile(item, position) {
            for tile in Self::get_valid_pillow_tiles(item) {
                if item.get_position().get_rotation() == 0 {
                    destination.set_y(tile.get_y());
                } else {
                    destination.set_x(tile.get_x());
                }

                break;
            }
        }

        destination
    }

    /// Mirrors `isValidPillowTile(Item, Position)`.
    pub fn is_valid_pillow_tile(item: &Item, entity_position: &Position) -> bool {
        if entity_position == item.get_position() {
            return true;
        }

        for valid_tile in Self::get_valid_pillow_tiles(item) {
            if &valid_tile == entity_position {
                return true;
            }
        }

        false
    }

    /// Mirrors `getValidPillowTiles(Item)`.
    pub fn get_valid_pillow_tiles(item: &Item) -> Vec<Position> {
        let mut tiles = Vec::new();
        tiles.push(Position::new_xy(item.get_position().get_x(), item.get_position().get_y()));

        let mut valid_pillow_x = -1;
        let mut valid_pillow_y = -1;

        if item.get_position().get_rotation() == 0 {
            valid_pillow_x = item.get_position().get_x() + 1;
            valid_pillow_y = item.get_position().get_y();
        }

        if item.get_position().get_rotation() == 2 {
            valid_pillow_x = item.get_position().get_x();
            valid_pillow_y = item.get_position().get_y() + 1;
        }

        tiles.push(Position::new_xy(valid_pillow_x, valid_pillow_y));
        tiles
    }
}

impl Default for BedInteractor {
    fn default() -> Self {
        Self::new()
    }
}
