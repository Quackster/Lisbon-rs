//! Mirrors `net.h4bbo.lisbon.messages.incoming.games.UNOBSERVEINSTANCE`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct UNOBSERVEINSTANCE;

impl MessageEvent for UNOBSERVEINSTANCE {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };

        if room_user.get_observing_game_id() != -1 {
            // The Java `stopObservingGame` uses the `RoomPlayer`
            // back-reference `Player`; it is resolved through the
            // entity manager.
            let player_id = player.get_details().get_id();
            let player_arc = room_user
                .get_room()
                .and_then(|room| {
                    room.get_entity_manager()
                        .get_players()
                        .into_iter()
                        .find(|p| p.lock().get_details().get_id() == player_id)
                });

            if let Some(player_arc) = player_arc {
                room_user.stop_observing_game(&player_arc);
            }
        }

        Ok(())
    }
}
