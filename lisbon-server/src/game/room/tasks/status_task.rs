//! Mirrors `net.h4bbo.lisbon.game.room.tasks.StatusTask`.
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::player::player::Player;
use crate::game::room::entities::room_entity::RoomEntity;
use crate::game::room::room::Room;

/// Mirrors `StatusTask` (a `Runnable` scheduled every 1 s by the
/// `RoomTaskManager`).
pub struct StatusTask {
    room: Room,
}

impl StatusTask {
    /// Mirrors the `StatusTask(Room)` constructor.
    pub fn new(room: &Room) -> Self {
        Self {
            room: room.clone(),
        }
    }

    /// Mirrors `run()`.
    pub fn run(&self) {
        let entities = self.room.get_entity_manager().get_entities();
        if entities.is_empty() {
            return;
        }

        for entity in &entities {
            let Some(room_user) = entity.get_room_user() else {
                continue;
            };

            // The Java `getRoom() == this.room` reference check maps to
            // the room id.
            let Some(room) = room_user.get_room() else {
                continue;
            };

            if room.get_id() != self.room.get_id() {
                continue;
            }

            self.process_entity(entity, room_user);
        }
    }

    /// Mirrors `processEntity(Entity)`.
    fn process_entity(&self, entity: &Box<dyn Entity + Send>, room_user: &RoomEntity) {
        if entity.get_type() == EntityType::Player {
            if let Some(player) = entity.as_player() {
                room_user.handle_spam_ticks();
                self.process_head_rotation(room_user);
                self.process_chat_bubble(room_user);
                Self::process_pool_queue(player, room_user);
            }
        }

        if entity.get_type() == EntityType::Pet {
            if let Some(pet) = entity.as_pet() {
                let mut task_guard = pet.get_room_user_pet().task_guard();
                if let Some(task) = task_guard.task_mut() {
                    task.tick();
                }
            }
        }

        room_user.tick_statuses();
    }

    /// Mirrors the static `processPoolQueue(Player)`.
    pub fn process_pool_queue(player: &Player, room_user: &RoomEntity) {
        if room_user.get_room().is_none() {
            return;
        }

        let Some(room) = player
            .get_room_user()
            .and_then(|ru| ru.get_room())
        else {
            return;
        };
        let room_model = room.get_data().get_model().to_string();

        if room_model == "pool_b"
            && (player.get_details().get_tickets() == 0
                || player.get_details().get_pool_figure().is_empty())
        {
            return;
        }

        if room_user.is_walking() {
            return;
        }

        let Some(current_item) = room_user.get_current_item() else {
            return;
        };

        if current_item.get_definition().get_sprite() != "queue_tile2" {
            return;
        }

        let current_position = current_item.get_position();

        if room_model == "park_a" {
            if !crate::game::infobus::infobus_manager::InfobusManager::get_instance()
                .is_door_open()
            {
                return;
            }

            if current_position.copy()
                == crate::game::pathfinder::position::Position::new_xy(28, 5)
            {
                let moved_to_bus_front = player
                    .get_room_user()
                    .map(|ru| ru.walk_to(28, 4))
                    .unwrap_or(false);

                if !moved_to_bus_front {
                    if let Some(infobus_room) =
                        crate::game::room::room_manager::RoomManager::get_instance()
                            .get_room_by_model("park_b")
                    {
                        let infobus_room = infobus_room.lock().clone();
                        let entity: &(dyn crate::game::entity::entity::Entity + Send) = player;
                        infobus_room
                            .get_entity_manager()
                            .enter_room_entity(&infobus_room, entity, None);
                    }
                }

                return;
            }

            if current_position.copy()
                == crate::game::pathfinder::position::Position::new_xy(28, 4)
            {
                if let Some(infobus_room) =
                    crate::game::room::room_manager::RoomManager::get_instance()
                        .get_room_by_model("park_b")
                {
                    let infobus_room = infobus_room.lock().clone();
                    let entity: &(dyn crate::game::entity::entity::Entity + Send) = player;
                    infobus_room
                        .get_entity_manager()
                        .enter_room_entity(&infobus_room, entity, None);
                }

                return;
            }

            if let Some(next_queue_tile) =
                crate::game::infobus::infobus_manager::InfobusManager::get_instance()
                    .get_next_queue_tile(&current_position)
            {
                room_user.walk_to(next_queue_tile.get_x(), next_queue_tile.get_y());
            }

            return;
        }

        let front = current_position.get_square_in_front();
        room_user.walk_to(front.get_x(), front.get_y());
    }

    /// Mirrors the static `processChatBubble(Player)`.
    fn process_chat_bubble(&self, room_user: &RoomEntity) {
        let Some(room) = room_user.get_room() else {
            return;
        };

        if room_user.get_chat_bubble_timer() != -1
            && crate::util::date_util::DateUtil::get_current_time_seconds() as i64
                > room_user.get_chat_bubble_timer()
        {
            room_user.set_typing(false);
            room_user.stop_chat_bubble_timer();
            room.send(
                &crate::messages::outgoing::rooms::user::typing_status::TYPING_STATUS::new(
                    room_user.get_instance_id(),
                    false,
                ),
            );
        }
    }

    /// Mirrors the static `processHeadRotation(Player)`.
    fn process_head_rotation(&self, room_user: &RoomEntity) {
        if room_user.get_look_timer() != -1
            && crate::util::date_util::DateUtil::get_current_time_seconds()
                > room_user.get_look_timer()
        {
            room_user.stop_look_timer();

            let mut position = room_user.get_position();
            position.set_head_rotation(position.get_body_rotation());
            room_user.set_position(position);

            room_user.set_needs_update(true);
        }
    }
}
