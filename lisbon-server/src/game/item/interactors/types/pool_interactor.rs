//! Mirrors `net.h4bbo.lisbon.game.item.interactors.types.PoolInteractor`.
use crate::game::entity::entity::Entity;
use crate::game::pathfinder::position::Position;
use crate::game::room::enums::status_type::StatusType;
use crate::game::room::room::Room;

pub struct PoolInteractor;

impl PoolInteractor {
    /// Mirrors `getTileStatus(Room, Entity, Position, Position, boolean)`.
    pub fn get_tile_status(
        room: &Room,
        entity: &(dyn Entity + Send),
        current: &Position,
        tmp: &Position,
        _is_final_move: bool,
    ) -> bool {
        let mapping = room.get_mapping();
        let mapping = mapping.lock();
        let Some(from_tile) = mapping.get_tile_by_position(room, current) else {
            return false;
        };
        let Some(to_tile) = mapping.get_tile_by_position(room, tmp) else {
            return false;
        };

        let from_item = from_tile.get_highest_item();
        let to_item = to_tile.get_highest_item();

        // Only check these below if the user is in a pool room.
        if let Some(model) = room.get_model() {
            if model.get_name().starts_with("pool_") || model.get_name() == "md_a" {
                if let Some(from_item) = from_item {
                    // Check if they have swimmers before trying to enter pool
                    if from_item.get_definition().get_sprite() == "poolEnter"
                        || from_item.get_definition().get_sprite() == "poolExit"
                    {
                        return entity.get_details().get_pool_figure().len() > 0;
                    }
                }

                if let Some(to_item) = to_item {
                    // Check if they have swimmers before trying to enter pool
                    if to_item.get_definition().get_sprite() == "poolEnter"
                        || to_item.get_definition().get_sprite() == "poolExit"
                    {
                        return entity.get_details().get_pool_figure().len() > 0;
                    }

                    // Don't allow to "enter" the pool if they're already swimming
                    if entity
                        .get_room_user()
                        .map_or(false, |room_user| room_user.contains_status(StatusType::Swim))
                        && to_item.get_definition().get_sprite() == "poolEnter"
                    {
                        return false;
                    }

                    // Don't allow to "leave" the pool if they're not swimming
                    if !entity
                        .get_room_user()
                        .map_or(false, |room_user| room_user.contains_status(StatusType::Swim))
                        && to_item.get_definition().get_sprite() == "poolExit"
                    {
                        return false;
                    }

                    // Don't allow people to enter the booth if it's closed, or don't allow
                    // if they attempt to use the pool lift without swimmers
                    if to_item.get_definition().get_sprite() == "poolBooth"
                        || to_item.get_definition().get_sprite() == "poolLift"
                    {
                        return to_item.get_current_program_value() != "close";
                    }
                }
            }
        }

        true
    }
}

impl Default for PoolInteractor {
    fn default() -> Self {
        Self
    }
}
