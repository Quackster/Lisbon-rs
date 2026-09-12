//! Mirrors `net.h4bbo.lisbon.game.entity.EntityType`.
//!
//! The Java enum carries a `Class<? extends Entity>` per variant; the
//! reflection-based class lookup is dropped here since the `Entity` class
//! hierarchy is not yet ported. Only the discriminant is mirrored.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EntityType {
    Player,
    Pet,
    Bot,
}
