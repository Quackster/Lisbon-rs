//! Mirrors `net.h4bbo.lisbon.game.room.tasks.PetTask`.
use rand::Rng;

use crate::game::entity::entity::Entity;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::pets::pet::Pet;
use crate::game::pets::pet_action::PetAction;
use crate::game::pets::pet_manager::PetManager;
use crate::game::pets::pet_type::PetType;
use crate::game::room::room::Room;
use crate::game::room::enums::status_type::StatusType;
use crate::messages::outgoing::rooms::user::chat_message::ChatMessageType;
use crate::util::date_util::DateUtil;
use crate::util::string_util::StringUtil;

/// Mirrors `PetTask` (the Java `extends TickTask` timers are stored
/// directly; the `timeUntilNextTick` field lives on this struct since
/// Rust traits carry no fields).
pub struct PetTask {
    pet: Pet,
    room: Room,
    time_until_next_tick: i64,
    eating_timer: i32,
    drink_timer: i32,
    play_timer: i32,
    interaction_timer: i32,
}

const POSSIBLE_ACTIONS: [PetAction; 4] = [
    PetAction::Walk,
    PetAction::Lay,
    PetAction::Sit,
    PetAction::Play,
];

impl PetTask {
    /// Mirrors the `PetTask(Pet, Room)` constructor.
    pub fn new(pet: Pet, room: &Room) -> Self {
        Self {
            pet,
            room: room.clone(),
            time_until_next_tick: 0,
            eating_timer: 0,
            drink_timer: 0,
            play_timer: 0,
            interaction_timer: 0,
        }
    }

    /// Mirrors `tick()`.
    pub fn tick(&mut self) {
        self.tick_timers();

        if !self.is_tick_runnable() {
            return;
        }

        if !self.can_move() {
            return;
        }

        if self.pet.get_hunger() <= 2 {
            if self.try_eat() {
                return;
            }
        }

        if self.pet.get_thirst() <= 1 {
            if self.try_drink() {
                return;
            }
        }

        let pet_action =
            POSSIBLE_ACTIONS[rand::thread_rng().gen_range(0..(POSSIBLE_ACTIONS.len() - 1))];

        match pet_action {
            PetAction::Walk => {
                self.walk();
            }
            PetAction::Sit => {
                self.sit_for(0);
            }
            PetAction::Play => {
                self.try_play();
            }
            PetAction::Lay => {
                self.lay_for(0);
            }
            PetAction::Talk => {}
        }

        if rand::thread_rng().gen_range(0..25) == 0 {
            self.talk();
        }

        self.set_time_until_next_tick(rand::thread_rng().gen_range(10..26));
    }

    /// Mirrors `walk()`.
    fn walk(&self) {
        let pet: &(dyn Entity + Send) = &self.pet as &(dyn Entity + Send);
        let Some(available_tile) = self
            .room
            .get_mapping()
            .lock()
            .get_random_walkable_bound(&self.room, Some(pet), false)
        else {
            return;
        };

        self.pet
            .get_room_user_pet()
            .walk_to(available_tile.get_x(), available_tile.get_y());
    }

    /// Mirrors `sit()`.
    pub fn sit(&self) {
        self.sit_for(0);
    }

    /// Mirrors `sit(int)`.
    fn sit_for(&self, time: i32) {
        let room_user = self.pet.get_room_user_pet();
        if room_user.is_walking() {
            room_user.stop_walking();
        }

        let mut position = room_user.get_position();
        position.set_rotation(position.get_rotation() / 2 * 2);
        room_user.set_position(position);

        if time > 0 {
            room_user.set_status_timed(
                StatusType::Sit,
                &StringUtil::format(room_user.get_position().get_z()).to_string(),
                time,
                None,
                -1,
                -1,
            );
        } else {
            room_user.set_status(
                StatusType::Sit,
                &StringUtil::format(room_user.get_position().get_z()).to_string(),
            );
        }

        room_user.remove_status(StatusType::Lay);
        room_user.set_needs_update(true);
    }

    /// Mirrors `lay()`.
    pub fn lay(&self) {
        self.lay_for(0);
    }

    /// Mirrors `lay(int)`.
    fn lay_for(&self, time: i32) {
        let room_user = self.pet.get_room_user_pet();
        if room_user.is_walking() {
            room_user.stop_walking();
        }

        let mut position = room_user.get_position();
        position.set_rotation(position.get_rotation() / 2 * 2);
        room_user.set_position(position);

        if time > 0 {
            room_user.set_status_timed(
                StatusType::Lay,
                &StringUtil::format(room_user.get_position().get_z()).to_string(),
                time,
                None,
                -1,
                -1,
            );
        } else {
            room_user.set_status(
                StatusType::Lay,
                &StringUtil::format(room_user.get_position().get_z()).to_string(),
            );
        }

        room_user.remove_status(StatusType::Sit);
        room_user.set_needs_update(true);
    }

    /// Mirrors `tickTimers()`.
    fn tick_timers(&mut self) {
        if self.eating_timer != 0 {
            self.eating_timer -= 1;

            if self.eating_timer == 1 {
                self.eating_complete(true);
            }
        }

        if self.drink_timer != 0 {
            self.drink_timer -= 1;

            if self.drink_timer == 1 {
                self.drinking_complete(true);
            }
        }

        if self.play_timer != 0 {
            self.play_timer -= 1;

            if self.play_timer == 1 {
                self.playing_complete(true);
            }
        }

        if self.interaction_timer != 0 {
            self.interaction_timer -= 1;
        }
    }

    /// Mirrors `tryEat()`.
    fn try_eat(&self) -> bool {
        self.talk();
        let pet_type = PetManager::get_instance().get_type(&self.pet);

        let pet: &(dyn Entity + Send) = &self.pet as &(dyn Entity + Send);
        let food_in_room: Vec<_> = self
            .room
            .get_item_manager()
            .get_floor_items()
            .into_iter()
            .filter(|item| {
                (item.has_behaviour(ItemBehaviour::PetFood)
                    || match pet_type {
                        Some(PetType::Cat) => item.has_behaviour(ItemBehaviour::PetCatFood),
                        Some(PetType::Dog) => item.has_behaviour(ItemBehaviour::PetDogFood),
                        Some(PetType::Croc) => item.has_behaviour(ItemBehaviour::PetCrocFood),
                        None => false,
                    })
                    && !item.get_custom_data().eq_ignore_ascii_case("4")
                    && item
                        .get_tile()
                        .map(|tile| tile.lock().get_other_entities(pet).is_empty())
                        .unwrap_or(false)
            })
            .collect();

        if food_in_room.is_empty() {
            return false;
        }

        let item = &food_in_room[0];

        self.pet
            .get_room_user_pet()
            .walk_to(item.get_position().get_x(), item.get_position().get_y())
    }

    /// Mirrors `startEating()`.
    pub fn start_eating(&mut self) {
        self.pet.get_room_user_pet().set_status(StatusType::Eat, "1");
        self.pet.get_room_user_pet().set_needs_update(true);

        self.eating_timer = 30;
    }

    /// Mirrors `eatingComplete(boolean)`.
    pub fn eating_complete(&mut self, reap_benefits: bool) {
        if reap_benefits {
            self.pet
                .pet_details_mut()
                .set_last_eat(DateUtil::get_current_time_seconds() as i64);
            self.pet.save_details();
        }

        let room_user = self.pet.get_room_user_pet();
        room_user.remove_status(StatusType::Eat);
        room_user.set_status_timed(StatusType::Smile, "1", 30, None, -1, -1);
        room_user.set_needs_update(true);

        if reap_benefits {
            let Some(mut current_item) = room_user.get_current_item() else {
                self.talk();
                return;
            };
            if current_item.has_behaviour(ItemBehaviour::PetFood) {
                if current_item.has_behaviour(ItemBehaviour::PetCatFood)
                    || current_item.has_behaviour(ItemBehaviour::PetDogFood)
                {
                    self.room.get_mapping().lock().remove_item(&self.room, &mut current_item);
                    current_item.delete();
                } else if let Ok(state) = current_item.get_custom_data().parse::<i32>() {
                    let state = state + 1;

                    if state <= 4 {
                        current_item.set_custom_data(&state.to_string());
                        current_item.update_status();
                        current_item.save();
                    }
                }
            }
        }

        self.talk();
    }

    /// Mirrors `tryDrink()`.
    fn try_drink(&self) -> bool {
        self.talk();

        let pet: &(dyn Entity + Send) = &self.pet as &(dyn Entity + Send);
        let drink_in_room: Vec<_> = self
            .room
            .get_item_manager()
            .get_floor_items()
            .into_iter()
            .filter(|item| {
                item.has_behaviour(ItemBehaviour::PetWaterBowl)
                    && !item.get_custom_data().eq_ignore_ascii_case("0")
                    && item
                        .get_tile()
                        .map(|tile| tile.lock().get_other_entities(pet).is_empty())
                        .unwrap_or(false)
            })
            .collect();

        if drink_in_room.is_empty() {
            return false;
        }

        let item = &drink_in_room[0];

        self.pet
            .get_room_user_pet()
            .walk_to(item.get_position().get_x(), item.get_position().get_y())
    }

    /// Mirrors `startDrinking()`.
    pub fn start_drinking(&mut self) {
        self.pet.get_room_user_pet().set_status(StatusType::Eat, "1");
        self.pet.get_room_user_pet().set_needs_update(true);

        self.drink_timer = 30;
    }

    /// Mirrors `drinkingComplete(boolean)`.
    pub fn drinking_complete(&mut self, reap_benefits: bool) {
        if reap_benefits {
            self.pet
                .pet_details_mut()
                .set_last_drink(DateUtil::get_current_time_seconds() as i64);
            self.pet.save_details();
        }

        let room_user = self.pet.get_room_user_pet();
        room_user.remove_status(StatusType::Eat);
        room_user.set_status_timed(StatusType::Smile, "1", 30, None, -1, -1);
        room_user.set_needs_update(true);

        if reap_benefits {
            let Some(mut current_item) = room_user.get_current_item() else {
                self.talk();
                return;
            };
            if current_item.has_behaviour(ItemBehaviour::PetWaterBowl) {
                if let Ok(state) = current_item.get_custom_data().parse::<i32>() {
                    let state = state - 1;

                    if state >= 0 {
                        current_item.set_custom_data(&state.to_string());
                        current_item.update_status();
                        current_item.save();
                    }
                }
            }
        }

        self.talk();
    }

    /// Mirrors `tryPlay()`.
    fn try_play(&self) {
        self.talk();

        let pet: &(dyn Entity + Send) = &self.pet as &(dyn Entity + Send);
        let food_in_room: Vec<_> = self
            .room
            .get_item_manager()
            .get_floor_items()
            .into_iter()
            .filter(|item| {
                item.has_behaviour(ItemBehaviour::PetToy)
                    && item
                        .get_tile()
                        .map(|tile| tile.lock().get_other_entities(pet).is_empty())
                        .unwrap_or(false)
            })
            .collect();

        if food_in_room.is_empty() {
            return;
        }

        let item = &food_in_room[0];

        self.pet
            .get_room_user_pet()
            .walk_to(item.get_position().get_x(), item.get_position().get_y());
    }

    /// Mirrors `startPlay()`.
    pub fn start_play(&mut self) {
        self.pet.get_room_user_pet().set_status(StatusType::Play, "1");
        self.pet.get_room_user_pet().set_needs_update(true);

        self.talk();

        self.play_timer = 15;
    }

    /// Mirrors `playingComplete(boolean)`.
    pub fn playing_complete(&mut self, reap_benefits: bool) {
        if reap_benefits {
            self.pet
                .pet_details_mut()
                .set_last_play_toy(DateUtil::get_current_time_seconds() as i64);
            self.pet.save_details();
        }

        let room_user = self.pet.get_room_user_pet();
        room_user.remove_status(StatusType::Play);
        room_user.set_status_timed(StatusType::Smile, "1", 30, None, -1, -1);
        room_user.set_needs_update(true);

        self.talk();
    }

    /// Mirrors `talk()`.
    pub fn talk(&self) {
        self.pet.get_room_user_pet().talk(
            &PetManager::get_instance()
                .get_random_speech(self.pet.get_pet_details().get_type()),
            ChatMessageType::Chat,
        );
    }

    /// Mirrors `isTickRunnable()`.
    fn is_tick_runnable(&self) -> bool {
        DateUtil::get_current_time_seconds() as i64 > self.time_until_next_tick
    }

    /// Mirrors `canMove()`.
    pub fn can_move(&self) -> bool {
        self.eating_timer == 0
            && self.drink_timer == 0
            && self.play_timer == 0
            && self.interaction_timer == 0
    }

    /// Mirrors `jump(int)`.
    pub fn jump(&mut self, length: i32) {
        let room_user = self.pet.get_room_user_pet();
        if room_user.is_walking() {
            room_user.stop_walking();
        }

        room_user.clear_statuses();
        let mut position = room_user.get_position();
        position.set_rotation(position.get_body_rotation());
        room_user.set_position(position);

        if length > 0 {
            room_user.set_status_timed(
                StatusType::Jump,
                StatusType::Jump.status_code().to_lowercase().as_str(),
                length,
                None,
                -1,
                -1,
            );
            self.interaction_timer = length;
        } else {
            room_user.set_status(
                StatusType::Jump,
                StatusType::Jump.status_code().to_lowercase().as_str(),
            );
        }

        room_user.set_needs_update(true);
    }

    /// Mirrors `playDead(int)`.
    pub fn play_dead(&mut self, length: i32) {
        let room_user = self.pet.get_room_user_pet();
        if room_user.is_walking() {
            room_user.stop_walking();
        }

        room_user.clear_statuses();
        let mut position = room_user.get_position();
        position.set_rotation(position.get_body_rotation());
        room_user.set_position(position);

        if length > 0 {
            room_user.set_status_timed(
                StatusType::Dead,
                StatusType::Dead.status_code().to_lowercase().as_str(),
                length,
                None,
                -1,
                -1,
            );
            self.interaction_timer = length;
        } else {
            room_user.set_status(
                StatusType::Dead,
                StatusType::Dead.status_code().to_lowercase().as_str(),
            );
        }

        room_user.set_needs_update(true);
    }

    /// Mirrors `setInteractionTimer(int)`.
    pub fn set_interaction_timer(&mut self, i: i32) {
        self.interaction_timer = i;
    }

    /// Mirrors `setTimeUntilNextTick(int)`.
    pub fn set_time_until_next_tick(&mut self, secs: i32) {
        self.time_until_next_tick = DateUtil::get_current_time_seconds() as i64 + secs as i64;
    }

    /// Mirrors `getTimeUntilNextTick()`.
    pub fn get_time_until_next_tick(&self) -> i64 {
        self.time_until_next_tick
    }
}
