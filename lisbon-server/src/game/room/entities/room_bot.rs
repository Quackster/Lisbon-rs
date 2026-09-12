//! Mirrors `net.h4bbo.lisbon.game.room.entities.RoomBot`.
//!
//! Java `RoomBot extends RoomEntity`; Rust has no subtyping, so the base
//! entity is composed.
use crate::game::room::entities::room_entity::RoomEntity;

#[derive(Clone)]
pub struct RoomBot {
    pub entity: RoomEntity,
}

impl RoomBot {
    /// Mirrors the `RoomBot(Entity)` constructor (the `Entity` back-reference
    /// is omitted; the `Bot` owns this instance).
    pub fn new() -> Self {
        Self {
            entity: RoomEntity::default(),
        }
    }
}
