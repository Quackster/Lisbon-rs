//! Mirrors `net.h4bbo.lisbon.server.mus.connection.MusClient`.
//!
//! The Netty `Channel` field is represented by an outbound `mpsc` sender (the
//! connection task owns the socket write half).

use tokio::sync::mpsc::UnboundedSender;

/// A MUS (multi-user server / RCON) client connection.
pub struct MusClient {
    // Port note: the Java field is a Netty `Channel`; here the outbound write
    // channel stands in for it (`getChannel()` is folded into `send`).
    channel: UnboundedSender<Vec<u8>>,
    photo_text: String,
    user_id: i32,
}

impl MusClient {
    /// Mirrors the `MusClient(Channel)` constructor.
    pub fn new(channel: UnboundedSender<Vec<u8>>) -> Self {
        Self {
            channel,
            user_id: 0,
            photo_text: String::new(),
        }
    }

    /// Mirrors `getUserId()`.
    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    /// Mirrors `setUserId(int)`.
    pub fn set_user_id(&mut self, user_id: i32) {
        self.user_id = user_id;
    }

    /// Mirrors `getPhotoText()`.
    pub fn get_photo_text(&self) -> String {
        self.photo_text.clone()
    }

    /// Mirrors `setPhotoText(String)`.
    pub fn set_photo_text(&mut self, photo_text: &str) {
        self.photo_text = photo_text.to_string();
    }

    /// Mirrors `getChannel()` (write side) / `ctx.channel().writeAndFlush(...)`.
    ///
    /// Sends an already-encoded frame to the client.
    pub fn send(&self, frame: Vec<u8>) {
        let _ = self.channel.send(frame);
    }
}
