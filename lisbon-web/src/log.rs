//! Mirrors `org.alexdev.http.log.Log`.

/// Mirrors `org.alexdev.http.log.Log`.
pub struct Log;

impl Log {
    /// Mirrors `getErrorLogger()`.
    pub fn get_error_logger() -> lisbon_server::log::ErrorLogger {
        lisbon_server::log::Log::get_error_logger()
    }
}
