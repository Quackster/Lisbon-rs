//! Mirrors `net.h4bbo.lisbon.game.triggers.GenericTrigger`.
//!
//! The Java `GenericTrigger` is an abstract class that simply holds a
//! `Room`; the interactors store a `GenericTrigger` field the same way.
//! It is mirrored here as a unit struct alongside the `Trigger` trait
//! (the room-trigger interface).

/// Mirrors the `GenericTrigger` field used by the item interactors.
pub struct GenericTrigger;

/// Mirrors the trigger callbacks (the Java `GenericTrigger` is the base
/// class for the room triggers).
///
/// The Java `GenericTrigger` is an abstract class; the concrete triggers
/// override the callbacks they need. The trait object is stored by the
/// `RoomModel`.
pub trait Trigger: Send + Sync {
    /// Mirrors `onRoomEntry(Entity, Room, boolean, Object...)`.
    fn on_room_entry(
        &self,
        _entity: &dyn crate::game::entity::entity::Entity,
        _room: &crate::game::room::room::Room,
        _first_entry: bool,
        _custom_args: &[Box<dyn std::any::Any>],
    ) {}

    /// Mirrors `onRoomLeave(Entity, Room, Object...)`.
    fn on_room_leave(
        &self,
        _entity: &dyn crate::game::entity::entity::Entity,
        _room: &crate::game::room::room::Room,
        _custom_args: &[Box<dyn std::any::Any>],
    ) {}

    /// Mirrors `onEntityMove`.
    fn on_entity_move(
        &self,
        _entity: &dyn crate::game::entity::entity::Entity,
        _position: &crate::game::pathfinder::position::Position,
        _room: &crate::game::room::room::Room,
    ) {}

    /// Mirrors `onEntityLeave`.
    fn on_entity_leave(
        &self,
        _entity: &dyn crate::game::entity::entity::Entity,
        _room_entity: &crate::game::room::entities::room_entity::RoomEntity,
        _item: &crate::game::item::item::Item,
    ) {}

    /// Mirrors `onEntityStop`.
    fn on_entity_stop(
        &self,
        _entity: &dyn crate::game::entity::entity::Entity,
        _room_entity: &crate::game::room::entities::room_entity::RoomEntity,
        _item: &crate::game::item::item::Item,
        _is_rotation: bool,
    ) {}

    /// Mirrors `onEntityUpdate`.
    fn on_entity_update(
        &self,
        _entity: &dyn crate::game::entity::entity::Entity,
        _room_entity: &crate::game::room::entities::room_entity::RoomEntity,
        _item: &crate::game::item::item::Item,
    ) {}

    /// Mirrors `onEntitySitDown`.
    fn on_entity_sit_down(
        &self,
        _entity: &dyn crate::game::entity::entity::Entity,
        _room_entity: &crate::game::room::entities::room_entity::RoomEntity,
        _item: &crate::game::item::item::Item,
    ) {}

    /// Mirrors `onEntityWakeUp`.
    fn on_entity_wake_up(
        &self,
        _entity: &dyn crate::game::entity::entity::Entity,
        _room_entity: &crate::game::room::entities::room_entity::RoomEntity,
        _item: &crate::game::item::item::Item,
    ) {}
}
