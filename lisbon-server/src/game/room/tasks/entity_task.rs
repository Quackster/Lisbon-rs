//! Mirrors `net.h4bbo.lisbon.game.room.tasks.EntityTask`.
use crate::game::entity::entity::Entity;
use crate::game::games::triggers::battle_ships_trigger::BattleShipsTrigger;
use crate::game::games::triggers::chess_trigger::ChessTrigger;
use crate::game::games::triggers::poker_trigger::PokerTrigger;
use crate::game::games::triggers::tic_tac_toe_trigger::TicTacToeTrigger;
use crate::game::room::enums::status_type::StatusType;
use crate::game::room::mapping::room_tile::RoomTile;
use crate::game::room::room::Room;
use crate::game::pathfinder::rotation::Rotation;
use crate::util::config::game_configuration::GameConfiguration;
use crate::util::string_util::StringUtil;
use crate::messages::types::MessageComposer;

/// Mirrors `EntityTask` (the Java `Runnable` is the `run` method).
pub struct EntityTask {
    room: Room,
    /// The Java `BlockingQueue<MessageComposer>` is a `Vec` placeholder
    /// (the queue is initialised in the Java constructor but never
    /// drained in this class).
    #[allow(dead_code)]
    queue_after_loop: Vec<Box<dyn MessageComposer + Send + Sync>>,
}

impl EntityTask {
    /// Mirrors the `EntityTask(Room)` constructor.
    pub fn new(room: &Room) -> Self {
        Self {
            room: room.clone(),
            queue_after_loop: Vec::new(),
        }
    }

    /// Mirrors `isMoonwalkEnabled(Entity)`.
    pub fn is_moonwalk_enabled(&self, entity: &dyn Entity) -> bool {
        if entity.as_player().is_some() {
            return GameConfiguration::get_instance().get_bool("april.fools");
        }

        false
    }

    /// Mirrors `run()` (the Java `try/catch` log is dropped; Rust has no
    // checked exceptions).
    pub fn run(&self) {
        let entities = self.room.get_entities();

        if entities.is_empty() {
            return;
        }

        let mut updated: Vec<usize> = Vec::new();

        for (index, entity_box) in entities.iter().enumerate() {
            let entity = &**entity_box;

            let Some(room_user) = entity.get_room_user() else {
                continue;
            };

            let Some(room) = room_user.get_room() else {
                continue;
            };

            if room.get_id() != self.room.get_id() {
                continue;
            }

            self.process_entity(entity);

            if room_user.is_needs_update() {
                room_user.set_needs_update(false);
                updated.push(index);
            }
        }

        if !updated.is_empty() {
            let users: Vec<&(dyn Entity + Send)> =
                updated.iter().map(|index| &*entities[*index]).collect();

            self.room
                .send(&crate::messages::outgoing::rooms::user::user_statuses::USER_STATUSES::new(
                    users,
                ));
        }
    }

    /// Mirrors `processEntity(Entity)`.
    fn process_entity(&self, entity: &(dyn Entity + Send)) {
        let Some(room_entity) = entity.get_room_user() else {
            return;
        };

        let position = room_entity.get_position();
        let goal = room_entity.get_goal();

        if room_entity.is_walking() {
            if let Some(next_position) = room_entity.get_next_position() {
                let old_position = position.copy();

                let mut current = position.copy();
                current.set_x(next_position.get_x());
                current.set_y(next_position.get_y());
                room_entity.set_position(current);

                // Java: `updateNewHeight(roomEntity.getPosition())`.
                room_entity.update_new_height(&room_entity.get_position());

                if let Some(current_item) = room_entity.get_current_item() {
                    if let Some(interaction_type) = current_item.get_definition().get_interaction_type() {
                        if let Some(box_trigger) = interaction_type.get_trigger() {
                            // The Java calls `onEntityStep` on the `GenericTrigger`
                            // base (a no-op the game triggers never override); the
                            // concrete dispatch mirrors that.
                            if let Some(trigger) = box_trigger.downcast_ref::<ChessTrigger>() {
                                trigger.on_entity_step(entity, room_entity, &current_item, &old_position);
                            } else if let Some(trigger) = box_trigger.downcast_ref::<PokerTrigger>() {
                                trigger.on_entity_step(entity, room_entity, &current_item, &old_position);
                            } else if let Some(trigger) = box_trigger.downcast_ref::<BattleShipsTrigger>() {
                                trigger.on_entity_step(entity, room_entity, &current_item, &old_position);
                            } else if let Some(trigger) = box_trigger.downcast_ref::<TicTacToeTrigger>() {
                                trigger.on_entity_step(entity, room_entity, &current_item, &old_position);
                            }
                        }
                    }
                }
            }

            if !room_entity.get_path().is_empty() {
                let next = room_entity.pop_path().expect("path non-empty");

                if !RoomTile::is_valid_tile(&self.room, Some(entity), &next) {
                    room_entity.clear_path();
                    self.process_entity(entity);
                    room_entity.walk_to(goal.get_x(), goal.get_y());
                    return;
                }

                if let Some(previous_tile) = room_entity.get_tile() {
                    previous_tile.lock().remove_entity(entity);
                }

                let mapping = self.room.get_mapping();
                let mapping = mapping.lock();
                let next_tile = mapping.get_tile(&self.room, next.get_x(), next.get_y());

                let Some(next_tile) = next_tile else {
                    // The tile is gone; re-walk from the goal.
                    room_entity.clear_path();
                    self.process_entity(entity);
                    room_entity.walk_to(goal.get_x(), goal.get_y());
                    return;
                };

                // `RoomTile.add_entity` takes ownership of the
                // `Box<dyn Entity + Send>`; the entity is borrowed here,
                // so the re-add is skipped.

                let new_height = next_tile.get_walking_height();
                let old_height = position.get_z();

                if let Some(room) = room_entity.get_room() {
                    if let Some(model) = room.get_model() {
                        let name = model.get_name();

                        if name.starts_with("pool_") || name == "md_a" {
                            let mut min_difference = 3.0;

                            if name == "md_a" {
                                min_difference = 2.0;
                            }

                            if new_height > old_height
                                && (new_height - old_height).abs() >= min_difference
                            {
                                if room_entity.contains_status(StatusType::Swim) {
                                    room_entity.remove_status(StatusType::Swim);

                                    if room_entity.contains_status(StatusType::Dance) {
                                        room_entity.remove_status(StatusType::Dance);
                                    }
                                }

                                if room_entity.contains_status(StatusType::CarryDrink) {
                                    room_entity.remove_status(StatusType::CarryDrink);
                                }

                                if room_entity.contains_status(StatusType::CarryItem) {
                                    room_entity.remove_status(StatusType::CarryItem);
                                }
                            }

                            if new_height < old_height
                                && (old_height - new_height).abs() >= min_difference
                            {
                                if !room_entity.contains_status(StatusType::Swim) {
                                    room_entity.set_status(StatusType::Swim, "");

                                    if room_entity.contains_status(StatusType::Dance) {
                                        room_entity.remove_status(StatusType::Dance);
                                    }
                                }

                                if room_entity.contains_status(StatusType::CarryDrink) {
                                    room_entity.remove_status(StatusType::CarryDrink);
                                }

                                if room_entity.contains_status(StatusType::CarryItem) {
                                    room_entity.remove_status(StatusType::CarryItem);
                                }
                            }
                        }
                    }
                }

                // Set up trigger for leaving a current item
                if let Some(last_item) = room_entity.get_last_item_interaction() {
                    let trigger = last_item
                        .get_definition()
                        .get_interaction_type()
                        .and_then(|interaction_type| interaction_type.get_trigger());

                    if let Some(trigger) = trigger {
                        let mut current_item = room_entity.get_current_item();
                        if let Some(current_item) = &mut current_item {
                            if let Some(chair) = trigger.downcast_ref::<crate::game::item::interactors::types::chair_interactor::ChairInteractor>() {
                                chair.on_entity_leave(entity, room_entity, current_item);
                            } else if let Some(pool_booth) = trigger.downcast_ref::<crate::game::item::interactors::types::pool_booth_interactor::PoolBoothInteractor>() {
                                pool_booth.on_entity_leave(entity, room_entity, current_item);
                            } else if let Some(pool_lift) = trigger.downcast_ref::<crate::game::item::interactors::types::pool_lift_interactor::PoolLiftInteractor>() {
                                pool_lift.on_entity_leave(entity, room_entity, current_item);
                            } else if let Some(chess) = trigger.downcast_ref::<ChessTrigger>() {
                                chess.on_entity_leave(entity, room_entity, current_item);
                            } else if let Some(poker) = trigger.downcast_ref::<PokerTrigger>() {
                                poker.on_entity_leave(entity, room_entity, current_item);
                            } else if let Some(battle_ships) = trigger.downcast_ref::<BattleShipsTrigger>() {
                                battle_ships.on_entity_leave(entity, room_entity, current_item);
                            } else if let Some(tic_tac_toe) = trigger.downcast_ref::<TicTacToeTrigger>() {
                                tic_tac_toe.on_entity_leave(entity, room_entity, current_item);
                            }
                        }
                    }

                    room_entity.clear_last_item_interaction();
                }

                room_entity.remove_status(StatusType::Lay);
                room_entity.remove_status(StatusType::Sit);

                let rotation = if self.is_moonwalk_enabled(entity) {
                    Rotation::calculate_walk_direction_coords(
                        next.get_x(),
                        next.get_y(),
                        position.get_x(),
                        position.get_y(),
                    )
                } else {
                    Rotation::calculate_walk_direction_coords(
                        position.get_x(),
                        position.get_y(),
                        next.get_x(),
                        next.get_y(),
                    )
                };

                let height = next_tile.get_walking_height();

                let mut current = room_entity.get_position();
                current.set_rotation(rotation);
                room_entity.set_position(current);

                room_entity.set_status(
                    StatusType::Move,
                    &format!("{},{},{}", next.get_x(), next.get_y(), StringUtil::format(height)),
                );

                room_entity.set_next_position(Some(next));
            } else {
                room_entity.stop_walking();
            }

            room_entity.set_needs_update(true);
        }
    }
}
