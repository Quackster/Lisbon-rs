//! Mirrors `net.h4bbo.lisbon.game.room.entities.RoomPet`.
use rand::Rng;

use crate::game::entity::entity::Entity;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::item::Item;
use crate::game::pathfinder::position::Position;
use crate::game::pets::pet::Pet;
use crate::game::pets::pet_manager::PetManager;
use crate::game::pets::pet_type::PetType;
use crate::game::room::entities::room_entity::RoomEntity;
use crate::game::room::room::Room;
use crate::game::room::tasks::pet_task::PetTask;

/// Mirrors `RoomPet` (Java `extends RoomEntity`; Rust has no subtyping,
/// so the base entity is composed).
pub struct RoomPet {
    pub entity: RoomEntity,
    pub pet: Pet,
    #[allow(dead_code)]
    item: Option<Item>,
    task: Option<PetTask>,
}

impl RoomPet {
    /// Mirrors the `RoomPet(Entity)` constructor (the Java `(Pet) entity`
    /// cast is mirrored with `as_pet`).
    pub fn new(entity: &(dyn Entity + Send)) -> Self {
        Self {
            entity: RoomEntity::default(),
            pet: entity
                .as_pet()
                .cloned()
                .expect("entity must be a Pet"),
            item: None,
            task: None,
        }
    }

    /// Mirrors `invokeItem(Position, boolean)`.
    pub fn invoke_item(&self, old_position: &Position, instant_update: bool) {
        self.entity.invoke_item(Some(old_position), instant_update);
    }

    /// Mirrors `tryDrinking()`.
    pub fn try_drinking(&mut self) {
        let Some(room) = self.entity.get_room() else {
            return;
        };

        let bowls_in_room: Vec<Item> = room
            .get_item_manager()
            .get_floor_items()
            .into_iter()
            .filter(|item| {
                item.has_behaviour(ItemBehaviour::PetWaterBowl)
                    && item.get_custom_data().eq_ignore_ascii_case("1")
            })
            .collect();

        if bowls_in_room.is_empty() {
            return;
        }

        // Java `Collections.shuffle` + `get(0)` == a random element.
        let item = &bowls_in_room[rand::thread_rng().gen_range(0..bowls_in_room.len())];

        self.pet
            .get_room_user_pet()
            .walk_to(item.get_position().get_x(), item.get_position().get_y());

        if !self.pet.get_room_user_pet().is_walking() {
            return;
        }

        self.item = Some(item.clone());
    }

    /// Mirrors `tryEating()`.
    pub fn try_eating(&mut self) {
        let Some(room) = self.entity.get_room() else {
            return;
        };

        let pet_type = PetManager::get_instance().get_type(&self.pet);

        let mut food_in_room: Option<Vec<Item>> = None;

        match pet_type {
            Some(PetType::Dog) => {
                food_in_room = Some(
                    room.get_item_manager()
                        .get_floor_items()
                        .into_iter()
                        .filter(|item| {
                            item.has_behaviour(ItemBehaviour::PetFood)
                                || item.has_behaviour(ItemBehaviour::PetDogFood)
                        })
                        .collect(),
                );
            }
            Some(PetType::Cat) => {
                food_in_room = Some(
                    room.get_item_manager()
                        .get_floor_items()
                        .into_iter()
                        .filter(|item| {
                            item.has_behaviour(ItemBehaviour::PetFood)
                                || item.has_behaviour(ItemBehaviour::PetCatFood)
                        })
                        .collect(),
                );
            }
            Some(PetType::Croc) => {
                food_in_room = Some(
                    room.get_item_manager()
                        .get_floor_items()
                        .into_iter()
                        .filter(|item| {
                            item.has_behaviour(ItemBehaviour::PetFood)
                                || item.has_behaviour(ItemBehaviour::PetCrocFood)
                        })
                        .collect(),
                );
            }
            None => {}
        }

        let Some(food) = food_in_room else {
            return;
        };

        if food.is_empty() {
            return;
        }

        // Java `Collections.shuffle` + `get(0)` == a random element.
        let item = &food[rand::thread_rng().gen_range(0..food.len())];

        self.pet
            .get_room_user_pet()
            .walk_to(item.get_position().get_x(), item.get_position().get_y());

        if !self.pet.get_room_user_pet().is_walking() {
            return;
        }

        self.item = Some(item.clone());
    }

    /// Mirrors `stopWalking()` (the Java body beyond `super.stopWalking()`
    /// is commented out in the Java source).
    pub fn stop_walking(&self) {
        self.entity.stop_walking();
    }

    /// Mirrors `getTask()`.
    pub fn get_task(&self) -> Option<&PetTask> {
        self.task.as_ref()
    }

    /// Mirrors `createTask(Room)`.
    pub fn create_task(&mut self, room: &Room) {
        self.task = Some(PetTask::new(self.pet.clone(), room));
    }
}
