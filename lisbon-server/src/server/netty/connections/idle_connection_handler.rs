//! Mirrors `net.h4bbo.lisbon.server.netty.connections.IdleConnectionHandler`.
//!
//! The Java `IdleStateHandler(60, 0, 0)` + `IdleConnectionHandler` pair is
//! reproduced by the read loop's 60-second timeout, which invokes
//! `on_read_idle` on a `READER_IDLE` event.

use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::user::ping::PING;

/// Reacts to read-idle events for a connection.
pub struct IdleConnectionHandler;

impl IdleConnectionHandler {
    /// Mirrors `userEventTriggered(ChannelHandlerContext, Object)` for the
    /// `IdleState.READER_IDLE` case.
    ///
    /// Port note: `Player::send` and `Player::kick_from_server` are stubs;
    /// the ping / kick are issued against them and will take effect once those
    /// are ported.
    pub fn on_read_idle(player: &mut Player) {
        if player.is_ping_ok() {
            player.set_ping_ok(false);
            player.send(&PING);
        } else {
            if player.is_logged_in() {
                tracing::info!(
                    "Player {} has timed out",
                    player.get_details().get_name()
                );
            } else {
                tracing::info!(
                    "Connection {} has timed out",
                    player.get_network().get_connection_id()
                );
            }

            player.kick_from_server();
        }
    }
}
