//! Mirrors `net.h4bbo.lisbon.game.room.managers.RoomTradeManager`.
use crate::dao::mysql::item_dao::ItemDao;
use crate::dao::mysql::transaction_dao::TransactionDao;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::room::enums::status_type::StatusType;
use crate::messages::outgoing::trade::trade_close::TRADE_CLOSE;
use crate::messages::outgoing::trade::trade_items::TRADE_ITEMS;

pub struct RoomTradeManager;

impl RoomTradeManager {
    /// Mirrors `close(RoomPlayer)`.
    pub fn close(player: &Player) {
        let Some(room_entity) = player.get_room_user() else {
            return;
        };

        let Some(partner) = room_entity.get_trade_partner() else {
            return;
        };

        let partner = partner.lock();

        player.send(&TRADE_CLOSE);
        if let Some(inventory) = player.get_inventory() {
            inventory.view(player, "new");
        }

        partner.send(&TRADE_CLOSE);
        if let Some(inventory) = partner.get_inventory() {
            inventory.view(&partner, "new");
        }

        Self::reset(&partner);
        Self::reset(player);
    }

    /// Mirrors `reset(RoomPlayer)`.
    fn reset(player: &Player) {
        let Some(room_entity) = player.get_room_user() else {
            return;
        };

        room_entity.clear_trade_items();
        room_entity.set_trade_accept(false);
        room_entity.set_trade_partner(None);
        room_entity.remove_status(StatusType::Trade);
        room_entity.set_needs_update(true);
    }

    /// Mirrors `refreshWindow(Player)`.
    pub fn refresh_window(player: &Player) {
        let Some(room_user) = player.get_room_user() else {
            return;
        };

        let Some(trade_partner) = room_user.get_trade_partner() else {
            return;
        };

        let trade_partner = trade_partner.lock();

        player.send(
            &TRADE_ITEMS::new(
                player,
                room_user.get_trade_items(),
                room_user.has_accepted_trade(),
                &trade_partner,
                trade_partner
                    .get_room_user()
                    .map(|e| e.get_trade_items())
                    .unwrap_or_default(),
                trade_partner
                    .get_room_user()
                    .map_or(false, |e| e.has_accepted_trade()),
            ),
        );
    }

    /// Mirrors `addItems(Player, Player)`.
    pub fn add_items(player: &Player, trade_partner: &Player) {
        let mut items_to_update: Vec<_> = Vec::new();

        for item in trade_partner
            .get_room_user()
            .map(|e| e.get_trade_items())
            .unwrap_or_default()
        {
            if let Some(inventory) = trade_partner.get_inventory() {
                inventory.remove_item(&item);
            }
            if let Some(inventory) = player.get_inventory() {
                inventory.add_item(&item);
            }

            let mut item = item;
            item.set_owner_id(player.get_details().get_id());

            let definition_name = item.get_definition().get_name().to_string();
            let partner_name = trade_partner.get_details().get_name().to_string();
            let player_name = player.get_details().get_name().to_string();

            TransactionDao::create_transaction(
                player.get_details().get_id(),
                &item.get_id().to_string(),
                &item.get_definition().get_id().to_string(),
                1,
                &format!("Traded {definition_name} from {partner_name}"),
                0,
                trade_partner.get_details().get_id(),
                false,
            );
            TransactionDao::create_transaction(
                trade_partner.get_details().get_id(),
                &item.get_id().to_string(),
                &item.get_definition().get_id().to_string(),
                1,
                &format!("Traded {definition_name} to {player_name}"),
                0,
                player.get_details().get_id(),
                false,
            );

            items_to_update.push(item.clone());
        }

        ItemDao::update_items(&items_to_update);
    }
}
