//! Mirrors `net.h4bbo.lisbon.game.item.roller.RollingAnalysis`.
use crate::game::item::item::Item;
use crate::game::pathfinder::position::Position;
use crate::game::room::room::Room;

/// Mirrors `RollingAnalysis<T>`.
pub trait RollingAnalysis<T> {
    /// Mirrors `canRoll(T, Item, Room)`.
    fn can_roll(&self, target: &mut T, roller: &Item, room: &Room) -> Option<Position>;

    /// Mirrors `doRoll(T, Item, Room, Position, Position)`.
    fn do_roll(
        &self,
        target: &mut T,
        roller: &Item,
        room: &Room,
        from_position: &Position,
        next_position: &mut Position,
    );
}
