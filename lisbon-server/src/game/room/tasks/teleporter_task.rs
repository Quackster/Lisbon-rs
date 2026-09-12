//! Mirrors `net.h4bbo.lisbon.game.room.tasks.TeleporterTask`.
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::item::item::Item;
use crate::game::room::room::Room;
use crate::messages::outgoing::rooms::items::broadcast_teleporter::BROADCAST_TELEPORTER;

/// Mirrors `TeleporterTask` (the Java `Runnable` is the `run` method).
pub struct TeleporterTask {
    item: Item,
    entity: Box<dyn Entity + Send>,
    room: Room,
}

impl TeleporterTask {
    /// Mirrors the `TeleporterTask(Item, Entity, Room)` constructor.
    pub fn new(
        linked_teleporter: Item,
        entity: Box<dyn Entity + Send>,
        room: &Room,
    ) -> Self {
        Self {
            item: linked_teleporter,
            entity,
            room: room.clone(),
        }
    }

    /// Mirrors `run()`.
    pub fn run(&self) {
        let entity = self.entity.as_ref();

        let Some(room_user) = entity.get_room_user() else {
            return;
        };

        room_user.warp(&self.item.get_position().copy(), true, false);

        if entity.get_type() == EntityType::Player {
            if let Some(player) = entity.as_player() {
                if let Some(player_room_user) = player.get_room_user() {
                    player_room_user.set_authenticate_teleporter_id(-1);
                }
            }
        }

        self.room.send(&BROADCAST_TELEPORTER::new(
            self.item.clone(),
            entity.get_details().get_name(),
            false,
        ));
    }
}
