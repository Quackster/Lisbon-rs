//! Mirrors `net.h4bbo.lisbon.game.room.Room`.
//!
//! `data`/`model`/`mapping` and the managers are owned (and `Clone`), so the
//! getters hand out plain references. The small flags are atomics and the
//! collections (`rights`, `votes`) are `Arc`-wrapped so a `Room` clone (as done
//! by the room tasks) shares state, matching the Java single shared `Room`.
use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicI32};
use std::sync::Arc;

use parking_lot::Mutex;

use crate::dao::mysql::pet_dao::PetDao;
use crate::dao::mysql::room_dao::RoomDao;
use crate::dao::mysql::room_vote_dao::RoomVoteDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::game_scheduler::GameScheduler;
use crate::game::navigator::navigator_category::NavigatorCategory;
use crate::game::navigator::navigator_manager::NavigatorManager;
use crate::game::room::enums::status_type::StatusType;
use crate::game::room::managers::room_entity_manager::RoomEntityManager;
use crate::game::room::managers::room_item_manager::RoomItemManager;
use crate::game::room::managers::room_task_manager::RoomTaskManager;
use crate::game::room::mapping::room_mapping::RoomMapping;
use crate::game::room::models::room_model::RoomModel;
use crate::game::room::room_data::RoomData;
use crate::game::room::room_manager::RoomManager;
use crate::messages::outgoing::rooms::moderation::youarecontroller::YOUARECONTROLLER;
use crate::messages::outgoing::rooms::moderation::younotcontroller::YOUNOTCONTROLLER;
use crate::messages::outgoing::rooms::moderation::youarowner::YOUAROWNER;
use crate::messages::outgoing::rooms::room_forward::ROOMFORWARD;
use crate::messages::outgoing::rooms::update_votes::UPDATE_VOTES;
use crate::messages::types::MessageComposer;
use crate::util::config::game_configuration::GameConfiguration;

#[derive(Clone)]
pub struct Room {
    data: RoomData,
    model: Option<RoomModel>,
    mapping: std::sync::Arc<parking_lot::Mutex<RoomMapping>>,
    entity_manager: RoomEntityManager,
    item_manager: RoomItemManager,
    task_manager: Arc<RoomTaskManager>,
    is_active: Arc<AtomicBool>,
    is_game_arena: Arc<AtomicBool>,
    follow_redirect: Arc<AtomicI32>,
    rights: Arc<Mutex<Vec<i32>>>,
    votes: Arc<Mutex<HashMap<i32, i32>>>,
}

impl Room {
    /// Mirrors the `Room()` constructor.
    pub fn new() -> Self {
        Self {
            data: RoomData::new(),
            model: None,
            mapping: std::sync::Arc::new(parking_lot::Mutex::new(RoomMapping::new())),
            entity_manager: RoomEntityManager::new(),
            item_manager: RoomItemManager::new(),
            task_manager: Arc::new(RoomTaskManager::new()),
            is_active: Arc::new(AtomicBool::new(false)),
            is_game_arena: Arc::new(AtomicBool::new(false)),
            follow_redirect: Arc::new(AtomicI32::new(0)),
            rights: Arc::new(Mutex::new(Vec::new())),
            votes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Send a packet to all players.
    pub fn send(&self, composer: &dyn MessageComposer) {
        for player in self.get_entity_manager().get_players() {
            player.lock().send(composer);
        }
    }

    /// Mirrors `send(MessageComposer, List<Player>)`.
    // The Java implementation ignores the player list and sends to all
    // room players.
    pub fn send_to(&self, composer: &dyn MessageComposer, _player_ids: &[i32]) {
        self.send(composer);
    }

    /// Checks if the user id is the owner of the room.
    pub fn is_owner(&self, owner_id: i32) -> bool {
        self.get_data().get_owner_id() == owner_id
    }

    /// Get if the player has rights, with super users included.
    pub fn has_rights(&self, user_id: i32) -> bool {
        self.has_rights_with_super_users(user_id, true)
    }

    /// Get if the player has rights.
    pub fn has_rights_with_super_users(&self, user_id: i32, include_super_users: bool) -> bool {
        if self.is_owner(user_id) {
            return true;
        }

        if include_super_users && self.get_data().allow_super_users() {
            return true;
        }

        if self.rights.lock().contains(&user_id) {
            return true;
        }

        false
    }

    /// Check if a certain user has voted.
    pub fn has_voted(&self, user_id: i32) -> bool {
        self.votes.lock().contains_key(&user_id)
    }

    /// Add a vote to this room.
    pub fn add_vote(&mut self, answer: i32, user_id: i32) {
        let sum = {
            let mut votes = self.votes.lock();
            votes.insert(user_id, answer);
            let mut sum: i32 = votes.values().sum();
            if sum < 0 {
                sum = 0;
            }
            sum
        };

        self.data.set_rating(sum);
        RoomDao::save_rating(self.get_id(), sum);

        for player in self.get_entity_manager().get_players() {
            let player = player.lock();
            let voted = self.has_voted(player.get_details().get_id());

            if voted || self.is_owner(player.get_details().get_id()) {
                player.send(&UPDATE_VOTES::new(self.data.get_rating()));
            }
        }

        RoomVoteDao::vote(user_id, self.data.get_id(), answer);
    }

    /// Send a forward packet to a user.
    pub fn forward(&self, player: &crate::game::player::player::Player, ignore_redirection: bool) {
        let mut room_id = self.get_id();
        let mut is_public = self.is_public_room();

        // If you tried to follow someone in an arena, send them to the lobby.
        if self.get_data().is_game_arena() {
            let Some(model_type) = self.get_data().get_game_lobby() else {
                return;
            };
            let Some(room) = RoomManager::get_instance().get_room_by_model(model_type) else {
                return;
            };
            room_id = room.lock().get_id();
            is_public = true;
        }

        if is_public && !ignore_redirection {
            if let Some(room) = RoomManager::get_instance().get_room_by_id(room_id) {
                let room = room.lock();
                if room.get_data().is_navigator_hide() {
                    room_id = room.get_follow_redirect();
                }
            }
        }

        if is_public {
            room_id += RoomManager::PUBLIC_ROOM_OFFSET;
        }

        player.send(&ROOMFORWARD::new(is_public, room_id));
    }

    /// Refresh the room rights for a user.
    pub fn refresh_rights(&self, player: &crate::game::player::player::Player) {
        let user_id = player.get_details().get_id();

        if self.has_rights(user_id) {
            player.send(&YOUARECONTROLLER);
        } else {
            player.send(&YOUNOTCONTROLLER);
        }

        let mut rights_value = String::new();

        if self.is_owner(user_id) || player.has_fuse(&Fuseright::AnyRoomController) {
            player.send(&YOUAROWNER);
            rights_value = "useradmin".to_string();
        }

        if let Some(room_user) = player.get_room_user() {
            room_user.remove_status(StatusType::FlatControl);

            if self.has_rights(user_id)
                || self.is_owner(user_id)
                || player.has_fuse(&Fuseright::AnyRoomController)
            {
                room_user.set_status(StatusType::FlatControl, &rights_value);
            }

            room_user.set_needs_update(true);
        }
    }

    /// Try to dispose the room; it happens when there are no users in the room.
    pub fn try_dispose(&self) -> bool {
        if !self.get_entity_manager().get_players().is_empty() {
            return false;
        }

        let seconds = if GameConfiguration::get_instance().get_bool("room.dispose.timer.enabled") {
            GameConfiguration::get_instance().get_integer("room.dispose.timer.seconds") as i64
        } else {
            0
        };

        // The deferred cleanup runs on the (currently no-op) scheduler service,
        // operating on a shared snapshot of the room.
        let room = self.clone();
        GameScheduler::get_instance().schedule(move || {
            if !room.get_entity_manager().get_players().is_empty() {
                return;
            }

            room.get_item_manager().reset_item_states();

            for pet in room.get_entity_manager().get_pets() {
                let Some(room_user) = pet.get_room_user() else {
                    continue;
                };
                PetDao::save_coordinates(
                    pet.get_pet_details().get_id(),
                    room_user.get_position().get_x(),
                    room_user.get_position().get_y(),
                    room_user.get_position().get_rotation(),
                );
            }

            room.set_active(false);
            room.get_task_manager().stop_tasks();

            room.get_item_manager().clear_items();
            room.rights.lock().clear();
            room.votes.lock().clear();
            room.get_entity_manager().clear_entities();

            RoomManager::get_instance().remove_room(room.get_data().get_id());
        }, seconds * 1000);

        true
    }

    /// Get the task manager (per room, shared by `Room` clones).
    pub fn get_task_manager(&self) -> &RoomTaskManager {
        &self.task_manager
    }

    /// Get the entity manager for this room.
    pub fn get_entity_manager(&self) -> &RoomEntityManager {
        &self.entity_manager
    }

    /// Get the item manager for this room.
    pub fn get_item_manager(&self) -> &RoomItemManager {
        &self.item_manager
    }

    /// Get the mapping for this room.
    pub fn get_mapping(&self) -> std::sync::Arc<parking_lot::Mutex<RoomMapping>> {
        self.mapping.clone()
    }

    /// Get the room data for this room.
    pub fn get_data(&self) -> &RoomData {
        &self.data
    }

    /// Mutable access to the room data (for `RoomDao` fills).
    pub fn get_data_mut(&mut self) -> &mut RoomData {
        &mut self.data
    }

    /// Get the room model instance.
    pub fn get_model(&self) -> Option<&RoomModel> {
        self.model.as_ref()
    }

    /// Set the room model, overriding the instance.
    pub fn set_room_model(&mut self, room_model: RoomModel) {
        self.model = Some(room_model);
    }

    /// Get the navigator category for this room.
    pub fn get_category(&self) -> Option<NavigatorCategory> {
        NavigatorManager::get_instance().get_category_by_id(self.get_data().get_category_id())
    }

    /// Get the entire list of entities in the room (bots and pets; players are
    // accessed via `get_entity_manager().get_players()`).
    pub fn get_entities(&self) -> Vec<Box<dyn Entity + Send>> {
        self.get_entity_manager().get_entities()
    }

    /// Get the entire list of items in the room.
    pub fn get_items(&self) -> Vec<crate::game::item::item::Item> {
        self.get_item_manager().get_items()
    }

    /// Get a list of user ids with room rights.
    pub fn get_rights(&self) -> Vec<i32> {
        self.rights.lock().clone()
    }

    /// Mirrors `getRights().add(int)` (the Rust rights list is `Arc`-wrapped).
    pub fn add_right(&self, user_id: i32) {
        self.rights.lock().push(user_id);
    }

    /// Mirrors `getRights().remove(int)`.
    pub fn remove_right(&self, user_id: i32) {
        let mut rights = self.rights.lock();
        rights.retain(|id| *id != user_id);
    }

    /// Mirrors `getRights().clear()`.
    pub fn clear_rights(&self) {
        self.rights.lock().clear();
    }

    /// Get a map of votes.
    /// Mirrors `getRights().clear()` / `addAll(...)`.
    pub fn set_rights(&self, rights: Vec<i32>) {
        *self.rights.lock() = rights;
    }

    /// Mirrors `getVotes().clear()` / `putAll(...)`.
    pub fn set_votes(&self, votes: HashMap<i32, i32>) {
        *self.votes.lock() = votes;
    }

    /// Mirrors `getData().setVisitorsNow(int)`.
    pub fn set_visitors_now(&mut self, visitors_now: i32) {
        self.data.set_visitors_now(visitors_now);
    }

    pub fn get_votes(&self) -> HashMap<i32, i32> {
        self.votes.lock().clone()
    }

    /// Get whether the room is a public room or not.
    pub fn is_public_room(&self) -> bool {
        self.get_data().get_owner_id() == 0
    }

    /// Check if this room is for club members only.
    pub fn is_club_only(&self) -> bool {
        if !self.is_public_room() {
            return false;
        }

        // 8 is the Club Only category.
        self.get_category().map_or(false, |category| category.get_id() == 8)
    }

    /// Get the room id of this room.
    pub fn get_id(&self) -> i32 {
        self.get_data().get_id()
    }

    /// Get if the room is active (has players in it).
    pub fn is_active(&self) -> bool {
        self.is_active.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Set if the room is active.
    pub fn set_active(&self, active: bool) {
        self.is_active.store(active, std::sync::atomic::Ordering::Relaxed);
    }

    /// Get the follow redirect room id.
    pub fn get_follow_redirect(&self) -> i32 {
        self.follow_redirect.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Set the follow redirect room id.
    pub fn set_follow_redirect(&self, follow_redirect: i32) {
        self.follow_redirect
            .store(follow_redirect, std::sync::atomic::Ordering::Relaxed);
    }

    /// Get whether this room is a game arena.
    pub fn is_game_arena(&self) -> bool {
        self.is_game_arena
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Set whether this room is a game arena.
    pub fn set_game_arena(&self, game_arena: bool) {
        self.is_game_arena
            .store(game_arena, std::sync::atomic::Ordering::Relaxed);
    }

    /// Add an item to the room.
    pub fn push_item(&self, item: crate::game::item::item::Item) {
        self.get_item_manager().push_item(item);
    }

    /// Remove an item from the room.
    pub fn remove_item(&self, item: &crate::game::item::item::Item) {
        self.get_item_manager().remove_item(item);
    }
}

impl Default for Room {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Room")
            .field("id", &self.get_id())
            .field("is_active", &self.is_active())
            .field("is_game_arena", &self.is_game_arena())
            .finish()
    }
}

impl serde::Serialize for Room {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct RoomView<'a> {
            id: i32,
            name: &'a str,
            owner_id: i32,
            owner_name: &'a str,
            description: &'a str,
            model: &'a str,
            access_type: &'a str,
            visitors_now: i32,
            visitors_max: i32,
            rating: i32,
            group_id: i32,
            category_id: i32,
        }

        let data = self.get_data();

        RoomView {
            id: data.get_id(),
            name: data.get_name(),
            owner_id: data.get_owner_id(),
            owner_name: data.get_owner_name(),
            description: data.get_description(),
            model: data.get_model(),
            access_type: data.get_access_type(),
            visitors_now: data.get_visitors_now(),
            visitors_max: data.get_visitors_max(),
            rating: data.get_rating(),
            group_id: data.get_group_id(),
            category_id: data.get_category_id(),
        }
        .serialize(serializer)
    }
}
