//! Mirrors `net.h4bbo.lisbon.server.netty.GameChannelPipeline`.
//!
//! The Java class adds/removes crypto handlers in a Netty pipeline around the
//! (de)coders. In tokio there is no dynamic pipeline; the crypto handlers
//! (`InboundCryptoDecoder` / `OutboundCryptoEncoder`) run conditionally based on
//! the player's cipher + encrypted flags, so "adding/removing a handler" maps
//! onto setting/clearing those flags.

use crate::crypto::habbo_cipher::HabboCipher;
use crate::game::player::player::Player;

/// Pipeline-name constants (mirrored for fidelity).
pub struct GameChannelPipeline;

impl GameChannelPipeline {
    /// Mirrors the `INBOUND_CRYPTO` constant.
    pub const INBOUND_CRYPTO: &'static str = "inboundCrypto";
    /// Mirrors the `DECODER` constant.
    pub const DECODER: &'static str = "gameDecoder";
    /// Mirrors the `OUTBOUND_CRYPTO` constant.
    pub const OUTBOUND_CRYPTO: &'static str = "outboundCrypto";
    /// Mirrors the `ENCODER` constant.
    pub const ENCODER: &'static str = "gameEncoder";

    /// Mirrors `resetCrypto(Player)`.
    ///
    /// Port note: no dynamic pipeline to strip; the crypto flags on the
    /// player are reset, which disables both crypto handlers.
    pub fn reset_crypto(player: &mut Player) {
        player.reset_crypto();
    }

    /// Mirrors `enableInboundCrypto(Player, HabboCipher)`.
    pub fn enable_inbound_crypto(player: &mut Player, cipher: HabboCipher) {
        player.set_inbound_cipher(Some(cipher));
        player.set_inbound_encrypted(true);
        // Port note: `addBefore(..., InboundCryptoDecoder)` has no pipeline
        // equivalent; the decoder runs when these flags are set.
    }

    /// Mirrors `disableInboundCrypto(Player)`.
    pub fn disable_inbound_crypto(player: &mut Player) {
        player.set_inbound_cipher(None);
        player.set_inbound_encrypted(false);
    }

    /// Mirrors `enableOutboundCrypto(Player, HabboCipher)`.
    pub fn enable_outbound_crypto(player: &mut Player, cipher: HabboCipher) {
        player.set_outbound_cipher(Some(cipher));
        player.set_outbound_encrypted(true);
        // Port note: `addBefore(..., OutboundCryptoEncoder)` has no pipeline
        // equivalent; the encoder runs when these flags are set.
    }

    /// Mirrors `disableOutboundCrypto(Player)`.
    pub fn disable_outbound_crypto(player: &mut Player) {
        player.set_outbound_cipher(None);
        player.set_outbound_encrypted(false);
    }
}
