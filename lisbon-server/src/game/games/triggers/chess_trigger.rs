//! Mirrors `net.h4bbo.lisbon.game.games.triggers.ChessTrigger`.
//!
//! Rust has no inheritance; the trigger embeds the `GameTrigger` base
//! and registers its concrete games on the base `game_instances` list
//! (the Java constructor adds them to `getGameInstances()`).
use parking_lot::{Mutex, MutexGuard};

use crate::game::entity::entity::Entity;
use crate::game::games::gamehalls::game_chess::GameChess;
use crate::game::games::gamehalls::gamehall_game::GamehallGameHandle;
use crate::game::games::triggers::game_trigger::GameTrigger;
use crate::game::item::item::Item;
use crate::game::pathfinder::position::Position;
use crate::game::room::entities::room_entity::RoomEntity;

pub struct ChessTrigger {
    base: GameTrigger,
}

impl ChessTrigger {
    /// Mirrors the `ChessTrigger` constructor.
    pub fn new() -> Self {
        let mut base = GameTrigger::new();

        for chair_group in Self::get_chair_groups() {
            base.add_game_instance(Box::new(GameChess::new(chair_group)));
        }

        Self { base }
    }

    /// Mirrors `onEntityStep(Entity, RoomEntity, Item, Position)` (delegates
    /// to `super`).
    pub fn on_entity_step(
        &self,
        entity: &dyn Entity,
        room_entity: &RoomEntity,
        item: &Item,
        old_position: &Position,
    ) {
        self.base.on_entity_step(entity, room_entity, item, old_position)
    }

    /// Mirrors `onEntityStop(Entity, RoomEntity, Item, boolean)` (delegates
    /// to `super`).
    pub fn on_entity_stop(
        &self,
        entity: &dyn Entity,
        room_entity: &RoomEntity,
        item: &Item,
        is_rotation: bool,
    ) {
        self.base.on_entity_stop(entity, room_entity, item, is_rotation)
    }

    /// Mirrors `onEntityLeave(Entity, RoomEntity, Item)` (delegates
    /// to `super`).
    pub fn on_entity_leave(
        &self,
        entity: &dyn Entity,
        room_entity: &RoomEntity,
        item: &Item,
    ) {
        self.base.on_entity_leave(entity, room_entity, item)
    }

    /// Mirrors `getChairGroups()` (an associated function since the Java
    /// constructor calls it before the instance exists).
    pub fn get_chair_groups() -> Vec<Vec<[i32; 2]>> {
        vec![
            vec![[2, 7], [2, 9]],
            vec![[6, 14], [4, 14]],
            vec![[12, 14], [12, 12]],
            vec![[13, 7], [13, 5]],
            vec![[7, 3], [9, 3]],
        ]
    }

    /// Mirrors `getGameInstance(Position)`.
    pub fn get_game_instance(
        &self,
        position: &Position,
    ) -> Option<MutexGuard<'_, Box<dyn GamehallGameHandle>>> {
        self.base.get_game_instance(position)
    }

    /// Mirrors `getGameInstances()`.
    pub fn get_game_instances(&self) -> &[Mutex<Box<dyn GamehallGameHandle>>] {
        self.base.get_game_instances()
    }
}
