//! Mirrors `net.h4bbo.lisbon.game.moderation.ModerationAction`.

use crate::game::player::player::Player;
use crate::game::room::room::Room;
use crate::server::netty::streams::NettyRequest;

/// Mirrors the `ModerationAction` interface.
pub trait ModerationAction {
    /// Mirrors `performAction(Player, Room, String, String, NettyRequest)`.
    fn perform_action(
        &self,
        player: &Player,
        room: &Room,
        alert_message: &str,
        notes: &str,
        reader: &mut NettyRequest,
    );
}
