//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.items.CONVERT_FURNI_TO_CREDITS`.
use crate::dao::mysql::item_dao::ItemDao;
use crate::dao::mysql::transaction_dao::TransactionDao;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::player::player::Player;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::messages::outgoing::user::currencies::credit_balance::CREDIT_BALANCE;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct CONVERT_FURNI_TO_CREDITS;

impl MessageEvent for CONVERT_FURNI_TO_CREDITS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, _player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        Ok(())
    }

    fn handle_mut(&self, player: &mut Player, reader: &mut NettyRequest) -> Result<(), String> {
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

        let item_id = reader.read_int();

        if item_id < 0 {
            return Ok(());
        }

        let Some(mut item) = room.get_item_manager().get_by_id(item_id) else {
            return Ok(());
        };

        if !item.has_behaviour(ItemBehaviour::Redeemable) {
            return Ok(());
        }

        // Sprite is of format CF_50_goldbar. This retrieves the 50 part.
        // The Java `ArrayIndexOutOfBounds` / `NumberFormatException` on a
        // malformed sprite.
        let Some(amount) = item
            .get_definition()
            .get_sprite()
            .split('_')
            .nth(1)
            .and_then(|part| part.parse::<i32>().ok())
        else {
            return Ok(());
        };

        // Delete item and update credits amount in one atomic operation
        let current_amount = ItemDao::redeem_credit_item(amount, item_id, player.get_details().get_id());

        // Couldn't redeem item (database error)
        if current_amount == -1 {
            // TODO: find real composer for this. Maybe use error composer?
            player.send(&ALERT::new(
                "Unable to redeem furniture! Contact staff or support team.",
            ));
            return Ok(());
        }

        // Notify room of item removal and set credits of player
        room.get_mapping().lock().remove_item(&room, &mut item);
        player.get_details_mut().set_credits(current_amount);

        TransactionDao::create_transaction(
            player.get_details().get_id(),
            &item.get_id().to_string(),
            "",
            1,
            &format!(
                "Exchanged {} into {} credits",
                item.get_definition().get_name(),
                amount
            ),
            amount,
            0,
            false,
        );

        // Send new credit amount
        player.send(&CREDIT_BALANCE::new(player.get_details().get_credits()));

        Ok(())
    }
}
