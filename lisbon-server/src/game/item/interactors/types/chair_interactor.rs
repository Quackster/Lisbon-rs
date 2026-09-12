//! Mirrors `net.h4bbo.lisbon.game.item.interactors.types.ChairInteractor`.
use crate::game::entity::entity::Entity;
use crate::game::item::item::Item;
use crate::game::room::entities::room_entity::RoomEntity;
use crate::game::room::enums::status_type::StatusType;
use crate::game::triggers::generic_trigger::GenericTrigger;
use crate::util::string_util::StringUtil;

pub struct ChairInteractor {
    #[allow(dead_code)]
    trigger: GenericTrigger,
}

impl ChairInteractor {
    /// Mirrors the `ChairInteractor()` constructor.
    pub fn new() -> Self {
        Self {
            #[allow(dead_code)]
    trigger: GenericTrigger,
        }
    }

    /// Mirrors `onEntityStop(Entity, RoomEntity, Item, boolean)`.
    pub fn on_entity_stop(
        &self,
        _entity: &(dyn Entity + Send),
        room_entity: &RoomEntity,
        item: &Item,
        _is_rotation: bool,
    ) {
        let is_rolling = room_entity.is_rolling();

        let head_rotation = room_entity.get_position().get_head_rotation();

        let mut position = room_entity.get_position();
        position.set_rotation(item.get_position().get_rotation());
        room_entity.set_position(position);
        room_entity.remove_status(StatusType::Dance);
        room_entity.set_status(
            StatusType::Sit,
            &format!("{}", StringUtil::format(item.get_definition().get_top_height())),
        );
        room_entity.set_needs_update(true);

        if is_rolling && room_entity.get_look_timer() > -1 {
            let mut position = room_entity.get_position();
            position.set_head_rotation(head_rotation);
            room_entity.set_position(position);
        }
    }

    /// Mirrors `onEntityLeave(Entity, RoomEntity, Item)`.
    pub fn on_entity_leave(
        &self,
        _entity: &(dyn Entity + Send),
        _room_entity: &RoomEntity,
        _item: &Item,
    ) {}
}

impl Default for ChairInteractor {
    fn default() -> Self {
        Self::new()
    }
}
