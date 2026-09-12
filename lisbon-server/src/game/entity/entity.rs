//! Mirrors `net.h4bbo.lisbon.game.entity.Entity`.
//!
//! The Java class is pure abstract with only abstract methods, so it maps to
//! a plain Rust trait.

use crate::game::bot::bot::Bot;
use crate::game::entity::entity_type::EntityType;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::pets::pet::Pet;
use crate::game::player::player::Player;
use crate::game::player::player_details::PlayerDetails;
use crate::game::room::entities::room_entity::RoomEntity;

/// Mirrors the abstract `Entity` base class.
pub trait Entity {
    /// Checks for permission.
    fn has_fuse(&self, permission: &Fuseright) -> bool;

    /// Gets the details.
    fn get_details(&self) -> &PlayerDetails;

    /// Gets the room user.
    fn get_room_user(&self) -> Option<&RoomEntity>;

    /// Gets the type.
    fn get_type(&self) -> EntityType;

    /// Dispose.
    fn dispose(&mut self);

    /// Gets the entity as a `Player` (`None` for the other entity types),
    /// mirroring the Java `instanceof Player` cast.
    fn as_player(&self) -> Option<&Player> {
        None
    }

    /// Gets the entity as a mutable `Player` (`None` for the other entity
    /// types), mirroring the Java `instanceof Player` cast when the
    /// command mutates the player.
    fn as_player_mut(&mut self) -> Option<&mut Player> {
        None
    }

    /// Gets the entity as a `Pet` (`None` for the other entity types),
    /// mirroring the Java `instanceof Pet` cast.
    fn as_pet(&self) -> Option<&Pet> {
        None
    }

    /// Gets the entity as a `Bot` (`None` for the other entity types),
    /// mirroring the Java `instanceof Bot` cast.
    fn as_bot(&self) -> Option<&Bot> {
        None
    }
}
