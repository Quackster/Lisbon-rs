//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.G_STAT`.
use crate::game::entity::entity::Entity;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::player::player::Player;
use crate::game::room::room_manager::RoomManager;
use crate::messages::outgoing::games::gamestart::GAMESTART;
use crate::messages::outgoing::rooms::items::dice_value::DICE_VALUE;
use crate::messages::outgoing::rooms::items::show_program::SHOWPROGRAM;
use crate::messages::outgoing::rooms::items::stuff_data_update::STUFFDATAUPDATE;
use crate::messages::outgoing::rooms::user::user_objects::USER_OBJECTS;
use crate::messages::outgoing::rooms::user::user_statuses::USER_STATUSES;
use crate::messages::outgoing::rooms::user::youarespectator::YOUARESPECTATOR;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct G_STAT;

impl MessageEvent for G_STAT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        if let Some(game_player_arc) = room_user.get_game_player() {
            let game_player = game_player_arc.lock();

            if game_player.is_spectator() {
                player.send(&YOUARESPECTATOR);

                // The Java NPEs when the game is gone.
                let game = game_player.get_game();
                if let Some(game) = &game {
                    if game.is_game_started() {
                        player.send(&GAMESTART::new(game.get_total_seconds_left()));
                    }
                }

                return Ok(());
            }

            if game_player.is_in_game() {
                return Ok(()); // Not needed for game arenas
            }
        }

        // Only refresh rights when in a private room.
        if !room.is_public_room() {
            room.refresh_rights(player);
        }

        let player_id = player.get_details().get_id();
        room.send_to(&USER_OBJECTS::from_entity(player), &[player_id]);

        let entities = room.get_entities();
        player.send(&USER_OBJECTS::new(&entities));

        room.get_entity_manager().try_room_entry(&room, player);

        let room_entry_badges = RoomManager::get_instance().get_room_entry_badges();
        if let Some(badges) = room_entry_badges.get(&room.get_id()) {
            for badge in badges {
                player.get_badge_manager().try_add_badge(badge, None, 0);
            }
        }

        let entities = room.get_entities();
        let entity_refs: Vec<&(dyn Entity + Send)> =
            entities.iter().map(|entity| entity.as_ref()).collect();
        player.send(&USER_STATUSES::new(entity_refs));

        room_user.set_needs_update(true);

        for item in room.get_items() {
            if item.get_current_program_value().len() > 0 {
                player.send(&SHOWPROGRAM::new(vec![
                    item.get_current_program().to_string(),
                    item.get_current_program_value().to_string(),
                ]));
            }

            if item.get_requires_update() {
                if item.has_behaviour(ItemBehaviour::WheelOfFortune) {
                    player.send(&STUFFDATAUPDATE::new(Box::new(item.clone())));
                }

                if item.has_behaviour(ItemBehaviour::Dice) {
                    player.send(&DICE_VALUE::new(item.get_id(), true, 0));
                }
            }
        }

        Ok(())
    }
}
