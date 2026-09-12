//! Mirrors `net.h4bbo.lisbon.messages.incoming.handshake.GENERATEKEY`.
use crate::crypto::diffie_hellman::DiffieHellman;
use crate::crypto::habbo_cipher::HabboCipher;
use crate::game::player::player::{CryptoMode, Player};
use crate::messages::outgoing::handshake::available_sets::AVAILABLE_SETS;
use crate::messages::outgoing::handshake::secret_key::SECRET_KEY;
use crate::messages::types::MessageEvent;
use crate::server::netty::game_channel_pipeline::GameChannelPipeline;
use crate::server::netty::streams::NettyRequest;
use crate::util::config::game_configuration::GameConfiguration;

const INIT_COMPAT_SHARED_SECRET: [u8; 1] = [0x01];

#[allow(non_camel_case_types)]
pub struct GENERATEKEY;

impl MessageEvent for GENERATEKEY {
    /// Not dispatched; the real handling lives in `handle_mut`, which the
    /// connection dispatcher invokes with exclusive `Player` access.
    fn handle(&self, _player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        Ok(())
    }

    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle_mut(&self, player: &mut Player, reader: &mut NettyRequest) -> Result<(), String> {
        if player.is_logged_in() {
            return Ok(());
        }

        reader.contents();
        player.set_crypto_mode(CryptoMode::Init);

        let mut cipher = HabboCipher::default();
        cipher.init_init_socket(&INIT_COMPAT_SHARED_SECRET);
        GameChannelPipeline::enable_inbound_crypto(player, cipher);

        player.send(&SECRET_KEY::new(&DiffieHellman::init_compatibility_public_key_hex()));

        player.send(&AVAILABLE_SETS::new(format!(
            "[{}]",
            GameConfiguration::get_instance().get_string("users.figure.parts.default")
        )));

        Ok(())
    }
}
