//! Mirrors `net.h4bbo.lisbon.messages.incoming.trade.TRADE_ADDITEM`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::room::managers::room_trade_manager::RoomTradeManager;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct TRADE_ADDITEM;

impl MessageEvent for TRADE_ADDITEM {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };

        if room_user.get_room().is_none() {
            return Ok(());
        }

        if room_user.get_trade_partner().is_none() {
            return Ok(());
        }

        let item_id = match reader
            .contents()
            .and_then(|contents| contents.parse::<i32>().ok())
        {
            Some(value) => value,
            None => return Ok(()),
        };

        let inventory = player.get_inventory();
        let Some(inventory_item) = inventory.and_then(|inv| inv.get_item(item_id)) else {
            return Ok(());
        };

        room_user.add_trade_item(&inventory_item);

        let Some(trade_partner) = room_user.get_trade_partner() else {
            return Ok(());
        };

        let trade_partner = trade_partner.lock();

        RoomTradeManager::refresh_window(player);
        RoomTradeManager::refresh_window(&trade_partner);

        Ok(())
    }
}
