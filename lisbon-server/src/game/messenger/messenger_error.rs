//! Mirrors `net.h4bbo.lisbon.game.messenger.MessengerError`.

use crate::game::messenger::messenger_error_reason::MessengerErrorReason;
use crate::game::messenger::messenger_error_type::MessengerErrorType;

#[derive(Clone, Debug)]
pub struct MessengerError {
    causer: Option<String>,
    error: MessengerErrorType,
    reason: Option<MessengerErrorReason>,
}

impl MessengerError {
    /// Mirrors the `MessengerError(MessengerErrorType)` constructor.
    pub fn new(error: MessengerErrorType) -> Self {
        Self {
            causer: None,
            error,
            reason: None,
        }
    }

    /// Mirrors the `MessengerError(MessengerErrorType, MessengerErrorReason)`
    /// constructor.
    pub fn new_with_reason(error: MessengerErrorType, reason: MessengerErrorReason) -> Self {
        Self {
            causer: None,
            error,
            reason: Some(reason),
        }
    }

    /// Mirrors `getCauser`.
    pub fn get_causer(&self) -> Option<&str> {
        self.causer.as_deref()
    }

    /// Mirrors `setCauser`.
    pub fn set_causer(&mut self, causer: &str) {
        self.causer = Some(causer.to_string());
    }

    /// Mirrors `getErrorType`.
    pub fn get_error_type(&self) -> MessengerErrorType {
        self.error
    }

    /// Mirrors `getErrorReason`.
    pub fn get_error_reason(&self) -> Option<MessengerErrorReason> {
        self.reason
    }
}
