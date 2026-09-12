//! Mirrors `net.h4bbo.lisbon.messages.incoming.trade.TRADE_CLOSE`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::room::managers::room_trade_manager::RoomTradeManager;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct TRADE_CLOSE;

impl MessageEvent for TRADE_CLOSE {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        if player.get_room_user().and_then(|e| e.get_room()).is_none() {
            return Ok(());
        }

        if player
            .get_room_user()
            .and_then(|e| e.get_trade_partner())
            .is_none()
        {
            return Ok(());
        }

        RoomTradeManager::close(player);

        Ok(())
    }
}
