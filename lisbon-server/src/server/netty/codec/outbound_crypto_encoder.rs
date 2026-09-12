//! Mirrors `net.h4bbo.lisbon.server.netty.codec.OutboundCryptoEncoder`.
//!
//! Encrypts outbound frames after the normal Lisbon encoder serialises them.
//! Java extends `MessageToByteEncoder<ByteBuf>`; here it takes the player's
//! outbound cipher and the serialised frame and returns the hex-encoded
//! ciphertext.

use crate::game::player::player::Player;
use crate::server::netty::game_channel_pipeline::GameChannelPipeline;

/// Encrypts outbound frames for a connection with active outbound crypto.
pub struct OutboundCryptoEncoder;

impl OutboundCryptoEncoder {
    /// Mirrors `encode(ChannelHandlerContext, ByteBuf, ByteBuf)`.
    ///
    /// Returns the frame unchanged when outbound crypto is disabled; when
    /// enabled but the cipher is missing, disables outbound crypto (mirroring
    /// `GameChannelPipeline.disableOutboundCrypto`) and returns the frame
    /// unchanged. Otherwise hex-encrypts the frame and writes the player's
    /// advanced cipher back.
    ///
    /// Port note: the cipher is a `Copy` value on the `Player`; the
    /// advanced state is written back via `set_outbound_cipher`, preserving the
    /// Java in-place mutation semantics.
    pub fn encrypt_frame(player: &mut Player, plaintext: &[u8]) -> Vec<u8> {
        if !player.is_outbound_encrypted() {
            return plaintext.to_vec();
        }

        let Some(mut cipher) = player.get_outbound_cipher() else {
            GameChannelPipeline::disable_outbound_crypto(player);
            return plaintext.to_vec();
        };

        let encrypted = cipher.encrypt_to_hex(plaintext);
        player.set_outbound_cipher(Some(cipher));
        encrypted
    }
}
