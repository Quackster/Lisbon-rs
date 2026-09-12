//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.user.CHAT_MESSAGE`.

use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

/// Mirrors the nested `CHAT_MESSAGE.ChatMessageType` enum.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ChatMessageType {
    Chat,
    Shout,
    Whisper,
}

impl ChatMessageType {
    /// Mirrors `getHeader`.
    pub fn get_header(&self) -> i16 {
        match self {
            ChatMessageType::Chat => 24,
            ChatMessageType::Shout => 26,
            ChatMessageType::Whisper => 25,
        }
    }

    /// Mirrors `ChatMessageType.valueOf` (Java throws on an unknown name;
    /// this returns `None` instead).
    pub fn value_of(name: &str) -> Option<ChatMessageType> {
        match name {
            "CHAT" => Some(ChatMessageType::Chat),
            "SHOUT" => Some(ChatMessageType::Shout),
            "WHISPER" => Some(ChatMessageType::Whisper),
            _ => None,
        }
    }
}

/// Mirrors `CHAT_MESSAGE`.
#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct CHAT_MESSAGE {
    message_type: ChatMessageType,
    instance_id: i32,
    message: String,
}

impl CHAT_MESSAGE {
    /// Mirrors the `CHAT_MESSAGE(ChatMessageType, int, String)` constructor.
    pub fn new(message_type: ChatMessageType, instance_id: i32, message: &str) -> Self {
        Self {
            message_type,
            instance_id,
            message: message.to_string(),
        }
    }
}

impl MessageComposer for CHAT_MESSAGE {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.instance_id);
        response.write_string(&self.message);
    }

    fn get_header(&self) -> i16 {
        self.message_type.get_header()
    }
}
