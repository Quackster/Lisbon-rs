//! Mirrors `net.h4bbo.lisbon.game.pets.Pet`.
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::pets::pet_details::PetDetails;
use crate::game::pets::pet_stat::PetStat;
use crate::game::player::player_details::PlayerDetails;
use crate::game::room::entities::room_entity::RoomEntity;

#[derive(Clone, Debug)]
pub struct Pet {
    pet_details: PetDetails,
    // Port note: the Java `PetDetails extends PlayerDetails` base is not
    // mirrored; the `Entity` trait demands a `PlayerDetails`.
    details: PlayerDetails,
    // Port note: the Java field is `RoomPet roomUser`; the `Entity` trait
    // demands a `RoomEntity`.
    room_user: RoomEntity,
}

impl Pet {
    /// Mirrors the `Pet(PetDetails)` constructor (the `RoomPet` is a stub).
    pub fn new(pet_details: PetDetails) -> Self {
        Self {
            // The Java `RoomPet` wraps the `Pet`; the `Entity` trait demands a
            // `RoomEntity`, so a default (empty) entity is held.
            room_user: RoomEntity::default(),
            details: PlayerDetails::new(),
            pet_details,
        }
    }

    /// Mirrors the `getDetails()` mutable accessor (Rust-specific helper
    /// for the `saveDetails` flows).
    pub fn pet_details_mut(&mut self) -> &mut PetDetails {
        &mut self.pet_details
    }

    /// Mirrors `saveDetails()`.
    pub fn save_details(&self) {
        crate::dao::mysql::pet_dao::PetDao::save_details(
            self.pet_details.get_id(),
            &self.pet_details,
        );
    }

    /// Mirrors `getDetails()`.
    pub fn get_pet_details(&self) -> &PetDetails {
        &self.pet_details
    }

    /// Mirrors `getRoomUser()`.
    pub fn get_room_user_pet(&self) -> &RoomEntity {
        &self.room_user
    }

    /// Mirrors `getAge()`.
    pub fn get_age(&self) -> i32 {
        let seconds = crate::util::date_util::DateUtil::get_current_time_seconds() as i64;
        ((seconds - self.pet_details.get_born()) / 86_400) as i32
    }

    /// Mirrors `getHunger()`.
    pub fn get_hunger(&self) -> i32 {
        crate::game::pets::pet_manager::PetManager::get_instance()
            .get_pet_stats(self.pet_details.get_last_eat(), PetStat::Hunger)
    }

    /// Mirrors `getThirst()`.
    pub fn get_thirst(&self) -> i32 {
        crate::game::pets::pet_manager::PetManager::get_instance()
            .get_pet_stats(self.pet_details.get_last_drink(), PetStat::Thirst)
    }

    /// Mirrors `getHappiness()`.
    pub fn get_happiness(&self) -> i32 {
        crate::game::pets::pet_manager::PetManager::get_instance()
            .get_pet_stats(self.pet_details.get_last_play_toy(), PetStat::Happiness)
    }
}

impl Entity for Pet {
    fn has_fuse(&self, _permission: &Fuseright) -> bool {
        false
    }

    fn get_details(&self) -> &PlayerDetails {
        &self.details
    }

    fn get_room_user(&self) -> Option<&RoomEntity> {
        Some(&self.room_user)
    }

    fn get_type(&self) -> EntityType {
        EntityType::Pet
    }

    fn dispose(&mut self) {}

    fn as_pet(&self) -> Option<&Pet> {
        Some(self)
    }
}
