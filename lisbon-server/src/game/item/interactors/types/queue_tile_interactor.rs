//! Mirrors `net.h4bbo.lisbon.game.item.interactors.types.QueueTileInteractor`.
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::item::item::Item;
use crate::game::pathfinder::position::Position;
use crate::game::room::entities::room_entity::RoomEntity;
use crate::game::room::tasks::status_task::StatusTask;
use crate::game::triggers::generic_trigger::GenericTrigger;
use crate::messages::outgoing::user::currencies::no_tickets::NO_TICKETS;

pub struct QueueTileInteractor {
    #[allow(dead_code)]
    trigger: GenericTrigger,
}

impl QueueTileInteractor {
    /// Mirrors the `QueueTileInteractor()` constructor.
    pub fn new() -> Self {
        Self {
            #[allow(dead_code)]
    trigger: GenericTrigger,
        }
    }

    /// Mirrors `onEntityStep(Entity, RoomEntity, Item, Position)`.
    pub fn on_entity_step(
        &self,
        entity: &(dyn Entity + Send),
        room_entity: &RoomEntity,
        _item: &Item,
        old_position: &Position,
    ) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        let Some(player) = entity.as_player() else {
            return;
        };

        let Some(room) = room_entity.get_room() else {
            return;
        };

        if room.get_data().get_model() != "pool_b" {
            return;
        }

        if player.get_details().get_tickets() == 0
            || player.get_details().get_pool_figure().is_empty()
        {
            let mut old_position = old_position.clone();
            old_position.set_rotation(2);
            // Make user face this way, like the original Lido behaviour

            let Some(room_user) = player.get_room_user() else {
                return;
            };

            room_user.stop_walking();
            room_user.warp(&old_position, false, false);

            if player.get_details().get_tickets() == 0 {
                player.send(&NO_TICKETS);
            }
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

        let Some(player) = entity.as_player() else {
            return;
        };

        if let Some(room) = room_entity.get_room() {
            if room.get_data().get_model() == "pool_b" {
                if player.get_details().get_tickets() == 0
                    || player.get_details().get_pool_figure().is_empty()
                {
                    return;
                }
            }
        }

        // When they stop walking, advance public room queues one square at a time.
        StatusTask::process_pool_queue(player, room_entity);
    }
}

impl Default for QueueTileInteractor {
    fn default() -> Self {
        Self::new()
    }
}
