//! Mirrors `net.h4bbo.lisbon.messages.incoming.handshake.INIT_CRYPTO`.
use crate::game::player::player::Player;
use crate::messages::outgoing::handshake::crypto_parameters::CRYPTO_PARAMETERS;
use crate::messages::types::MessageEvent;
use crate::server::netty::game_channel_pipeline::GameChannelPipeline;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct INIT_CRYPTO;

impl MessageEvent for INIT_CRYPTO {
    /// Not dispatched; the real handling lives in `handle_mut`, which the
    /// connection dispatcher invokes with exclusive `Player` access.
    fn handle(&self, _player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        Ok(())
    }

    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle_mut(&self, player: &mut Player, _reader: &mut NettyRequest) -> Result<(), String> {
        if player.is_logged_in() {
            return Ok(());
        }

        GameChannelPipeline::reset_crypto(player);
        player.send(&CRYPTO_PARAMETERS);

        Ok(())
    }
}
