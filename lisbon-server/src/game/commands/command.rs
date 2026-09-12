//! Mirrors `net.h4bbo.lisbon.game.commands.Command`.
//!
//! The Java class is abstract with a shared `permissions` / `arguments`
//! list populated by `addPermissions` / `addArguments` in the constructor,
//! so it maps to a trait whose implementors carry the two lists.

use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;

/// Mirrors the abstract `Command` base class.
pub trait Command: Send + Sync {
    /// Mirrors the no-arg constructor (calls `addPermissions` and
    /// `addArguments`).
    fn new() -> Self
    where
        Self: Sized;

    /// Mirrors `addPermissions()`.
    fn add_permissions(&mut self);

    /// Mirrors `addArguments()` (the Java default is a no-op).
    fn add_arguments(&mut self) {}

    /// Mirrors `handleCommand(Entity, String, String[])`.
    fn handle_command(&self, entity: &mut dyn Entity, message: &str, args: &[String]);

    /// Mirrors `getDescription()`.
    fn get_description(&self) -> String;

    /// Mirrors `getPermissions()`.
    fn get_permissions(&self) -> Vec<Fuseright>;

    /// Mirrors `getArguments()`.
    fn get_arguments(&self) -> Vec<String>;
}
