//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.items.REMOVEITEM`.
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::games::triggers::battle_ships_trigger::BattleShipsTrigger;
use crate::game::games::triggers::chess_trigger::ChessTrigger;
use crate::game::games::triggers::poker_trigger::PokerTrigger;
use crate::game::games::triggers::tic_tac_toe_trigger::TicTacToeTrigger;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::interactors::types::chair_interactor::ChairInteractor;
use crate::game::item::interactors::types::pool_booth_interactor::PoolBoothInteractor;
use crate::game::item::interactors::types::pool_lift_interactor::PoolLiftInteractor;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct REMOVEITEM;

impl MessageEvent for REMOVEITEM {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room) = player
            .get_room_user()
            .and_then(|room_user| room_user.get_room())
        else {
            return Ok(());
        };

        if !room.is_owner(player.get_details().get_id())
            && !player.has_fuse(&Fuseright::AnyRoomController)
        {
            return Ok(());
        }

        // The Java throws on a non-numeric id.
        let Ok(item_id) = reader.contents().unwrap_or_default().parse::<i32>() else {
            return Ok(());
        };

        let Some(mut item) = room.get_item_manager().get_by_id(item_id) else {
            return Ok(());
        };

        if !room.is_owner(player.get_details().get_id())
            && !(item.has_behaviour(ItemBehaviour::Photo)
                && !player.has_fuse(&Fuseright::RemovePhotos))
            && !(item.has_behaviour(ItemBehaviour::PostIt)
                && !player.has_fuse(&Fuseright::RemoveStickies))
        {
            return Ok(());
        }

        // Set up trigger for leaving a current item
        if let Some(room_user) = player.get_room_user() {
            if let Some(mut current_item) = room_user.get_current_item() {
                // The Java NPEs when the trigger is missing.
                let trigger = current_item
                    .get_definition()
                    .get_interaction_type()
                    .and_then(|interaction_type| interaction_type.get_trigger());

                if let Some(trigger) = trigger {
                    // The Java `onEntityLeave` virtual call; the concrete
                    // dispatch covers the ported overrides.
                    if let Some(chair) = trigger.downcast_ref::<ChairInteractor>() {
                        chair.on_entity_leave(player, room_user, &current_item);
                    } else if let Some(pool_booth) =
                        trigger.downcast_ref::<PoolBoothInteractor>()
                    {
                        pool_booth.on_entity_leave(player, room_user, &mut current_item);
                    } else if let Some(pool_lift) =
                        trigger.downcast_ref::<PoolLiftInteractor>()
                    {
                        pool_lift.on_entity_leave(player, room_user, &mut current_item);
                    } else if let Some(chess) = trigger.downcast_ref::<ChessTrigger>() {
                        chess.on_entity_leave(player, room_user, &current_item);
                    } else if let Some(poker) = trigger.downcast_ref::<PokerTrigger>() {
                        poker.on_entity_leave(player, room_user, &current_item);
                    } else if let Some(battle_ships) = {
                        trigger.downcast_ref::<BattleShipsTrigger>()
                    } {
                        battle_ships.on_entity_leave(player, room_user, &current_item);
                    } else if let Some(tic_tac_toe) = {
                        trigger.downcast_ref::<TicTacToeTrigger>()
                    } {
                        tic_tac_toe.on_entity_leave(player, room_user, &current_item);
                    }
                }
            }
        }

        room.get_mapping().lock().pickup_item(&room, player, &mut item);
        item.delete();

        Ok(())
    }
}
