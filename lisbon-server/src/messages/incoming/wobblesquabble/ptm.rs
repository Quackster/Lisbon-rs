//! Mirrors `net.h4bbo.lisbon.messages.incoming.wobblesquabble.PTM`.
use crate::game::games::wobblesquabble::wobble_squabble_manager::WobbleSquabbleManager;
use crate::game::games::wobblesquabble::wobble_squabble_move::WobbleSquabbleMove;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct PTM;

impl MessageEvent for PTM {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        if !WobbleSquabbleManager::get_instance().is_playing(player) {
            return Ok(());
        }

        let contents = reader.contents().unwrap_or_default();

        let Some(move_) = WobbleSquabbleMove::get_move(&contents) else {
            return Ok(());
        };

        let Some(ws_player) = WobbleSquabbleManager::get_instance().get_player(player) else {
            return Ok(());
        };
        let mut ws_player = ws_player.lock();

        ws_player.set_move(move_);
        ws_player.set_requires_update(true);

        Ok(())
    }
}
