//! Mirrors `net.h4bbo.lisbon.game.games.snowstorm.util.SnowStormMessage`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::games::player::game_player::GamePlayer;
use crate::game::games::snowstorm::snow_storm_game::SnowStormGame;
use crate::server::netty::streams::NettyRequest;

/// Mirrors the `SnowStormMessage` interface (the `Send + Sync`
// supertraits let the handler singleton live in a `static`).
pub trait SnowStormMessage: Send + Sync {
    /// Mirrors `handle(NettyRequest, SnowStormGame, GamePlayer)`.
    fn handle(
        &self,
        request: &mut NettyRequest,
        snow_storm_game: &Arc<SnowStormGame>,
        game_player: &Arc<Mutex<GamePlayer>>,
    );
}
