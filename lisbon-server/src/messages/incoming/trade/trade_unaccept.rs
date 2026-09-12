//! Mirrors `net.h4bbo.lisbon.messages.incoming.trade.TRADE_UNACCEPT`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::room::managers::room_trade_manager::RoomTradeManager;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct TRADE_UNACCEPT;

impl MessageEvent for TRADE_UNACCEPT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };

        if room_user.get_room().is_none() {
            return Ok(());
        }

        let Some(trade_partner) = room_user.get_trade_partner() else {
            return Ok(());
        };

        let trade_partner = trade_partner.lock();

        room_user.set_trade_accept(false);

        RoomTradeManager::refresh_window(player);
        RoomTradeManager::refresh_window(&trade_partner);

        Ok(())
    }
}
