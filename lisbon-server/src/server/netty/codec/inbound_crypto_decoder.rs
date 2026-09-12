//! Mirrors `net.h4bbo.lisbon.server.netty.codec.InboundCryptoDecoder`.
//!
//! Decrypts Director-style init handshake frames before the normal packet
//! decoder sees them. Java extends `ByteToMessageDecoder`; here it operates on
//! the connection's `Vec<u8>` accumulator plus the player's inbound cipher,
//! and returns the decrypted plaintext (or falls back to plaintext decoding by
//! clearing the player's inbound-crypto flags).

use crate::game::player::player::Player;
use crate::server::netty::game_channel_pipeline::GameChannelPipeline;
use crate::util::encoding::base64_encoding::Base64Encoding;

/// The number of hex bytes in an encrypted length prefix.
const ENCRYPTED_LENGTH_HEX_BYTES: usize = 6;
/// The number of bytes in the (decrypted) plaintext length prefix.
const PLAINTEXT_LENGTH_BYTES: usize = 3;

/// Decrypts inbound frames for a connection with active inbound crypto.
pub struct InboundCryptoDecoder;

impl InboundCryptoDecoder {
    /// Mirrors `decode(ChannelHandlerContext, ByteBuf, List<Object>)`.
    ///
    /// Returns `Some(plaintext)` when one encrypted frame was fully decrypted
    /// (the encrypted bytes are removed from `buf`, and the player's advanced
    /// cipher is written back), or `None` when more bytes are required. When a
    /// frame cannot be decrypted, the player's inbound crypto is disabled
    /// (mirroring `fallbackToPlaintext`) so the next pass uses plaintext.
    ///
    /// Port note: the cipher is a `Copy` value on the `Player`; the
    /// advanced state is written back via `set_inbound_cipher`, which preserves
    /// the Java in-place mutation semantics.
    pub fn decode(player: &mut Player, buf: &mut Vec<u8>) -> Option<Vec<u8>> {
        if !player.is_inbound_encrypted() {
            return None;
        }
        let Some(mut cipher) = player.get_inbound_cipher() else {
            // fallbackToPlaintext(player, in, out, null)
            GameChannelPipeline::disable_inbound_crypto(player);
            return None;
        };

        if buf.len() < ENCRYPTED_LENGTH_HEX_BYTES {
            if buf.len() >= PLAINTEXT_LENGTH_BYTES && !Self::looks_like_hex(buf, buf.len()) {
                GameChannelPipeline::disable_inbound_crypto(player);
                tracing::warn!(
                    "Session {} stayed in plaintext after crypto setup, falling back to plaintext decoding",
                    Self::remote_address(player)
                );
            }
            return None;
        }

        if !Self::looks_like_hex(buf, ENCRYPTED_LENGTH_HEX_BYTES) {
            GameChannelPipeline::disable_inbound_crypto(player);
            tracing::warn!(
                "Session {} sent non-hex data after crypto setup, falling back to plaintext decoding",
                Self::remote_address(player)
            );
            return None;
        }

        // in.markReaderIndex(); byte[] encryptedLength = read 6 bytes;
        let encrypted_length = buf[..ENCRYPTED_LENGTH_HEX_BYTES].to_vec();

        // byte[] decryptedLength = cipher.copy().decryptHexStream(encryptedLength);
        let decrypted_length = cipher.copy().decrypt_hex_stream(&encrypted_length);
        if !Self::is_habbo_base64_prefix(&decrypted_length) {
            tracing::warn!(
                "Invalid encrypted length prefix for {}",
                Self::remote_address(player)
            );
            // The 6 bytes are consumed (no reset), mirroring the Java return.
            buf.drain(..ENCRYPTED_LENGTH_HEX_BYTES);
            return None;
        }

        // int bodyLength = Base64Encoding.decode(decryptedLength[0..3]);
        let body_length = Base64Encoding::decode(&decrypted_length[..PLAINTEXT_LENGTH_BYTES]);
        if body_length < 2 {
            tracing::warn!(
                "Invalid encrypted message length {} for {}",
                body_length,
                Self::remote_address(player)
            );
            buf.drain(..ENCRYPTED_LENGTH_HEX_BYTES);
            return None;
        }

        // if (in.readableBytes() < encryptedBodyHexBytes) resetReaderIndex(); return;
        let encrypted_body_hex_bytes = (body_length as usize) * 2;
        if buf.len() < ENCRYPTED_LENGTH_HEX_BYTES + encrypted_body_hex_bytes {
            return None;
        }

        // byte[] encryptedFrame = encryptedLength ++ readBody;
        let mut encrypted_frame = encrypted_length;
        encrypted_frame
            .extend_from_slice(&buf[ENCRYPTED_LENGTH_HEX_BYTES..ENCRYPTED_LENGTH_HEX_BYTES + encrypted_body_hex_bytes]);
        buf.drain(..ENCRYPTED_LENGTH_HEX_BYTES + encrypted_body_hex_bytes);

        // out.add(cipher.decryptFrame(encryptedFrame));
        let plaintext = cipher.decrypt_frame(&encrypted_frame);
        player.set_inbound_cipher(Some(cipher));
        Some(plaintext)
    }

    /// Mirrors `remoteAddress(Player)`.
    fn remote_address(player: &Player) -> String {
        player.get_network().get_remote_ip().to_string()
    }

    /// Mirrors `looksLikeHex(ByteBuf, int)`.
    fn looks_like_hex(buf: &[u8], bytes_to_check: usize) -> bool {
        buf.iter().take(bytes_to_check).all(Self::is_hex)
    }

    /// Mirrors `isHex(byte)`.
    fn is_hex(value: &u8) -> bool {
        (b'0'..=b'9').contains(value)
            || (b'A'..=b'F').contains(value)
            || (b'a'..=b'f').contains(value)
    }

    /// Mirrors `isHabboBase64Prefix(byte[])`.
    fn is_habbo_base64_prefix(value: &[u8]) -> bool {
        value.len() >= PLAINTEXT_LENGTH_BYTES
            && Self::is_habbo_base64_byte(value[0])
            && Self::is_habbo_base64_byte(value[1])
            && Self::is_habbo_base64_byte(value[2])
    }

    /// Mirrors `isHabboBase64Byte(byte)`.
    fn is_habbo_base64_byte(value: u8) -> bool {
        (64..=127).contains(&value)
    }
}
