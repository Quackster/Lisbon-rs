//! Mirrors `net.h4bbo.lisbon.game.bot.BotSpeech`.

use crate::messages::outgoing::rooms::user::chat_message::ChatMessageType;

#[derive(Clone)]
pub struct BotSpeech {
    speech: String,
    chat_message_type: ChatMessageType,
}

impl BotSpeech {
    /// Mirrors the `BotSpeech(String)` constructor (Java throws on an
    /// unknown `ChatMessageType` name; this defaults to `Chat`).
    pub fn new(speech: &str) -> Self {
        if speech.contains('#') {
            let parts: Vec<&str> = speech.split('#').collect();
            let chat_message_type = ChatMessageType::value_of(parts.get(1).copied().unwrap_or("CHAT"))
                .unwrap_or(ChatMessageType::Chat);

            Self {
                speech: parts[0].to_string(),
                chat_message_type,
            }
        } else {
            Self {
                speech: speech.to_string(),
                chat_message_type: ChatMessageType::Chat,
            }
        }
    }

    /// Mirrors `getSpeech`.
    pub fn get_speech(&self) -> &str {
        &self.speech
    }

    /// Mirrors `getChatMessageType`.
    pub fn get_chat_message_type(&self) -> ChatMessageType {
        self.chat_message_type
    }
}
