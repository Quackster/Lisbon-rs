//! Mirrors `net.h4bbo.lisbon.util.schedule.FutureRunnable`.
//!
//! Java wraps a `java.util.concurrent.Future<?>` to permit cancellation of a
//! scheduled runnable. Rust has no direct `Future<?>` equivalent that can be
//! stored and cancelled the same way, so we model the cancellation semantics
//! with an `Arc<AtomicBool>` token.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct FutureRunnable {
    future: Option<Arc<AtomicBool>>,
}

impl FutureRunnable {
    pub fn new() -> Self {
        Self { future: None }
    }

    /// Mirrors `getFuture()`.
    pub fn get_future(&self) -> Option<Arc<AtomicBool>> {
        self.future.clone()
    }

    /// Mirrors `setFuture(Future)`.
    pub fn set_future(&mut self, future: Arc<AtomicBool>) {
        self.future = Some(future);
    }

    /// Mirrors `cancelFuture()`.
    pub fn cancel_future(&mut self) {
        if let Some(f) = &self.future {
            f.store(true, Ordering::SeqCst);
            self.future = None;
        }
    }

    /// Returns `true` if the wrapped scheduled task was asked to cancel.
    pub fn is_cancelled(&self) -> bool {
        self.future
            .as_ref()
            .map(|f| f.load(Ordering::SeqCst))
            .unwrap_or(false)
    }
}

impl Default for FutureRunnable {
    fn default() -> Self {
        Self::new()
    }
}
