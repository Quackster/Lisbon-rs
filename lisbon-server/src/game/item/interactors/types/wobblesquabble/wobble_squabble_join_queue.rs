//! Mirrors `net.h4bbo.lisbon.game.item.interactors.types.wobblesquabble.WobbleSquabbleJoinQueue`.
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::games::wobblesquabble::wobble_squabble_manager::WobbleSquabbleManager;
use crate::game::item::interactors::interaction_type::InteractionType;
use crate::game::item::interactors::types::wobblesquabble::wobble_squabble_queue_tile::WobbleSquabbleQueueTile;
use crate::game::item::item::Item;
use crate::game::pathfinder::position::Position;
use crate::game::room::entities::room_entity::RoomEntity;
use crate::game::room::enums::status_type::StatusType;
use crate::game::triggers::generic_trigger::GenericTrigger;
use crate::messages::outgoing::alert::alert::ALERT;

pub struct WobbleSquabbleJoinQueue {
    #[allow(dead_code)]
    trigger: GenericTrigger,
}

impl WobbleSquabbleJoinQueue {
    /// Mirrors the `WobbleSquabbleJoinQueue()` constructor.
    pub fn new() -> Self {
        Self {
            #[allow(dead_code)]
    trigger: GenericTrigger,
        }
    }

    /// Mirrors `onEntityStep(Entity, RoomEntity, Item, Position)`.
    pub fn on_entity_step(
        &self,
        _entity: &(dyn Entity + Send),
        _room_entity: &RoomEntity,
        _item: &Item,
        _old_position: &Position,
    ) {}

    /// Mirrors `onEntityStop(Entity, RoomEntity, Item, boolean)`.
    pub fn on_entity_stop(
        &self,
        entity: &(dyn Entity + Send),
        room_entity: &RoomEntity,
        item: &Item,
        is_rotation: bool,
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

        let Some(player) = entity.as_player() else {
            return;
        };

        if player.get_details().get_tickets() < WobbleSquabbleManager::WS_GAME_TICKET_COST {
            player.send(&ALERT::new(&format!(
                "You need at least {} ticket(s) to play Wobble Squabble!",
                WobbleSquabbleManager::WS_GAME_TICKET_COST
            )));
            return; // Too poor!
        }

        let teleport_position_data: Vec<&str> = item.get_current_program().split(',').collect();

        let Some(x) = teleport_position_data.first().and_then(|v| v.parse::<i32>().ok())
        else {
            return;
        };
        let Some(y) = teleport_position_data.get(1).and_then(|v| v.parse::<i32>().ok())
        else {
            return;
        };
        let Some(rotation) = teleport_position_data.get(2).and_then(|v| v.parse::<i32>().ok())
        else {
            return;
        };

        let mut teleport_position = Position::new_xy(x, y);
        teleport_position.set_rotation(rotation);

        if let Some(room) = room_entity.get_room() {
            if let Some(room_tile) = room.get_mapping().lock().get_tile_by_position(&room, &teleport_position) {
                if !room_tile.get_entities().is_empty() {
                    return;
                }

                room_entity.remove_status(StatusType::Swim);
                room_entity.warp(&teleport_position, true, false);
            } else {
                return;
            }
        } else {
            return;
        }

        if let Some(box_trigger) = InteractionType::WsQueueTile.get_trigger() {
            if let Some(trigger) = box_trigger.downcast_ref::<WobbleSquabbleQueueTile>() {
                trigger.on_entity_stop(entity, room_entity, item, is_rotation);
            }
        }
    }
}

impl Default for WobbleSquabbleJoinQueue {
    fn default() -> Self {
        Self::new()
    }
}
