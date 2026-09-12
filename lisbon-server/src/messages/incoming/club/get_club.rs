//! Mirrors `net.h4bbo.lisbon.messages.incoming.club.GET_CLUB`.
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GET_CLUB;

impl MessageEvent for GET_CLUB {
    /// Not dispatched; the real handling lives in `handle_mut`, which the
    /// connection dispatcher invokes with exclusive `Player` access.
    fn handle(&self, _player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        Ok(())
    }

    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle_mut(&self, player: &mut Player, _reader: &mut NettyRequest) -> Result<(), String> {
        player.refresh_club();

        Ok(())
    }
}
