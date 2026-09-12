//! Mirrors `net.h4bbo.lisbon.game.item.interactors.types.PetFoodInteractor`.
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::item::item::Item;
use crate::game::pathfinder::position::Position;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::game::room::entities::room_entity::RoomEntity;
use crate::game::triggers::generic_trigger::GenericTrigger;

pub struct PetFoodInteractor {
    #[allow(dead_code)]
    trigger: GenericTrigger,
}

impl PetFoodInteractor {
    /// Mirrors the `PetFoodInteractor()` constructor.
    pub fn new() -> Self {
        Self {
            #[allow(dead_code)]
    trigger: GenericTrigger,
        }
    }

    /// Mirrors `onItemPlaced(Player, Room, Item)`.
    pub fn on_item_placed(&self, _player: &Player, _room: &Room, item: &mut Item) {
        if item.get_custom_data().trim().is_empty() {
            item.set_custom_data("0");
            item.update_status();
            item.save();
        }
    }

    /// Mirrors `onEntityStop(Entity, RoomEntity, Item, boolean)`.
    pub fn on_entity_stop(
        &self,
        entity: &(dyn Entity + Send),
        _room_entity: &RoomEntity,
        item: &Item,
        _is_rotation: bool,
    ) {
        if entity.get_type() != EntityType::Pet {
            return;
        }

        let Some(pet) = entity.as_pet() else {
            return;
        };

        let front = item.get_position().get_square_in_front();

        pet.get_room_user_pet().look(&front, true);

        let mut task_guard = pet.get_room_user_pet().task_guard();
        if let Some(task) = task_guard.task_mut() {
            task.start_eating();
        }
    }

    /// Mirrors `onItemPickup(Player, Room, Item)`.
    pub fn on_item_pickup(&self, _player: &Player, room: &Room, item: &Item) {
        Self::cancel_pets_eating(room, item.get_position());
    }

    /// Mirrors `onItemMoved(Player, Room, Item, boolean, Position, Item, Item)`.
    pub fn on_item_moved(
        &self,
        _player: &Player,
        room: &Room,
        _item: &Item,
        _is_rotation: bool,
        old_position: &Position,
        _item_below: Option<&Item>,
        _item_above: Option<&Item>,
    ) {
        Self::cancel_pets_eating(room, old_position);
    }

    /// Mirrors `cancelPetsEating(Room, Position)`.
    fn cancel_pets_eating(room: &Room, position: &Position) {
        if let Some(tile) = room.get_mapping().lock().get_tile_by_position(room, position) {
            for entity in tile.get_entities().iter() {
                if entity.get_type() == EntityType::Pet {
                    if let Some(pet) = entity.as_pet() {
                        let mut task_guard = pet.get_room_user_pet().task_guard();
                        if let Some(task) = task_guard.task_mut() {
                            task.eating_complete(false);
                        }
                    }
                }
            }
        }
    }
}

impl Default for PetFoodInteractor {
    fn default() -> Self {
        Self::new()
    }
}
