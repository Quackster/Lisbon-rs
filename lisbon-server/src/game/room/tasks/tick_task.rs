//! Mirrors `net.h4bbo.lisbon.game.room.tasks.TickTask`.
use crate::util::date_util::DateUtil;

/// Mirrors the abstract `TickTask` class (the Java `timeUntilNextTick`
/// field lives on the implementor side since Rust traits carry no
/// fields).
pub trait TickTask {
    /// Mirrors `tick()`.
    fn tick(&self);

    /// Mirrors `getTimeUntilNextTick()`.
    fn get_time_until_next_tick(&self) -> i64;

    /// Mirrors `setTimeUntilNextTick(int)`.
    ///
    /// Port note: the Java `timeUntilNextTick` field is stored on the
    /// implementor side (Rust traits carry no fields); implementors that
    /// need it override this.
    fn set_time_until_next_tick(&mut self, _secs: i32) {}

    /// Mirrors `isTickRunnable()`.
    fn is_tick_runnable(&self) -> bool {
        DateUtil::get_current_time_seconds() as i64 > self.get_time_until_next_tick()
    }
}
