//! Mirrors `net.h4bbo.lisbon.game.item.interactors.types.wobblesquabble.WobbleSquabbleQueueTile`.
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::games::wobblesquabble::wobble_squabble_manager::WobbleSquabbleManager;
use crate::game::item::item::Item;
use crate::game::room::entities::room_entity::RoomEntity;
use crate::game::triggers::generic_trigger::GenericTrigger;

pub struct WobbleSquabbleQueueTile {
    #[allow(dead_code)]
    trigger: GenericTrigger,
}

impl WobbleSquabbleQueueTile {
    /// Mirrors the `WobbleSquabbleQueueTile()` constructor.
    pub fn new() -> Self {
        Self {
            #[allow(dead_code)]
    trigger: GenericTrigger,
        }
    }

    /// Mirrors `onEntityStop(Entity, RoomEntity, Item, boolean)`.
    pub fn on_entity_stop(
        &self,
        entity: &(dyn Entity + Send),
        room_entity: &RoomEntity,
        _item: &Item,
        _is_rotation: bool,
    ) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        if let Some(room) = room_entity.get_room() {
            if room
                .get_task_manager()
                .has_task(WobbleSquabbleManager::get_instance().get_name())
            {
                return;
            }
        }

        let position = room_entity.get_position();
        let front = position.get_square_in_front();
        room_entity.walk_to(front.get_x(), front.get_y());
    }
}

impl Default for WobbleSquabbleQueueTile {
    fn default() -> Self {
        Self::new()
    }
}
