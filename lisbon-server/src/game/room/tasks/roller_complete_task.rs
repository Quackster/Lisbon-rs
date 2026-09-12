//! Mirrors `net.h4bbo.lisbon.game.room.tasks.RollerCompleteTask`.
use crate::game::entity::entity::Entity;
use crate::game::item::item::Item;
use crate::game::room::room::Room;

/// Mirrors `RollerCompleteTask` (the Java `Runnable` is the `run`
/// method).
pub struct RollerCompleteTask {
    // The Java `run()` body that consumed `room`
    // (`this.room.send(new MOVE_FLOORITEM(item))`) is commented out.
    #[allow(dead_code)]
    room: Room,
    rolling_items: Vec<Item>,
    rolling_entities: Vec<Box<dyn Entity + Send>>,
}

impl RollerCompleteTask {
    /// Mirrors the `(Collection<Item>, Set<Entity>, Room)` constructor.
    pub fn new(
        rolling_items: Vec<Item>,
        rolling_entities: Vec<Box<dyn Entity + Send>>,
        room: &Room,
    ) -> Self {
        Self {
            room: room.clone(),
            rolling_items,
            rolling_entities,
        }
    }

    /// Mirrors `run()` (the Java `getRoomUser()` NPE is an early
    // `continue` here).
    pub fn run(&mut self) {
        for item in &mut self.rolling_items {
            if item.get_rolling_data().is_none() {
                continue;
            }

            item.set_current_roll_blocked(false);
            item.set_rolling_data(None);
        }

        for entity in &self.rolling_entities {
            let entity = entity.as_ref();

            let Some(room_user) = entity.get_room_user() else {
                continue;
            };

            if room_user.get_rolling_data().is_none() {
                continue;
            }

            room_user.invoke_item(None, false);
            room_user.set_rolling_data(None);
        }
    }
}
