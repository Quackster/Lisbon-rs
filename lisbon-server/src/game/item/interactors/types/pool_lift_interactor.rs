//! Mirrors `net.h4bbo.lisbon.game.item.interactors.types.PoolLiftInteractor`.
use crate::dao::mysql::currency_dao::CurrencyDao;
use crate::game::entity::entity::Entity;
use crate::game::entity::entity_type::EntityType;
use crate::game::item::item::Item;
use crate::game::room::entities::room_entity::RoomEntity;
use crate::game::triggers::generic_trigger::GenericTrigger;
use crate::messages::outgoing::rooms::pool::jumpingplace_ok::JUMPINGPLACE_OK;
use crate::messages::outgoing::user::currencies::ticket_balance::TICKET_BALANCE;

pub struct PoolLiftInteractor {
    #[allow(dead_code)]
    trigger: GenericTrigger,
}

impl PoolLiftInteractor {
    /// Mirrors the `PoolLiftInteractor()` constructor.
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
        _room_entity: &RoomEntity,
        item: &mut Item,
        _is_rotation: bool,
    ) {
        if entity.get_type() != EntityType::Player {
            return;
        }

        let Some(player) = entity.as_player() else {
            return;
        };

        item.show_program(Some("close"));

        let Some(room_user) = player.get_room_user() else {
            return;
        };

        room_user.set_walking_allowed(false);
        // Port note: the Java `resetRoomTimer(120)` AFK override is not
        // ported (the Rust `RoomEntity` state has no AFK / sleep
        // timers); the no-arg `reset_room_timer` is the closest
        // equivalent.
        room_user.reset_room_timer();
        room_user.set_diving(true);

        CurrencyDao::decrease_tickets(player.get_details(), 1);

        player.send(&TICKET_BALANCE::new(player.get_details().get_tickets()));
        player.send(&JUMPINGPLACE_OK);
    }

    /// Mirrors `onEntityLeave(Entity, RoomEntity, Item)`.
    pub fn on_entity_leave(
        &self,
        _entity: &(dyn Entity + Send),
        _room_entity: &RoomEntity,
        item: &mut Item,
    ) {
        item.show_program(Some("open"));
    }
}

impl Default for PoolLiftInteractor {
    fn default() -> Self {
        Self::new()
    }
}
