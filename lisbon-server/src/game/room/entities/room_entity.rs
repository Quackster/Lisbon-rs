//! Mirrors `net.h4bbo.lisbon.game.room.entities.RoomEntity`.
//!
//! The state is `Arc`/`Mutex`-wrapped so a `RoomEntity` clone (as held by
//! `RoomBot`/`RoomPlayer`) shares the underlying state. The `entity` field is
//! a non-cloneable `Box<dyn Entity + Send>` (it is `Send`, keeping the state
//! `Send`); it is used by reference while the state lock is held. Getters
//! that hand a value across locks (`get_room`, `get_tile`) return owned
//! clones / shared handles rather than a borrow of a temporary guard.
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::bot::bot_manager::BotManager;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::games::player::game_player::GamePlayer;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::item::Item;
use crate::game::item::roller::rolling_data::RollingData;
use crate::game::moderation::chat_manager::ChatManager;
use crate::game::pathfinder::position::Position;
use crate::game::pathfinder::rotation::Rotation;
use crate::game::player::player::Player;
use crate::game::room::enums::drink_type::DrinkType;
use crate::game::room::enums::status_type::StatusType;
use crate::game::room::mapping::room_tile::RoomTile;
use crate::game::room::room::Room;
use crate::game::room::room_user_status::RoomUserStatus;
use crate::game::texts::texts_manager::TextsManager;
use crate::messages::outgoing::rooms::user::chat_message::ChatMessageType;
use crate::messages::outgoing::rooms::user::chat_message::CHAT_MESSAGE;
use crate::messages::outgoing::rooms::user::user_statuses::USER_STATUSES;
use crate::util::config::game_configuration::GameConfiguration;

#[derive(Clone)]
pub struct RoomEntity {
    inner: Arc<Mutex<RoomEntityState>>,
}

pub struct RoomEntityState {
    entity: Option<Box<dyn Entity + Send>>,
    position: Position,
    goal: Option<Position>,
    next_position: Option<Position>,
    room: Option<Arc<Mutex<Room>>>,
    rolling_data: Option<RollingData>,
    statuses: HashMap<String, RoomUserStatus>,
    path: Vec<Position>,
    task: Option<crate::game::room::tasks::pet_task::PetTask>,
    instance_id: i32,
    last_item_interaction: Option<Item>,
    is_walking: bool,
    is_walking_allowed: bool,
    being_kicked: bool,
    needs_update: bool,
    enable_walking_on_stop: bool,
    look_timer: i32,
    chat_bubble_timer: i64,
    // Player-specific fields (the Java `RoomPlayer` subclass state; Rust has
    // no subtyping, so they live here and the `RoomPlayer` methods below
    // operate on them).
    authenticate_id: i32,
    authenticate_teleporter_id: i32,
    pending_teleporter_id: i32,
    queued_teleporter_id: i32,
    observing_game_id: i32,
    lido_vote: i32,
    is_typing: bool,
    is_diving: bool,
    game_player: Option<Arc<Mutex<GamePlayer>>>,
    current_game_id: Option<String>,
    chat_spam_count: i32,
    chat_spam_ticks: i32,
    mute_time: i64,
    trade_partner: Option<Arc<Mutex<Player>>>,
    trade_items: Mutex<Vec<Item>>,
    trade_accept: bool,
}

impl RoomEntityState {
    pub fn task_mut(
        &mut self,
    ) -> Option<&mut crate::game::room::tasks::pet_task::PetTask> {
        self.task.as_mut()
    }

    fn with_entity(entity: Option<Box<dyn Entity + Send>>) -> Self {
        Self {
            entity,
            position: Position::default(),
            goal: None,
            next_position: None,
            room: None,
            rolling_data: None,
            statuses: HashMap::new(),
            path: Vec::new(),
            task: None,
            instance_id: -1,
            last_item_interaction: None,
            is_walking: false,
            is_walking_allowed: true,
            being_kicked: false,
            needs_update: false,
            enable_walking_on_stop: false,
            look_timer: 0,
            chat_bubble_timer: 0,
            authenticate_id: -1,
            authenticate_teleporter_id: -1,
            pending_teleporter_id: -1,
            queued_teleporter_id: -1,
            observing_game_id: -1,
            lido_vote: 0,
            is_typing: false,
            is_diving: false,
            game_player: None,
            current_game_id: None,
            chat_spam_count: 0,
            chat_spam_ticks: 16,
            mute_time: 0,
            trade_partner: None,
            trade_items: Mutex::new(Vec::new()),
            trade_accept: false,
        }
    }
}

impl RoomEntity {
    /// Mirrors the `RoomEntity(Entity)` constructor.
    pub fn new(entity: Option<Box<dyn Entity + Send>>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(RoomEntityState::with_entity(entity))),
        }
    }

    /// Mirors `getTradePartner`.
    pub fn get_trade_partner(&self) -> Option<Arc<Mutex<Player>>> {
        self.inner.lock().trade_partner.clone()
    }

    /// Mirors `setTradePartner(Player)`.
    pub fn set_trade_partner(&self, trade_partner: Option<Arc<Mutex<Player>>>) {
        self.inner.lock().trade_partner = trade_partner;
    }

    /// Mirrors `setTradeAccept(boolean)`.
    pub fn set_trade_accept(&self, trade_accept: bool) {
        self.inner.lock().trade_accept = trade_accept;
    }

    /// Mirrors `hasAcceptedTrade()`.
    pub fn has_accepted_trade(&self) -> bool {
        self.inner.lock().trade_accept
    }

    /// Mirrors `getTradeItems().add(Item)`.
    pub fn add_trade_item(&self, item: &Item) {
        self.inner.lock().trade_items.lock().push(item.clone());
    }

    /// Mirrors `getTradeItems().clear()`.
    pub fn clear_trade_items(&self) {
        self.inner.lock().trade_items.lock().clear();
    }

    /// Mirors `getTradeItems`.
    pub fn get_trade_items(&self) -> Vec<Item> {
        self.inner.lock().trade_items.lock().clone()
    }

    /// Mirrors `reset`.
    pub fn reset(&self) {
        let mut s = self.inner.lock();
        s.statuses.clear();
        s.path.clear();
        s.next_position = None;
        s.goal = None;
        s.room = None;
        s.rolling_data = None;
        s.last_item_interaction = None;
        s.is_walking = false;
        s.is_walking_allowed = true;
        s.being_kicked = false;
        s.instance_id = -1;
            s.is_typing = false;
            s.is_diving = false;
            s.chat_bubble_timer = 0;
            s.observing_game_id = -1;
            s.lido_vote = 0;
        s.pending_teleporter_id = -1;
        s.queued_teleporter_id = -1;
    }

    /// Mirrors `kick(boolean)`.
    pub fn kick(&self, allow_walking: bool) {
        let Some(room) = self.get_room() else {
            return;
        };

        let door_location = match room.get_model() {
            Some(model) => model.get_door_location().copy(),
            None => {
                self.leave_room(true);
                return;
            }
        };

        if door_location == self.get_position() {
            self.leave_room(true);
            return;
        }

        self.walk_to(door_location.get_x(), door_location.get_y());
        let was_walking = {
            let mut s = self.inner.lock();
            s.is_walking_allowed = allow_walking;
            s.being_kicked = true;
            s.is_walking
        };

        if !was_walking {
            self.leave_room(true);
        }
    }

    fn leave_room(&self, _hotel_view: bool) {
        let room = self.get_room();
        let state = self.inner.lock();
        let Some(room) = room else {
            return;
        };
        let Some(entity) = state.entity.as_deref() else {
            return;
        };
        room.get_entity_manager().leave_room(&room, entity, true);
    }

    /// Mirrors `walkTo(int, int)`.
    /// Mirrors `walkTo`.
    pub fn walk_to(&self, x: i32, y: i32) -> bool {
        let Some(room) = self.get_room() else {
            return false;
        };

        if crate::game::room::public_rooms::sun_terrace_handler::SunTerraceHandler::is_redirected(
            self, x, y,
        ) {
            return false;
        }

        if self.inner.lock().next_position.is_some() {
            let old_position = {
                let mut s = self.inner.lock();
                let old = s.position.copy();
                if let Some(next) = s.next_position.clone() {
                    s.position.set_x(next.get_x());
                    s.position.set_y(next.get_y());
                }
                old
            };
            self.update_new_height(&self.get_position());
            // Java triggers `onEntityStep` when the current item's
            // interaction has a trigger; the trigger wiring is covered by
            // the item behaviour port, so the step trigger is dropped here.
            let _ = old_position;
        }

        {
            let mapping = room.get_mapping();
            let mapping = mapping.lock();
            if mapping.get_tile(&room, x, y).is_none() {
                // "User requested X, Y from position" (commented in Java).
                return false;
            }
        }

        let goal = Position::new_xy(x, y);

        let valid = {
            let s = self.inner.lock();
            let entity = s.entity.as_deref();
            RoomTile::is_valid_tile(&room, entity, &goal)
        };
        if !valid {
            return false;
        }

        // `CAN_LAY_ON_TOP` pillow redirect (the `mapping` lock must be
        // dropped before the recursive `walk_to`).
        let pillow_destination = {
            let mapping = room.get_mapping();
            let mapping = mapping.lock();
            let mut destination = None;
            if let Some(tile) = mapping.get_tile(&room, x, y) {
                if let Some(highest_item) = tile.get_highest_item() {
                    if highest_item.has_behaviour(ItemBehaviour::CanLayOnTop)
                        && !crate::game::item::interactors::types::bed_interactor::BedInteractor::is_valid_pillow_tile(
                            highest_item, &goal,
                        )
                    {
                        destination = Some(
                            crate::game::item::interactors::types::bed_interactor::BedInteractor::convert_to_pillow(
                                &goal, highest_item,
                            ),
                        );
                    }
                }
            }
            destination
        };
        if let Some(destination) = pillow_destination {
            return self.walk_to(destination.get_x(), destination.get_y());
        }

        // The `entity` is a `Box<dyn Entity + Send>` (not cloneable); the
        // path is computed against the borrowed reference.
        let path_list = {
            let s = self.inner.lock();
            let Some(entity) = s.entity.as_deref() else {
                return false;
            };
            let position = s.position.copy();
            crate::game::pathfinder::pathfinder::Pathfinder::make_path(
                entity, &position, &goal,
            )
        };
        if path_list.is_empty() {
            return false;
        }

        let mut s = self.inner.lock();
        s.goal = Some(goal);
        s.path = path_list;
        s.is_walking = true;
        true
    }

    /// Mirrors `stopWalking`.
    pub fn stop_walking(&self) {
        let mut s = self.inner.lock();
        s.path.clear();
        s.is_walking = false;
        s.needs_update = true;
        s.next_position = None;
        s.statuses.remove(StatusType::Move.status_code());

        if s.enable_walking_on_stop {
            s.enable_walking_on_stop = false;
            s.is_walking_allowed = true;
        }

        let is_player = s
            .entity
            .as_ref()
            .map(|e| e.get_type() == EntityType::Player)
            .unwrap_or(false);
        let being_kicked = s.being_kicked;
        let position = s.position.copy();
        let room_arc = s.room.clone();

        if is_player {
            if let Some(room_arc) = room_arc {
                let g = room_arc.lock();
                let is_public = g.is_public_room();
                let at_door = g
                    .get_model()
                    .map(|m| m.get_door_location().copy())
                    .map_or(false, |d| d == position);
                drop(g);
                // Java consults `WalkwaysManager.getWalkway` (not ported);
                // the walkway redirect is dropped, so a door tile in a flat
                // (or a kick) leaves the room.
                if at_door || (being_kicked && !is_public) || being_kicked {
                    if let Some(entity) = s.entity.as_deref() {
                        let rg = room_arc.lock();
                        rg.get_entity_manager().leave_room(&rg, entity, true);
                        return;
                    }
                }
            }
        }

        drop(s);
        self.invoke_item(None, false);
    }

    /// Mirrors `invokeItem(Position, boolean)`.
    pub fn invoke_item(&self, _old_position: Option<&Position>, invoke: bool) {
        let Some(room) = self.get_room() else {
            return;
        };

        let walking_height = self
            .get_tile()
            .map(|t| t.lock().get_walking_height())
            .unwrap_or(0.0);
        self.inner.lock().position.set_z(walking_height);

        let current_item = self.get_current_item();

        let can_interact = current_item
            .as_ref()
            .map(|i| {
                i.has_behaviour(ItemBehaviour::CanSitOnTop)
                    && i.has_behaviour(ItemBehaviour::CanLayOnTop)
            })
            .unwrap_or(false);

        if current_item.is_none() || !can_interact {
            let mut s = self.inner.lock();
            if s.statuses.contains_key(StatusType::Sit.status_code())
                || s.statuses.contains_key(StatusType::Lay.status_code())
            {
                s.statuses.remove(StatusType::Sit.status_code());
                s.statuses.remove(StatusType::Lay.status_code());
            }

            if current_item.is_none() {
                // Java triggers `onEntityLeave`; the `game::triggers` port is
                // pending, so the leave trigger is dropped.
                s.last_item_interaction = None;
            }
        } else if let Some(item) = &current_item {
            // Java triggers `onEntityStop`; the `game::triggers` port is
            // pending, so the stop trigger is dropped.
            self.inner.lock().last_item_interaction = Some(item.clone());
        }

        self.update_new_height(&self.get_position());

        if invoke {
            let state = self.inner.lock();
            let refs: Vec<&(dyn Entity + Send)> = state
                .entity
                .iter()
                .map(|entity| entity.as_ref())
                .collect();
            room.send(&USER_STATUSES::new(refs));
        } else {
            self.inner.lock().needs_update = true;
        }
    }

    /// Mirrors `carryItem(int, String)`.
    pub fn carry_item(&self, mut carry_id: i32, carry_name: Option<&str>) {
        if self.contains_status(StatusType::CarryItem) {
            return;
        }

        let mut use_raw_string = false;

        let mut drinks = [None; 26];
        drinks[1] = Some(DrinkType::Drink);
        drinks[2] = Some(DrinkType::Drink);
        drinks[3] = Some(DrinkType::Eat);
        drinks[4] = Some(DrinkType::Eat);
        drinks[5] = Some(DrinkType::Drink);
        drinks[6] = Some(DrinkType::Drink);
        drinks[7] = Some(DrinkType::Drink);
        drinks[8] = Some(DrinkType::Drink);
        drinks[9] = Some(DrinkType::Drink);
        drinks[10] = Some(DrinkType::Drink);
        drinks[11] = Some(DrinkType::Drink);
        drinks[12] = Some(DrinkType::Drink);
        drinks[13] = Some(DrinkType::Drink);
        drinks[14] = Some(DrinkType::Drink);
        drinks[15] = Some(DrinkType::Drink);
        drinks[16] = Some(DrinkType::Drink);
        drinks[17] = Some(DrinkType::Drink);
        drinks[18] = Some(DrinkType::Drink);
        drinks[19] = Some(DrinkType::Drink);
        drinks[20] = Some(DrinkType::Item);
        drinks[21] = Some(DrinkType::Eat);
        drinks[22] = Some(DrinkType::Drink);
        drinks[23] = Some(DrinkType::Drink);
        drinks[24] = Some(DrinkType::Drink);
        drinks[25] = Some(DrinkType::Drink);

        if let Some(name) = carry_name {
            for i in 0..=25 {
                let external_drink_name =
                    TextsManager::get_instance().get_value(&format!("handitem{}", i));

                if !external_drink_name.is_empty() && external_drink_name.eq_ignore_ascii_case(name) {
                    carry_id = i;
                }
            }
        }

        let type_ = if carry_id <= 0 || carry_id > 25 {
            use_raw_string = true;
            DrinkType::Drink
        } else {
            drinks[carry_id as usize].unwrap()
        };

        let (carry_status, use_status) = match type_ {
            DrinkType::Drink => (Some(StatusType::CarryDrink), Some(StatusType::UseDrink)),
            DrinkType::Eat => (Some(StatusType::CarryFood), Some(StatusType::UseFood)),
            DrinkType::Item => (Some(StatusType::CarryItem), Some(StatusType::UseItem)),
        };

        self.remove_status(StatusType::CarryItem);
        self.remove_status(StatusType::CarryFood);
        self.remove_status(StatusType::CarryDrink);
        self.remove_status(StatusType::Dance);

        let timer = GameConfiguration::get_instance().get_integer("carry.timer.seconds");

        if let (Some(carry_status), Some(use_status)) = (carry_status, use_status) {
            if !use_raw_string {
                if carry_status != StatusType::CarryItem {
                    self.set_status_timed(
                        carry_status,
                        &carry_id.to_string(),
                        timer,
                        Some(use_status),
                        12,
                        1,
                    );
                } else {
                    self.set_status(carry_status, &carry_id.to_string());
                }
            } else if let Some(name) = carry_name {
                self.set_status_timed(
                    carry_status,
                    name,
                    timer,
                    Some(use_status),
                    12,
                    1,
                );
            }
        }

        self.set_needs_update(true);
    }

    /// Mirrors `removeDrinks`.
    pub fn remove_drinks(&self) {
        if self.contains_status(StatusType::CarryFood)
            || self.contains_status(StatusType::CarryDrink)
        {
            self.remove_status(StatusType::CarryDrink);
            self.remove_status(StatusType::CarryFood);
            self.set_needs_update(true);
        }
    }

    /// Mirrors `talk(String, ChatMessageType)`.
    pub fn talk(&self, message: &str, chat_message_type: ChatMessageType) {
        let is_player = {
            let s = self.inner.lock();
            s.entity
                .as_ref()
                .map(|e| e.get_type() == EntityType::Player)
                .unwrap_or(false)
        };

        let Some(room) = self.get_room() else {
            return;
        };

        let mut recieve_messages: Vec<Arc<Mutex<Player>>> = Vec::new();

        if is_player {
            let sender_name = {
                let s = self.inner.lock();
                s.entity
                    .as_ref()
                    .and_then(|e| e.as_player())
                    .map(|p| p.get_details().get_name().to_string())
            };

            if let Some(sender_name) = sender_name {
                for session in room.get_entity_manager().get_players() {
                    let ignored = session.lock().get_ignored_list().contains(&sender_name);
                    if ignored {
                        continue;
                    }
                    recieve_messages.push(session);
                }
            }
        } else {
            for p in room.get_entity_manager().get_players() {
                recieve_messages.push(p);
            }
        }

        self.talk_to(message, chat_message_type, &recieve_messages);
    }

    /// Mirrors `talk(String, ChatMessageType, List<Player>)`.
    pub fn talk_to(
        &self,
        message: &str,
        chat_message_type: ChatMessageType,
        recieve_messages: &[Arc<Mutex<Player>>],
    ) {
        if chat_message_type != ChatMessageType::Whisper {
            let self_position = self.get_position();
            for player in recieve_messages {
                let p_guard = player.lock();
                let Some(room_user) = p_guard.get_room_user() else {
                    continue;
                };

                if room_user.contains_status(StatusType::AvatarSleep) {
                    continue;
                }

                if chat_message_type == ChatMessageType::Chat
                    && self_position.get_distance_squared(&room_user.get_position()) > 14
                {
                    continue;
                }

                room_user.look(&self_position, false);
            }
        }

        let words = message.split(' ').count();
        let talk_duration = if words == 1 {
            message.len() / 10
        } else if words <= 5 {
            words / 2
        } else {
            5
        };

        if talk_duration > 0 {
            self.set_status_timed(StatusType::Talk, "", talk_duration as i32, None, -1, -1);
            self.set_needs_update(true);
        }

        if let Some(gesture) = Self::get_chat_gesture(message) {
            self.set_status_timed(StatusType::Gesture, gesture, 5, None, -1, -1);
            self.set_needs_update(true);
        }

        let chat_msg = CHAT_MESSAGE::new(chat_message_type, self.get_instance_id(), message);
        for player in recieve_messages {
            player.lock().send(&chat_msg);
        }

        let is_player = {
            let s = self.inner.lock();
            s.entity
                .as_ref()
                .map(|e| e.get_type() == EntityType::Player)
                .unwrap_or(false)
        };

        if is_player {
            let room = self.get_room();
            let s = self.inner.lock();
            let player = s.entity.as_ref().and_then(|e| e.as_player());

            if let (Some(player), Some(room)) = (player, room) {
                if chat_message_type != ChatMessageType::Whisper {
                    BotManager::get_instance().handle_speech(player, &room, message);
                }
                ChatManager::get_instance().queue(player, &room, message, chat_message_type);
            }
            drop(s);
            self.reset_room_timer();
        }
    }

    /// Mirrors `getChatGesture(String)`.
    fn get_chat_gesture(message: &str) -> Option<&'static str> {
        let mut gesture: Option<&'static str> = None;

        if message.contains(":)")
            || message.contains(":-)")
            || message.contains(":p")
            || message.contains(":d")
            || message.contains(":D")
            || message.contains(";)")
            || message.contains(";-)")
        {
            gesture = Some("sml");
        }

        if gesture.is_none()
            && (message.contains(":s")
                || message.contains(":(")
                || message.contains(":-(")
                || message.contains(":'("))
        {
            gesture = Some("sad");
        }

        if gesture.is_none() && (message.contains(":o") || message.contains(":O")) {
            gesture = Some("srp");
        }

        if gesture.is_none() && (message.contains(":@") || message.contains(":>(")) {
            gesture = Some("agr");
        }

        gesture
    }

    /// Mirrors `look(Position, boolean)`.
    pub fn look(&self, towards: &Position, body: bool) {
        if self.is_walking() {
            return;
        }

        let position = self.get_position();

        let mut s = self.inner.lock();
        s.position.set_head_rotation(Rotation::get_head_rotation(
            position.get_rotation(),
            &position,
            towards,
        ));

        if body {
            let rotation = Rotation::calculate_human_direction(
                position.get_x(),
                position.get_y(),
                towards.get_x(),
                towards.get_y(),
            );
            s.position.set_head_rotation(rotation);
            s.position.set_body_rotation(rotation);
        } else {
            s.look_timer = 3;
        }

        s.needs_update = true;
    }

    /// Mirrors `wave`.
    pub fn wave(&self) {
        if self.contains_status(StatusType::Wave) {
            return;
        }

        if self.contains_status(StatusType::Dance) {
            self.remove_status(StatusType::Dance);
        }

        self.set_status(StatusType::Wave, "");

        if !self.is_walking() {
            if let Some(room) = self.get_room() {
                let state = self.inner.lock();
                let refs: Vec<&(dyn Entity + Send)> = state
                    .entity
                    .iter()
                    .map(|entity| entity.as_ref())
                    .collect();
                room.send(&USER_STATUSES::new(refs));
            }
        }

        // Java schedules a `WaveTask`; the `GameScheduler` service is a no-op
        // stub, so the deferred wave-end is dropped.
    }

    /// Mirrors `updateNewHeight(Position)`.
    pub fn update_new_height(&self, position: &Position) {
        let Some(room) = self.get_room() else {
            return;
        };

        let height_opt = {
            let mapping = room.get_mapping();
            let mapping = mapping.lock();
            mapping
                .get_tile_by_position(&room, position)
                .map(|t| t.get_walking_height())
        };
        let Some(height) = height_opt else {
            return;
        };
        let old_height = self.inner.lock().position.get_z();

        if height != old_height {
            self.inner.lock().position.set_z(height);
            self.set_needs_update(true);
        }
    }

    /// Mirrors `getTile`.
    pub fn get_tile(&self) -> Option<Arc<Mutex<RoomTile>>> {
        let state = self.inner.lock();
        let Some(room_arc) = state.room.as_ref() else {
            return None;
        };

        let room = room_arc.lock();
        room.get_mapping().lock()
            .get_tile_handle_for(&room, state.position.get_x(), state.position.get_y())
    }

    /// Mirrors `warp(Position, boolean, boolean)`.
    pub fn warp(&self, position: &Position, instant_update: bool, send_user_object: bool) {
        let Some(room) = self.get_room() else {
            return;
        };

        // Remove the entity from the old and next tiles while holding the
        // state lock (the entity is a non-cloneable `Box`, used by reference).
        {
            let state = self.inner.lock();
            if let Some(entity) = state.entity.as_deref() {
                if let Some(room_arc) = state.room.clone() {
                    let g = room_arc.lock();
                    let old_tile = g
                        .get_mapping().lock()
                        .get_tile_handle_for(
                            &g,
                            state.position.get_x(),
                            state.position.get_y(),
                        );
                    if let Some(old_tile) = old_tile {
                        old_tile.lock().remove_entity(entity);
                    }

                    if let Some(next) = state.next_position.clone() {
                        if let Some(mut next_tile) =
                            g.get_mapping().lock().get_tile_by_position(&g, &next)
                        {
                            next_tile.remove_entity(entity);
                        }
                    }
                }
            }
        }

        self.inner.lock().position = position.copy();
        self.update_new_height(&self.get_position());

        // Java `newTile.addEntity(entity)`; the `game::triggers` port is
        // pending, so the enter trigger is dropped.
        if instant_update {
            if send_user_object {
                // Java `new USER_OBJECTS(List.of(entity))`; `EntityState` is
                // not fully ported, so the object broadcast is dropped.
            }

            let state = self.inner.lock();
            let refs: Vec<&(dyn Entity + Send)> = state
                .entity
                .iter()
                .map(|entity| entity.as_ref())
                .collect();
            room.send(&USER_STATUSES::new(refs));

            // Java `invokeItem(oldTile.getPosition(), true)`.
            self.invoke_item(None, true);
        }
    }

    /// Mirrors `containsStatus`.
    pub fn contains_status(&self, status: StatusType) -> bool {
        self.inner.lock().statuses.contains_key(status.status_code())
    }

    /// Mirrors `removeStatus`.
    pub fn remove_status(&self, status: StatusType) {
        self.inner.lock().statuses.remove(status.status_code());
    }

    /// Mirrors `setStatus(StatusType, Object)`.
    pub fn set_status(&self, status: StatusType, value: &str) {
        let mut s = self.inner.lock();
        s.statuses.remove(status.status_code());
        s.statuses
            .insert(status.status_code().to_string(), RoomUserStatus::new(status, value));
    }

    /// Mirrors `setStatus` with a lifetime and optional action.
    pub fn set_status_timed(
        &self,
        status: StatusType,
        value: &str,
        sec_lifetime: i32,
        action: Option<StatusType>,
        sec_action_switch: i32,
        sec_switch_lifetime: i32,
    ) {
        let mut s = self.inner.lock();
        s.statuses.remove(status.status_code());
        s.statuses.insert(
            status.status_code().to_string(),
            RoomUserStatus::new_with_action(
                status,
                value,
                sec_lifetime,
                action,
                sec_action_switch,
                sec_switch_lifetime,
            ),
        );
    }

    /// Mirrors `isSittingOnGround`.
    pub fn is_sitting_on_ground(&self) -> bool {
        match self.get_current_item() {
            None => self.contains_status(StatusType::Sit),
            Some(item) => {
                if !item.has_behaviour(ItemBehaviour::CanSitOnTop) {
                    self.contains_status(StatusType::Sit)
                } else {
                    false
                }
            }
        }
    }

    /// Mirrors `isSittingOnChair`.
    pub fn is_sitting_on_chair(&self) -> bool {
        match self.get_current_item() {
            Some(item) => item.has_behaviour(ItemBehaviour::CanSitOnTop),
            None => false,
        }
    }

    /// Mirrors `getStatus(StatusType)`.
    pub fn get_status(&self, status: StatusType) -> Option<RoomUserStatus> {
        self.inner.lock().statuses.get(status.status_code()).cloned()
    }

    /// Mirrors `getStatuses`.
    pub fn get_statuses(&self) -> HashMap<String, RoomUserStatus> {
        self.inner.lock().statuses.clone()
    }

    /// Mirrors `getStatuses().clear()`.
    pub fn clear_statuses(&self) {
        self.inner.lock().statuses.clear();
    }

    /// Mirrors the `StatusTask` countdown block over `getStatuses()`:
    /// decrement the action-switch / action / lifetime countdowns, swap
    /// the key action when a switch happens and remove (returning the
    /// keys of) the statuses whose lifetime expired.
    pub fn tick_statuses(&self) -> Vec<String> {
        let mut to_remove = Vec::new();
        let mut needs_update = false;
        let mut state = self.inner.lock();

        for (key, status) in state.statuses.iter_mut() {
            if status.get_action_switch_countdown() > 0 {
                status.set_action_switch_countdown(status.get_action_switch_countdown() - 1);
            } else if status.get_action_switch_countdown() == 0 {
                status.set_action_switch_countdown(-1);
                status.set_action_countdown(status.get_sec_action_switch());
                status.swap_key_action();
                needs_update = true;
            }

            if status.get_action_countdown() > 0 {
                status.set_action_countdown(status.get_action_countdown() - 1);
            } else if status.get_action_countdown() == 0 {
                status.set_action_countdown(-1);
                status.set_action_switch_countdown(status.get_sec_switch_lifetime());
                status.swap_key_action();
                needs_update = true;
            }

            if status.get_lifetime_countdown() > 0 {
                status.set_lifetime_countdown(status.get_lifetime_countdown() - 1);
            } else if status.get_lifetime_countdown() == 0 {
                status.set_lifetime_countdown(-1);
                to_remove.push(key.clone());
                needs_update = true;
            }
        }

        if needs_update {
            state.needs_update = true;
        }

        for key in &to_remove {
            state.statuses.remove(key);
        }

        to_remove
    }

    /// Mirrors `getCurrentItem`.
    pub fn get_current_item(&self) -> Option<Item> {
        let tile = self.get_tile()?;
        let guard = tile.lock();
        guard.get_highest_item().map(|i| i.clone())
    }

    /// Mirrors `getPosition`.
    pub fn get_position(&self) -> Position {
        self.inner.lock().position.copy()
    }

    /// Mirrors `setPosition(Position)`.
    pub fn set_position(&self, position: Position) {
        self.inner.lock().position = position;
    }

    /// Mirrors `getGoal`.
    pub fn get_goal(&self) -> Position {
        self.inner.lock().goal.clone().unwrap_or_default()
    }

    /// Mirrors `getNextPosition`.
    pub fn get_next_position(&self) -> Option<Position> {
        self.inner.lock().next_position.clone()
    }

    /// Mirrors `setNextPosition(Position)`.
    pub fn set_next_position(&self, next_position: Option<Position>) {
        self.inner.lock().next_position = next_position;
    }

    /// Mirrors `getRoom`.
    pub fn get_room(&self) -> Option<Room> {
        let state = self.inner.lock();
        state.room.as_ref().map(|r| r.lock().clone())
    }

    /// Mirrors `setRoom(Room)`.
    pub fn set_room(&self, room: Room) {
        self.inner
            .lock()
            .room
            .replace(Arc::new(Mutex::new(room)));
    }

    /// Mirrors `getInstanceId`.
    pub fn get_instance_id(&self) -> i32 {
        self.inner.lock().instance_id
    }

    /// Mirrors `setInstanceId(int)`.
    pub fn set_instance_id(&self, instance_id: i32) {
        self.inner.lock().instance_id = instance_id;
    }

    /// Mirrors `getPath`.
    pub fn get_path(&self) -> Vec<Position> {
        self.inner.lock().path.clone()
    }

    /// Mirrors `getPath().pop()`.
    pub fn pop_path(&self) -> Option<Position> {
        self.inner.lock().path.pop()
    }

    /// Mirrors `getPath().clear()`.
    pub fn clear_path(&self) {
        self.inner.lock().path.clear();
    }

    /// Mirrors `isWalking`.
    pub fn is_walking(&self) -> bool {
        self.inner.lock().is_walking
    }

    /// Mirrors `setWalking(boolean)`.
    pub fn set_walking(&self, walking: bool) {
        self.inner.lock().is_walking = walking;
    }

    /// Mirrors `isNeedsUpdate`.
    pub fn is_needs_update(&self) -> bool {
        self.inner.lock().needs_update
    }

    /// Mirrors `setNeedsUpdate(boolean)`.
    pub fn set_needs_update(&self, needs_update: bool) {
        self.inner.lock().needs_update = needs_update;
    }

    /// Mirrors `getRollingData`.
    pub fn get_rolling_data(&self) -> Option<RollingData> {
        self.inner.lock().rolling_data.clone()
    }

    /// Mirrors `setRollingData(RollingData)`.
    pub fn set_rolling_data(&self, rolling_data: Option<RollingData>) {
        self.inner.lock().rolling_data = rolling_data;
    }

    /// Mirrors `isWalkingAllowed`.
    pub fn is_walking_allowed(&self) -> bool {
        self.inner.lock().is_walking_allowed
    }

    /// Mirrors `setWalkingAllowed(boolean)`.
    pub fn set_walking_allowed(&self, walking_allowed: bool) {
        self.inner.lock().is_walking_allowed = walking_allowed;
    }

    /// Mirrors `getLastItemInteraction`.
    pub fn get_last_item_interaction(&self) -> Option<Item> {
        self.inner.lock().last_item_interaction.clone()
    }

    /// Mirrors `setLastItemInteraction(Item)`.
    pub fn set_last_item_interaction(&self, item: &Item) {
        self.inner.lock().last_item_interaction = Some(item.clone());
    }

    /// Mirrors `setLastItemInteraction(null)`.
    pub fn clear_last_item_interaction(&self) {
        self.inner.lock().last_item_interaction = None;
    }

    /// Mirrors `isRolling`.
    pub fn is_rolling(&self) -> bool {
        self.inner.lock().rolling_data.is_some()
    }

    /// Mirrors `setEnableWalkingOnStop(boolean)`.
    pub fn set_enable_walking_on_stop(&self, enable_walking_on_stop: bool) {
        let mut s = self.inner.lock();
        s.enable_walking_on_stop = enable_walking_on_stop;
        s.is_walking_allowed = !enable_walking_on_stop;
    }

    // Player-specific state (the Java `RoomPlayer` subclass).

    /// Mirrors `handleSpamTicks`.
    pub fn handle_spam_ticks(&self) {
        let mut s = self.inner.lock();
        if s.chat_spam_ticks >= 0 {
            s.chat_spam_ticks -= 1;

            if s.chat_spam_ticks == -1 {
                s.chat_spam_count = 0;
            }
        }
    }

    /// Mirrors `getMuteTime`.
    pub fn get_mute_time(&self) -> i64 {
        self.inner.lock().mute_time
    }

    /// Mirrors `getAuthenticateId`.
    pub fn get_authenticate_id(&self) -> i32 {
        self.inner.lock().authenticate_id
    }

    /// Mirrors `setAuthenticateId(int)`.
    pub fn set_authenticate_id(&self, authenticate_id: i32) {
        self.inner.lock().authenticate_id = authenticate_id;
    }

    /// Mirrors `getAuthenticateTelporterId`.
    pub fn get_authenticate_teleporter_id(&self) -> i32 {
        self.inner.lock().authenticate_teleporter_id
    }

    /// Mirrors `setAuthenticateTelporterId(int)`.
    pub fn set_authenticate_teleporter_id(&self, authenticate_teleporter_id: i32) {
        self.inner.lock().authenticate_teleporter_id = authenticate_teleporter_id;
    }

    /// Mirors `getPendingTeleporterId`.
    pub fn get_pending_teleporter_id(&self) -> i32 {
        self.inner.lock().pending_teleporter_id
    }

    /// Mirors `setPendingTeleporterId(int)`.
    pub fn set_pending_teleporter_id(&self, pending_teleporter_id: i32) {
        self.inner.lock().pending_teleporter_id = pending_teleporter_id;
    }

    /// Mirors `getQueuedTeleporterId`.
    pub fn get_queued_teleporter_id(&self) -> i32 {
        self.inner.lock().queued_teleporter_id
    }

    /// Mirors `setQueuedTeleporterId(int)`.
    pub fn set_queued_teleporter_id(&self, queued_teleporter_id: i32) {
        self.inner.lock().queued_teleporter_id = queued_teleporter_id;
    }

    /// Mirors `isTyping`.
    pub fn is_typing(&self) -> bool {
        self.inner.lock().is_typing
    }

    /// Mirors `setTyping(boolean)`.
    pub fn set_typing(&self, typing: bool) {
        self.inner.lock().is_typing = typing;
    }

    /// Mirors `isDiving`.
    pub fn is_diving(&self) -> bool {
        self.inner.lock().is_diving
    }

    /// Mirors `setDiving(boolean)`.
    pub fn set_diving(&self, diving: bool) {
        self.inner.lock().is_diving = diving;
    }

    /// Mirors `getGamePlayer`.
    pub fn get_game_player(&self) -> Option<Arc<Mutex<GamePlayer>>> {
        self.inner.lock().game_player.clone()
    }

    /// Mirors `setGamePlayer(GamePlayer)`.
    pub fn set_game_player(&self, game_player: Arc<Mutex<GamePlayer>>) {
        self.inner.lock().game_player = Some(game_player);
    }

    /// Mirrors the `setGamePlayer(null)` calls (the Java `RoomPlayer`
    // method with a `null` argument).
    pub fn clear_game_player(&self) {
        self.inner.lock().game_player = None;
    }

    /// Mirors `getCurrentGameId`.
    pub fn get_current_game_id(&self) -> Option<String> {
        self.inner.lock().current_game_id.clone()
    }

    /// Mirors `setCurrentGameId(String)`.
    pub fn set_current_game_id(&self, current_game_id: Option<String>) {
        self.inner.lock().current_game_id = current_game_id;
    }

    /// Mirors `getObservingGameId`.
    pub fn get_observing_game_id(&self) -> i32 {
        self.inner.lock().observing_game_id
    }

    /// Mirors `setObservingGameId(int)`.
    pub fn set_observing_game_id(&self, observing_game_id: i32) {
        self.inner.lock().observing_game_id = observing_game_id;
    }

    /// Mirrors `stopObservingGame()`.
    pub fn stop_observing_game(&self, player: &Arc<Mutex<Player>>) {
        if self.get_observing_game_id() != -1 {
            if let Some(game) =
                crate::game::games::game_manager::GameManager::get_instance()
                    .get_game_by_id(self.get_observing_game_id())
            {
                game.remove_observer(player);
            }
        }
    }

    /// Mirors `getLidoVote`.
    pub fn get_lido_vote(&self) -> i32 {
        self.inner.lock().lido_vote
    }

    /// Mirors `setLidoVote(int)`.
    pub fn set_lido_vote(&self, lido_vote: i32) {
        self.inner.lock().lido_vote = lido_vote;
    }

    /// Mirrors `getTimerManager().resetRoomTimer()` (the `RoomTimerManager`
    /// is not embedded to avoid an `Arc` cycle; the timers are reset here).
    pub fn reset_room_timer(&self) {
        let mut s = self.inner.lock();
        s.look_timer = 0;
        s.chat_spam_ticks = 16;
    }

    /// Mirrors `getTimerManager().beginChatBubbleTimer()`.
    pub fn begin_chat_bubble_timer(&self) {
        let timeout = GameConfiguration::get_instance()
            .get_integer("chat.bubble.timeout.seconds");

        if timeout > 0 {
            let now = crate::util::date_util::DateUtil::get_current_time_seconds() as i64;
            let mut s = self.inner.lock();
            s.chat_bubble_timer = now + timeout as i64;
        }
    }

    /// Mirrors `getTimerManager().stopChatBubbleTimer()`.
    pub fn stop_chat_bubble_timer(&self) {
        self.inner.lock().chat_bubble_timer = -1;
    }

    /// Mirrors `getTimerManager().getChatBubbleTimer()`.
    pub fn get_chat_bubble_timer(&self) -> i64 {
        self.inner.lock().chat_bubble_timer
    }

    /// Mirrors `getTimerManager().getLookTimer()`.
    pub fn get_look_timer(&self) -> i32 {
        self.inner.lock().look_timer
    }

    /// Mirrors `getTimerManager().beginLookTimer()`.
    pub fn begin_look_timer(&self) {
        let now = crate::util::date_util::DateUtil::get_current_time_seconds();
        self.inner.lock().look_timer = now + 6;
    }

    /// Mirrors `getTimerManager().stopLookTimer()`.
    pub fn stop_look_timer(&self) {
        self.inner.lock().look_timer = -1;
    }

    /// Mirors `refreshAppearance`.
    pub fn refresh_appearance(&self) {
        let player_id = {
            let state = self.inner.lock();
            state
                .entity
                .as_ref()
                .and_then(|e| e.as_player())
                .map(|player| player.get_details().get_id())
        };
        let Some(player_id) = player_id else {
            return;
        };

        // Java reloads figure/sex/motto from `PlayerDao` and sends
        // `USER_OBJECT` + `FIGURE_CHANGE`; `PlayerDao.getDetails` and the
        // `USER_OBJECT` / `FIGURE_CHANGE` composers are not ported.

        if let Some(room) = self.get_room() {
            if let Some(trigger) = room.get_model().and_then(|model| model.get_room_trigger()) {
                if let Some(lobby_trigger) = trigger.as_game_lobby() {
                    // The Java `showPoints` uses the owning `Player`;
                    // it is resolved back to the Arc via the entity
                    // manager.
                    let player_arc = room
                        .get_entity_manager()
                        .get_players()
                        .into_iter()
                        .find(|p| p.lock().get_details().get_id() == player_id);

                    if let Some(player_arc) = player_arc {
                        lobby_trigger.show_points(&player_arc, &room);
                    }
                }
            }
        }
    }

    /// Mirrors the `RoomPet.createTask` state store (the Java `PetTask`
    /// is held by the `RoomPet`; Rust has no subtyping, so the base
    /// entity holds it).
    pub fn set_task(&self, task: crate::game::room::tasks::pet_task::PetTask) {
        self.inner.lock().task = Some(task);
    }

    /// Mirors `getTask()` (the guard is handed out because the `task` is
    /// mutated through it).
    pub fn task_guard(&self) -> parking_lot::MutexGuard<'_, RoomEntityState> {
        self.inner.lock()
    }
}

impl Default for RoomEntity {
    fn default() -> Self {
        Self::new(None)
    }
}

impl fmt::Debug for RoomEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RoomEntity")
            .field("instance_id", &self.get_instance_id())
            .finish()
    }
}
