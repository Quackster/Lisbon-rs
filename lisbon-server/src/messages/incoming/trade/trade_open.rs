//! Mirrors `net.h4bbo.lisbon.messages.incoming.trade.TRADE_OPEN`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::room::enums::status_type::StatusType;
use crate::game::room::managers::room_trade_manager::RoomTradeManager;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct TRADE_OPEN;

impl MessageEvent for TRADE_OPEN {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room) = player.get_room_user().and_then(|e| e.get_room()) else {
            return Ok(());
        };

        if room
            .get_category()
            .map_or(false, |category| category.has_allow_trading())
            == false
        {
            return Ok(());
        }

        if player
            .get_room_user()
            .and_then(|e| e.get_trade_partner())
            .is_some()
        {
            return Ok(());
        }

        let instance_id = match reader
            .contents()
            .and_then(|contents| contents.parse::<i32>().ok())
        {
            Some(value) => value,
            None => return Ok(()),
        };

        let trade_partner = room
            .get_entity_manager()
            .get_players()
            .into_iter()
            .find(|candidate| candidate.lock().get_details().get_id() == instance_id);

        let Some(trade_partner) = trade_partner else {
            return Ok(());
        };

        let partner = trade_partner.clone();
        RoomTradeManager::close(player);
        RoomTradeManager::close(&partner.lock());

        if let Some(room_user) = player.get_room_user() {
            room_user.set_status(StatusType::Trade, "");
            room_user.set_needs_update(true);
            room_user.set_trade_partner(Some(trade_partner));
        }

        let partner = partner.lock();
        if let Some(partner_room_user) = partner.get_room_user() {
            partner_room_user.set_status(StatusType::Trade, "");
            partner_room_user.set_needs_update(true);

            let Some(self_handle) = crate::game::player::player_manager::PlayerManager::get_instance()
                .get_player_by_id(player.get_details().get_id())
            else {
                return Ok(());
            };

            partner_room_user.set_trade_partner(Some(self_handle));
        }

        RoomTradeManager::refresh_window(player);
        RoomTradeManager::refresh_window(&partner);

        Ok(())
    }
}
