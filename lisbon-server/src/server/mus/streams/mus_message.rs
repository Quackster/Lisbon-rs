//! Mirrors `net.h4bbo.lisbon.server.mus.streams.MusMessage`.

use crate::server::mus::streams::mus_prop_list::MusPropList;

/// A single MUS protocol message.
pub struct MusMessage {
    size: i32,
    error_code: i32,
    timestamp: i64,
    subject: String,
    sender_id: String,
    receivers: Vec<String>,
    content_type: i16,
    content_int: i32,
    content_string: String,
    content_prop_list: Option<MusPropList>,
}

impl MusMessage {
    /// Mirrors the no-arg `MusMessage` constructor.
    pub fn new() -> Self {
        Self {
            size: 0,
            error_code: 0,
            timestamp: 0,
            subject: String::new(),
            sender_id: String::new(),
            receivers: Vec::new(),
            content_type: 0,
            content_int: 0,
            content_string: String::new(),
            content_prop_list: None,
        }
    }

    /// Mirrors `getSize()`.
    pub fn get_size(&self) -> i32 {
        self.size
    }

    /// Mirrors `setSize(int)`.
    pub fn set_size(&mut self, size: i32) {
        self.size = size;
    }

    /// Mirrors `getErrorCode()`.
    pub fn get_error_code(&self) -> i32 {
        self.error_code
    }

    /// Mirrors `setErrorCode(int)`.
    pub fn set_error_code(&mut self, error_code: i32) {
        self.error_code = error_code;
    }

    /// Mirrors `getTimestamp()`.
    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    /// Mirrors `setTimestamp(long)`.
    pub fn set_timestamp(&mut self, timestamp: i64) {
        self.timestamp = timestamp;
    }

    /// Mirrors `getSubject()`.
    pub fn get_subject(&self) -> &str {
        &self.subject
    }

    /// Mirrors `setSubject(String)`.
    pub fn set_subject(&mut self, subject: &str) {
        self.subject = subject.to_string();
    }

    /// Mirrors `getSenderId()`.
    pub fn get_sender_id(&self) -> &str {
        &self.sender_id
    }

    /// Mirrors `setSenderId(String)`.
    pub fn set_sender_id(&mut self, sender_id: &str) {
        self.sender_id = sender_id.to_string();
    }

    /// Mirrors `getReceivers()`.
    pub fn get_receivers(&self) -> &[String] {
        &self.receivers
    }

    /// Mirrors `setReceivers(String[])`.
    pub fn set_receivers(&mut self, receivers: Vec<String>) {
        self.receivers = receivers;
    }

    /// Mirrors `getContentType()`.
    pub fn get_content_type(&self) -> i16 {
        self.content_type
    }

    /// Mirrors `setContentType(short)`.
    pub fn set_content_type(&mut self, content_type: i16) {
        self.content_type = content_type;
    }

    /// Mirrors `getContentInt()`.
    pub fn get_content_int(&self) -> i32 {
        self.content_int
    }

    /// Mirrors `setContentInt(int)`.
    pub fn set_content_int(&mut self, content_int: i32) {
        self.content_int = content_int;
    }

    /// Mirrors `getContentString()`.
    pub fn get_content_string(&self) -> &str {
        &self.content_string
    }

    /// Mirrors `setContentString(String)`.
    pub fn set_content_string(&mut self, content_string: &str) {
        self.content_string = content_string.to_string();
    }

    /// Mirrors `getContentPropList()`.
    pub fn get_content_prop_list(&self) -> Option<&MusPropList> {
        self.content_prop_list.as_ref()
    }

    /// Mirrors `setContentPropList(MusPropList)`.
    pub fn set_content_prop_list(&mut self, content_prop_list: Option<MusPropList>) {
        self.content_prop_list = content_prop_list;
    }

    /// Mirrors `toString()`.
    ///
    /// The Java null-check on `contentString` is inapplicable (Rust `String` is
    /// never null); an empty string is rendered the same way.
    pub fn to_string(&self) -> String {
        format!("{}:\"{}\"", self.subject, self.content_string)
    }
}

impl Default for MusMessage {
    fn default() -> Self {
        Self::new()
    }
}
