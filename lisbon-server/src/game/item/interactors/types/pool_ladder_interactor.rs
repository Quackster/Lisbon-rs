//! Mirrors `net.h4bbo.lisbon.game.item.interactors.types.PoolLadderInteractor`.
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::item::item::Item;
use crate::game::pathfinder::position::Position;
use crate::game::room::entities::room_entity::RoomEntity;
use crate::game::room::enums::status_type::StatusType;
use crate::game::triggers::generic_trigger::GenericTrigger;

pub struct PoolLadderInteractor {
    #[allow(dead_code)]
    trigger: GenericTrigger,
}

impl PoolLadderInteractor {
    /// Mirrors the `PoolLadderInteractor()` constructor.
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
        item: &mut Item,
        _old_position: &Position,
    ) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        let Some(teleport_to) = item.get_teleport_to() else {
            return;
        };

        if item.get_definition().get_sprite() == "poolEnter" {
            if room_entity.contains_status(StatusType::Swim) {
                return;
            }

            room_entity.set_status(StatusType::Swim, "");
        }

        if item.get_definition().get_sprite() == "poolExit" {
            if !room_entity.contains_status(StatusType::Swim) {
                return;
            }

            room_entity.remove_status(StatusType::Swim);
        }

        room_entity.stop_walking();

        room_entity.warp(teleport_to, true, false);

        if let Some(swim_to) = item.get_swim_to() {
            room_entity.set_enable_walking_on_stop(true);
            room_entity.walk_to(swim_to.get_x(), swim_to.get_y());
        }

        item.show_program(None);
    }
}

impl Default for PoolLadderInteractor {
    fn default() -> Self {
        Self::new()
    }
}
