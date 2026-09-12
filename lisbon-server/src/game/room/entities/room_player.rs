//! Mirrors `net.h4bbo.lisbon.game.room.entities.RoomPlayer`.
//!
//! Java `RoomPlayer extends RoomEntity`; Rust has no subtyping, so the base
//! entity is composed. The player-specific state (teleporter ids, game
//! player, typing/diving, trade, spam timers) lives on the composed
//! `RoomEntity` (see `room_entity.rs`), since `Player` hands out
//! `&RoomEntity` via `get_room_user()` and callers call those methods on it.
//! The Java `Player` back-reference is omitted (the `Player` owns this
//! `RoomPlayer`, mirroring how the `Room` back-reference is omitted).
use crate::game::room::entities::room_entity::RoomEntity;

#[derive(Clone)]
pub struct RoomPlayer {
    pub entity: RoomEntity,
}

impl RoomPlayer {
    /// Mirrors the `RoomPlayer(Player)` constructor (the `Player`
    /// back-reference is omitted; the `Player` owns this instance).
    pub fn new() -> Self {
        Self {
            entity: RoomEntity::default(),
        }
    }
}

impl Default for RoomPlayer {
    fn default() -> Self {
        Self::new()
    }
}
