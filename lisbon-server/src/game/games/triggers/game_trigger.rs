//! Mirrors `net.h4bbo.lisbon.game.games.triggers.GameTrigger`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::games::gamehalls::gamehall_game::GamehallGameHandle;
use crate::game::item::interactors::interaction_type::InteractionType;
use crate::game::item::interactors::types::chair_interactor::ChairInteractor;
use crate::game::item::item::Item;
use crate::game::pathfinder::position::Position;
use crate::game::player::player_manager::PlayerManager;
use crate::game::room::entities::room_entity::RoomEntity;
use crate::messages::outgoing::rooms::games::close_game_board::CLOSEGAMEBOARD;
use crate::messages::outgoing::rooms::games::open_game_board::OPENGAMEBOARD;
use crate::messages::types::MessageComposer;

/// Mirrors the abstract `GameTrigger` class.
///
/// The game instances sit behind their own mutexes (the Java list is
/// unsynchronised; the per-instance guard is what the ported callers
/// need to mutate a `GamehallGame`).
pub struct GameTrigger {
    game_instances: Vec<Mutex<Box<dyn GamehallGameHandle>>>,
}

impl GameTrigger {
    /// Mirrors the `GameTrigger` constructor.
    pub fn new() -> Self {
        Self {
            game_instances: Vec::new(),
        }
    }

    /// Mirrors `getGameInstances().add(...)` (only used before the
    /// singleton is shared).
    pub fn add_game_instance(&mut self, instance: Box<dyn GamehallGameHandle>) {
        self.game_instances.push(Mutex::new(instance));
    }

    /// Mirrors `onEntityStep(Entity, RoomEntity, Item, Position)`.
    pub fn on_entity_step(
        &self,
        _entity: &dyn Entity,
        _room_entity: &RoomEntity,
        _item: &Item,
        _old_position: &Position,
    ) {
    }

    /// Mirrors `onEntityStop(Entity, RoomEntity, Item, boolean)`.
    pub fn on_entity_stop(
        &self,
        entity: &dyn Entity,
        room_entity: &RoomEntity,
        item: &Item,
        is_rotation: bool,
    ) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        let Some(player) = entity.as_player() else {
            return;
        };

        // Call default sitting trigger (the Java passes the `Entity`; the
        // ported interactor takes the `+ Send` bound the `Player` satisfies).
        if let Some(trigger) = InteractionType::Chair.get_trigger() {
            if let Some(chair_trigger) = trigger.downcast_ref::<ChairInteractor>() {
                chair_trigger.on_entity_stop(player, room_entity, item, is_rotation);
            }
        }

        // Handle game logic from here.
        let mut instance = match self.get_game_instance(item.get_position()) {
            Some(instance) => instance,
            None => {
                // The Java NPEs when the instance is missing.
                return;
            }
        };

        if !(instance.base().get_room_id() > 0) {
            // The Java NPEs when the room is gone.
            if let Some(room) = room_entity.get_room() {
                instance.set_room_id(room.get_id());
            }
        }

        let joined_players = instance.refresh_players();

        if instance.get_game_id().is_some() {
            if instance.get_players().len() as i32 >= instance.get_minimum_people_required() {
                for p in joined_players {
                    p.lock().send(&OPENGAMEBOARD::new(
                        instance.get_game_id().as_deref().unwrap_or(""),
                        &instance.get_game_fuse_type(),
                    )); // Player joined mid-game
                }
            }
        }

        if instance.get_game_id().is_none() {
            if instance.has_players_required() {
                // New game started
                instance.create_game_id();
                instance.game_start();
                let message: Arc<dyn MessageComposer + Send + Sync> =
                    Arc::new(OPENGAMEBOARD::new(
                        instance.get_game_id().as_deref().unwrap_or(""),
                        &instance.get_game_fuse_type(),
                    ));
                instance.send_to_everyone(&message);
            }
        }
    }

    /// Mirrors `onEntityLeave(Entity, RoomEntity, Item)`.
    pub fn on_entity_leave(
        &self,
        entity: &dyn Entity,
        _room_entity: &RoomEntity,
        item: &Item,
    ) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        let Some(player) = entity.as_player() else {
            return;
        };

        // The Java NPEs when there is no room user.
        let Some(room_user) = player.get_room_user() else {
            return;
        };

        let Some(mut instance) = self.get_game_instance(item.get_position()) else {
            return;
        };

        if room_user.get_current_game_id().is_none() {
            return;
        }

        if instance.get_game_id().is_some() {
            // If game has started
            let new_player_count = instance.get_players().len() as i32 - 1;

            if new_player_count >= instance.get_minimum_people_required() {
                player.send(&CLOSEGAMEBOARD::new(
                    instance.get_game_id().as_deref().unwrap_or(""),
                    &instance.get_game_fuse_type(),
                ));
            } else {
                let message: Arc<dyn MessageComposer + Send + Sync> =
                    Arc::new(CLOSEGAMEBOARD::new(
                        instance.get_game_id().as_deref().unwrap_or(""),
                        &instance.get_game_fuse_type(),
                    ));
                instance.send_to_everyone(&message);
            }
        }

        // The Java `getPlayers().remove(player)` is an identity remove;
        // the shared handle is looked up for the `Arc` identity.
        if let Some(player_arc) =
            PlayerManager::get_instance().get_player_by_id(player.get_details().get_id())
        {
            instance.remove_player(&player_arc);
        }

        room_user.set_current_game_id(None);

        if !instance.has_players_required() {
            instance.reset_game_id();
            instance.game_stop();
        }
    }

    /// Gets the game instance on this specified position.
    pub fn get_game_instance(
        &self,
        position: &Position,
    ) -> Option<parking_lot::MutexGuard<'_, Box<dyn GamehallGameHandle>>> {
        for slot in &self.game_instances {
            let instance = slot.lock();
            if instance.base().get_chair_coordinates().iter().any(|coordinate| {
                position == &Position::new_xy(coordinate[0], coordinate[1])
            }) {
                return Some(instance);
            }
        }
        None
    }

    /// Get all game instances.
    pub fn get_game_instances(&self) -> &[Mutex<Box<dyn GamehallGameHandle>>] {
        &self.game_instances
    }
}
