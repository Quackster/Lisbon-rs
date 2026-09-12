//! Mirrors `net.h4bbo.lisbon.messages.incoming.games.REQUESTFULLGAMESTATUS`.
use crate::game::entity::entity::Entity;
use crate::game::games::game_manager::GameManager;
use crate::game::player::player::Player;
use crate::messages::outgoing::games::fullgamestatus::FULLGAMESTATUS;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct REQUESTFULLGAMESTATUS;

impl MessageEvent for REQUESTFULLGAMESTATUS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        let Some(room_user) = player.get_room_user() else {
            return Ok(());
        };
        if room_user.get_room().is_none() {
            return Ok(());
        }

        let Some(game_player_arc) = room_user.get_game_player() else {
            return Ok(());
        };
        let game_id = game_player_arc.lock().get_game_id();

        let Some(game) = GameManager::get_instance().get_game_by_id(game_id) else {
            return Ok(());
        };

        // The Java `instanceof SnowStormGame` guard.
        if game.as_snow_storm().is_none() {
            return Ok(());
        }

        player.send(&FULLGAMESTATUS::new(game.clone()));

        Ok(())
    }
}
