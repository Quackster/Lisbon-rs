//! Mirrors `net.h4bbo.lisbon.messages.incoming.handshake.SECRETKEY`.
use crate::crypto::habbo_cipher::HabboCipher;
use crate::crypto::secret_key_codec::SecretKeyCodec;
use crate::game::player::player::{CryptoMode, Player};
use crate::messages::outgoing::handshake::end_of_crypto_params::END_OF_CRYPTO_PARAMS;
use crate::messages::types::MessageEvent;
use crate::server::netty::game_channel_pipeline::GameChannelPipeline;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct SECRETKEY;

impl MessageEvent for SECRETKEY {
    /// Not dispatched; the real handling lives in `handle_mut`, which the
    /// connection dispatcher invokes with exclusive `Player` access.
    fn handle(&self, _player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        Ok(())
    }

    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle_mut(&self, player: &mut Player, reader: &mut NettyRequest) -> Result<(), String> {
        let encoded_secret_key = reader.read_string();
        let secret_key = SecretKeyCodec::secret_decode(Some(&encoded_secret_key));

        if player.get_crypto_mode() == CryptoMode::None {
            tracing::warn!("Ignoring SECRETKEY before DH setup");
            return Ok(());
        }

        let mut cipher = HabboCipher::default();
        cipher.init_server_to_client_secret_key(secret_key);
        GameChannelPipeline::enable_outbound_crypto(player, cipher);

        player.send(&END_OF_CRYPTO_PARAMS);

        Ok(())
    }
}
