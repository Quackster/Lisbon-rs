//! Mirrors `net.h4bbo.lisbon.game.item.interactors.types.PetNestInteractor`.
use crate::dao::mysql::pet_dao::PetDao;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::item::item::Item;
use crate::game::pathfinder::position::Position;
use crate::game::pets::pet::Pet;
use crate::game::pets::pet_details::PetDetails;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::game::triggers::generic_trigger::GenericTrigger;

pub struct PetNestInteractor {
    #[allow(dead_code)]
    trigger: GenericTrigger,
}

impl PetNestInteractor {
    /// Mirrors the `PetNestInteractor()` constructor.
    pub fn new() -> Self {
        Self {
            #[allow(dead_code)]
    trigger: GenericTrigger,
        }
    }

    /// Mirrors `onItemPlaced(Player, Room, Item)`.
    pub fn on_item_placed(&self, _player: &Player, room: &Room, item: &Item) {
        if let Some(pet_details) = PetDao::get_pet_details(item.get_id()) {
            let pet = self.add_pet(room, &pet_details, item.get_position());

            let Some(room_user) = pet.get_room_user() else {
                return;
            };

            let position = room_user.get_position();

            PetDao::save_coordinates(
                pet_details.get_id(),
                position.get_x(),
                position.get_y(),
                position.get_rotation(),
            );
        }
    }

    /// Mirrors `onItemPickup(Player, Room, Item)`.
    pub fn on_item_pickup(&self, _player: &Player, room: &Room, item: &Item) {
        let Some(pet_details) = PetDao::get_pet_details(item.get_id()) else {
            return;
        };

        let pet_id = pet_details.get_id();

        let Some(pet) = room.get_entity_manager().get_by_id(pet_id, EntityType::Pet) else {
            return;
        };

        room.get_entity_manager().leave_room(room, pet.as_ref(), false);
    }

    /// Mirrors `addPet(Room, PetDetails, Position)`.
    pub fn add_pet(&self, room: &Room, pet_details: &PetDetails, position: &Position) -> Pet {
        let pet = Pet::new(pet_details.clone());
        let mut position = position.clone();

        if let Some(tile) = room
            .get_mapping().lock()
            .get_tile(room, position.get_x(), position.get_y())
        {
            position.set_z(tile.get_walking_height());
        }

        room.get_entity_manager().enter_room_entity(room, &pet, Some(&position));

        pet.get_room_user_pet()
            .set_task(crate::game::room::tasks::pet_task::PetTask::new(
                pet.clone(),
                room,
            ));

        if let Some(mut tile) = room.get_mapping().lock().get_tile_by_position(room, &position) {
            tile.add_entity(room, Box::new(pet.clone()));
        }

        // Note: the Java periodic `walkTo` schedule (`GameScheduler`) is
        // commented out in the Java source.

        pet
    }
}

impl Default for PetNestInteractor {
    fn default() -> Self {
        Self::new()
    }
}
