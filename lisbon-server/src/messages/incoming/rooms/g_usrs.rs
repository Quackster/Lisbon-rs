//! Mirrors `net.h4bbo.lisbon.messages.incoming.rooms.G_USRS`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::rooms::user::user_objects::USER_OBJECTS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct G_USRS;

impl MessageEvent for G_USRS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        if room_user.get_room().is_none() {
            return Ok(());
        }

        if let Some(game_player_arc) = room_user.get_game_player() {
            if game_player_arc.lock().is_in_game() {
                return Ok(()); // Not needed for game arenas
            }
        }

        let Some(room) = room_user.get_room() else {
            return Ok(());
        };

        player.send(&USER_OBJECTS::new(&room.get_entities()));

        Ok(())
    }
}
