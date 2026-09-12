//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.pool.BTCKS`.
use crate::dao::mysql::currency_dao::CurrencyDao;
use crate::dao::mysql::player_dao::PlayerDao;
use crate::game::entity::entity::Entity;
use crate::game::item::interactors::interaction_type::InteractionType;
use crate::game::item::interactors::types::wobblesquabble::wobble_squabble_join_queue::WobbleSquabbleJoinQueue;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::messages::outgoing::alert::no_user_found::NO_USER_FOUND;
use crate::messages::outgoing::catalogue::no_credits::NO_CREDITS;
use crate::messages::outgoing::user::currencies::credit_balance::CREDIT_BALANCE;
use crate::messages::outgoing::user::currencies::ticket_balance::TICKET_BALANCE;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct BTCKS;

impl MessageEvent for BTCKS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let mode = reader.read_int();
        let tickets_for = reader.read_string();

        if tickets_for.is_empty() {
            return Ok(());
        }

        let (cost_credits, tickets_amount) = if mode == 1 {
            (1, 2)
        } else {
            (6, 20)
        };

        if cost_credits > player.get_details().get_credits() {
            player.send(&NO_CREDITS);
            return Ok(());
        }

        let user_id = PlayerDao::get_id(&tickets_for);

        if user_id == -1 {
            player.send(&NO_USER_FOUND::new(&tickets_for));
            return Ok(());
        }

        let Some(details) = PlayerManager::get_instance().get_player_data_by_id(user_id) else {
            return Ok(());
        };

        CurrencyDao::increase_tickets(&details, tickets_amount);

        if let Some(ticket_player) = PlayerManager::get_instance().get_player_by_name(&tickets_for) {
            let ticket_player = ticket_player.lock();

            if user_id != player.get_details().get_id() {
                ticket_player.send(&ALERT::new(&format!(
                    "{} has gifted you tickets!",
                    player.get_details().get_name()
                )));
            }

            ticket_player.send(&TICKET_BALANCE::new(details.get_tickets()));
        }

        player.get_room_user().map(|room_user| room_user.reset_room_timer());

        CurrencyDao::decrease_credits(player.get_details(), cost_credits);
        player.send(&CREDIT_BALANCE::new(player.get_details().get_credits()));

        // Join queue after buying ticket.
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        if room.get_model().map(|model| model.get_name() == "md_a").unwrap_or(false) {
            let Some(item) = room_user.get_current_item() else {
                return Ok(());
            };

            if item.get_definition().get_interaction_type() == Some(InteractionType::WsJoinQueue) {
                if let Some(trigger) = item
                    .get_definition()
                    .get_interaction_type()
                    .unwrap()
                    .get_trigger()
                {
                    if let Some(ws_join) = trigger.downcast_ref::<WobbleSquabbleJoinQueue>() {
                        ws_join.on_entity_stop(player, room_user, &item, false);
                    }
                }
            }
        }

        Ok(())
    }
}
