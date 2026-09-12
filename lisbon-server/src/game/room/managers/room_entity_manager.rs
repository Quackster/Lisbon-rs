//! Mirrors `net.h4bbo.lisbon.game.room.managers.RoomEntityManager`.
//!
//! Players are stored as the canonical shared `Arc<Mutex<Player>>` handles;
//! bots and pets are stored as owned values. The generic entity list
//! (`get_entities`) is built from the owned bots and pets; players are reached
//! through `get_players()`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::dao::mysql::item_dao::ItemDao;
use crate::dao::mysql::room_dao::RoomDao;
use crate::dao::mysql::room_rights_dao::RoomRightsDao;
use crate::dao::mysql::room_vote_dao::RoomVoteDao;
use crate::game::bot::bot::Bot;
use crate::game::bot::bot_manager::BotManager;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::games::triggers::battle_ships_trigger::BattleShipsTrigger;
use crate::game::games::triggers::chess_trigger::ChessTrigger;
use crate::game::games::triggers::poker_trigger::PokerTrigger;
use crate::game::games::triggers::tic_tac_toe_trigger::TicTacToeTrigger;
use crate::game::item::interactors::types::chair_interactor::ChairInteractor;
use crate::game::item::interactors::types::pool_booth_interactor::PoolBoothInteractor;
use crate::game::item::interactors::types::pool_lift_interactor::PoolLiftInteractor;
use crate::game::item::public_items::public_item_parser::PublicItemParser;
use crate::game::pathfinder::position::Position;
use crate::game::pets::pet::Pet;
use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::game::room::room_manager::RoomManager;
use crate::game::triggers::generic_trigger::Trigger;
use crate::messages::outgoing::rooms::user::hotel_view::HOTEL_VIEW;
use crate::messages::outgoing::rooms::user::logout::LOGOUT;
use crate::messages::outgoing::rooms::user::user_objects::USER_OBJECTS;

#[derive(Clone)]
pub struct RoomEntityManager {
    players: Arc<Mutex<Vec<Arc<Mutex<Player>>>>>,
    bots: Arc<Mutex<Vec<Bot>>>,
    pets: Arc<Mutex<Vec<Pet>>>,
}

impl RoomEntityManager {
    /// Mirrors the `RoomEntityManager(Room)` constructor (the back-reference is
    // omitted; `room` is passed to the methods that need it).
    pub fn new() -> Self {
        Self {
            players: Arc::new(Mutex::new(Vec::new())),
            bots: Arc::new(Mutex::new(Vec::new())),
            pets: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Mirrors `getPlayers`.
    pub fn get_players(&self) -> Vec<Arc<Mutex<Player>>> {
        self.players.lock().clone()
    }

    /// Mirrors `getEntitiesByClass(Class<Bot>)`.
    pub fn get_bots(&self) -> Vec<Box<Bot>> {
        self.bots
            .lock()
            .iter()
            .map(|bot| Box::new(bot.clone()))
            .collect()
    }

    /// Mirrors `getEntitiesByClass(Class<Pet>)`.
    pub fn get_pets(&self) -> Vec<Pet> {
        self.pets.lock().clone()
    }

    /// Mirrors `getEntities` (the owned bots and pets; players are reached via
    // `get_players()`).
    pub fn get_entities(&self) -> Vec<Box<dyn Entity + Send>> {
        let mut entities: Vec<Box<dyn Entity + Send>> = self
            .bots
            .lock()
            .iter()
            .map(|bot| -> Box<dyn Entity + Send> { Box::new(bot.clone()) })
            .collect();

        for pet in self.pets.lock().iter() {
            entities.push(Box::new(pet.clone()));
        }

        entities
    }

    /// Mirrors `getById(int, EntityType)`.
    pub fn get_by_id(&self, id: i32, entity_type: EntityType) -> Option<Box<dyn Entity + Send>> {
        match entity_type {
            // A shared `Arc<Mutex<Player>>` can't be boxed as an owned
            // `Entity`; the player handle is exposed via `get_players()`.
            EntityType::Player => None,
            EntityType::Bot => self
                .bots
                .lock()
                .iter()
                .find(|bot| bot.get_details().get_id() == id)
                .map(|bot| -> Box<dyn Entity + Send> { Box::new(bot.clone()) }),
            EntityType::Pet => self
                .pets
                .lock()
                .iter()
                .find(|pet| pet.get_pet_details().get_id() == id)
                .map(|pet| -> Box<dyn Entity + Send> { Box::new(pet.clone()) }),
        }
    }

    /// Mirrors `enterRoom(Entity, Position)`.
    pub fn enter_room(&self, room: &Room, entity: &Bot, destination: Option<&Position>) {
        let _ = destination;
        // Port note: the Java `roomUser.reset()/setRoom/setInstanceId/
        // setPosition` and the `GamePlayer` handling are not ported (the
        // Rust `RoomPlayer` is a minimal composition).
        self.bots.lock().push(entity.clone());

        if self.players.lock().len() > 0 {
            room.send(&USER_OBJECTS::new(&[Box::new(entity.clone())]));
        }
    }

    /// Mirrors `enterRoom(Entity, Position)` (the generic overload).
    pub fn enter_room_entity(&self, room: &Room, entity: &dyn Entity, destination: Option<&Position>) {
        let _ = destination;

        // If the room is not loaded, add the room, as we intend to join it.
        if !RoomManager::get_instance().has_room(room.get_data().get_id()) && !room.is_game_arena() {
            RoomManager::get_instance().add_room(room);
        }

        match entity.get_type() {
            EntityType::Bot => {
                if let Some(bot) = entity.as_bot() {
                    self.bots.lock().push(bot.clone());
                }
            }
            EntityType::Pet => {
                if let Some(pet) = entity.as_pet() {
                    self.pets.lock().push(pet.clone());
                }
            }
            _ => {}
        }

        // Port note: the Java `USER_OBJECTS` send for non-players needs an
        // owned `Box<dyn Entity>`, which a borrowed `&dyn Entity` cannot be
        // converted to; the `roomUser` position / `GamePlayer` / packet
        // burst and `RoomDao.saveVisitors` parts are not ported (the Rust
        // `RoomPlayer` is a minimal composition).
    }

    /// Mirrors `tryRoomEntry(Player)`.
    pub fn try_room_entry(&self, room: &Room, player: &Player) {
        let is_room_active = room.is_active();

        if !is_room_active {
            room.set_active(true);
        }

        if let Some(trigger) = room.get_model().and_then(|model| model.get_room_trigger()) {
            trigger.on_room_entry(player, room, !is_room_active, &[]);
        }

        // Load bot data if first entry
        if !is_room_active {
            BotManager::get_instance().add_bots(room);

            // Mirrors the Java `tryInitialiseRoom` (room item / rights /
            // votes load on first entry).
            if !room.is_game_arena() {
                room.get_item_manager().clear_items();
                room.set_rights(Vec::new());
                room.set_votes(std::collections::HashMap::new());

                if room.is_public_room() {
                    if let Some(model) = room.get_model() {
                        for item in
                            PublicItemParser::get_public_items(room.get_data().get_id(), model.get_id())
                        {
                            room.get_item_manager().push_item(item);
                        }
                    }
                } else {
                    room.set_rights(RoomRightsDao::get_room_rights(room.get_data()));
                    room.set_votes(RoomVoteDao::get_ratings(room.get_data().get_id()));
                }

                for item in ItemDao::get_room_items(room.get_data()) {
                    room.get_item_manager().push_item(item);
                }
                room.get_item_manager().reset_item_states();
            }

            room.get_mapping().lock().regenerate_collision_map(room);
            room.get_task_manager().start_tasks(room);
        }
    }

    /// Mirrors `leaveRoom(Entity, boolean)` (the Java
    // `!room.getEntities().contains(entity)` early return is dropped; the
    // per-type `retain` removals are idempotent).
    pub fn leave_room(
        &self,
        room: &Room,
        entity: &(dyn Entity + Send),
        hotel_view: bool,
    ) {
        // The item `onEntityLeave` trigger (the Java virtual call; the
        // concrete dispatch covers the ported overrides).
        if let Some(room_user) = entity.get_room_user() {
            if let Some(mut current_item) = room_user.get_current_item() {
                let trigger = current_item
                    .get_definition()
                    .get_interaction_type()
                    .and_then(|interaction_type| interaction_type.get_trigger());
                if let Some(trigger) = trigger {
                    if let Some(chair) = trigger.downcast_ref::<ChairInteractor>() {
                        chair.on_entity_leave(entity, room_user, &current_item);
                    } else if let Some(pool_booth) =
                        trigger.downcast_ref::<PoolBoothInteractor>()
                    {
                        pool_booth.on_entity_leave(entity, room_user, &mut current_item);
                    } else if let Some(pool_lift) =
                        trigger.downcast_ref::<PoolLiftInteractor>()
                    {
                        pool_lift.on_entity_leave(entity, room_user, &mut current_item);
                    } else if let Some(chess) = trigger.downcast_ref::<ChessTrigger>() {
                        chess.on_entity_leave(entity, room_user, &current_item);
                    } else if let Some(poker) = trigger.downcast_ref::<PokerTrigger>() {
                        poker.on_entity_leave(entity, room_user, &current_item);
                    } else if let Some(battle_ships) = {
                        trigger.downcast_ref::<BattleShipsTrigger>()
                    } {
                        battle_ships.on_entity_leave(entity, room_user, &current_item);
                    } else if let Some(tic_tac_toe) = {
                        trigger.downcast_ref::<TicTacToeTrigger>()
                    } {
                        tic_tac_toe.on_entity_leave(entity, room_user, &current_item);
                    }
                }
            }
        }

        if let Some(bot) = entity.as_bot() {
            self.bots.lock().retain(|b| b.get_details().get_id() != bot.get_details().get_id());
        }
        if let Some(pet) = entity.as_pet() {
            self.pets
                .lock()
                .retain(|p| p.get_pet_details().get_id() != pet.get_pet_details().get_id());
        }
        if entity.get_type() == EntityType::Player {
            if let Some(player) = entity.as_player() {
                self.players.lock().retain(|p| {
                    p.lock().get_details().get_id() != player.get_details().get_id()
                });
            }
        }

        if let Some(trigger) = room.get_model().and_then(|model| model.get_room_trigger()) {
            trigger.on_room_leave(entity, room, &[]);
        }

        // Entity tile removal.
        if let Some(room_user) = entity.get_room_user() {
            if let Some(tile) = room_user.get_tile() {
                tile.lock().remove_entity(entity);
            }
            if let Some(next_position) = room_user.get_next_position() {
                if let Some(mut next_tile) = room
                    .get_mapping()
                    .lock()
                    .get_tile(room, next_position.get_x(), next_position.get_y())
                {
                    next_tile.remove_entity(entity);
                }
            }
        }

        // Port note: the Java `setVisitorsNow(getPlayers().size())`
        // in-memory update is not ported (the Rust `Room` `data` field is
        // not interior-mutable through the shared `&Room`); the
        // `RoomDao.saveVisitors` below persists the current value.
        room.send(&LOGOUT::new(
            entity
                .get_room_user()
                .map(|room_user| room_user.get_instance_id())
                .unwrap_or(entity.get_details().get_id()),
        ));
        room.try_dispose();
        entity.get_room_user().map(|room_user| room_user.reset());

        if entity.get_type() != EntityType::Player {
            return;
        }
        let Some(player) = entity.as_player() else {
            return;
        };

        if hotel_view {
            player.send(&HOTEL_VIEW);
        }
        if let Some(messenger) = player.get_messenger() {
            messenger.send_status_update();
        }
        RoomDao::save_visitors(room);

        // Port note: the Java `gamePlayer.leaveGame(...)` /
        // `setGamePlayer(null)` are not ported (the Rust `RoomPlayer` has
        // no `GamePlayer`).
    }

    /// Clear the entity collections (used by `Room.try_dispose`).
    pub fn clear_entities(&self) {
        self.bots.lock().clear();
        self.pets.lock().clear();
    }

    /// Add a player to the room.
    pub fn add_player(&self, player: Arc<Mutex<Player>>) {
        self.players.lock().push(player);
    }

    /// Remove a player from the room by id.
    pub fn remove_player(&self, id: i32) {
        self.players
            .lock()
            .retain(|player| player.lock().get_details().get_id() != id);
    }
}

impl Default for RoomEntityManager {
    fn default() -> Self {
        Self::new()
    }
}
