//! Mirrors `net.h4bbo.lisbon.game.messenger.MessengerMessage`.

#[derive(Clone, Debug, serde::Serialize)]
pub struct MessengerMessage {
    id: i32,
    to_id: i32,
    from_id: i32,
    time_set: i64,
    message: String,
}

impl MessengerMessage {
    /// Mirrors the `MessengerMessage(int, int, int, long, String)`
    /// constructor.
    pub fn new(id: i32, to_id: i32, from_id: i32, time_set: i64, message: &str) -> Self {
        Self {
            id,
            to_id,
            from_id,
            time_set,
            message: message.to_string(),
        }
    }

    /// Mirrors `getId`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `getToId`.
    pub fn get_to_id(&self) -> i32 {
        self.to_id
    }

    /// Mirrors `getFromId`.
    pub fn get_from_id(&self) -> i32 {
        self.from_id
    }

    /// Mirrors `getTimeSet`.
    pub fn get_time_set(&self) -> i64 {
        self.time_set
    }

    /// Mirrors `getMessage`.
    pub fn get_message(&self) -> &str {
        &self.message
    }
}
