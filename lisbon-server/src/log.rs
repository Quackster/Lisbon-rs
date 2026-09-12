//! Mirrors `net.h4bbo.lisbon.log.Log`.
//!
//! The Java class wraps a named slf4j logger ("ErrorLogger"). Here we map it to
//! a `tracing` target of the same name so the rest of the port can call
//! `Log::get_error_logger().error(...)` exactly as before.

/// A handle that mirrors the slf4j `Logger` returned by `getErrorLogger()`.
#[derive(Clone, Copy, Default)]
pub struct ErrorLogger;

impl ErrorLogger {
    /// Log an error message (mirrors `Logger#error(String)`).
    pub fn error(&self, msg: impl std::fmt::Display) {
        tracing::error!(target: "ErrorLogger", "{}", msg);
    }

    /// Log an error message with a trailing cause (mirrors
    /// `Logger#error(String, Throwable)`).
    pub fn error_with(&self, msg: impl std::fmt::Display, err: impl std::fmt::Display) {
        tracing::error!(target: "ErrorLogger", "{}: {}", msg, err);
    }
}

/// Mirrors `net.h4bbo.lisbon.log.Log`.
pub struct Log;

impl Log {
    /// Returns the error logger (mirrors `getErrorLogger()`).
    pub fn get_error_logger() -> ErrorLogger {
        ErrorLogger
    }
}
