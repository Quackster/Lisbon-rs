//! Lisbon - Habbo Hotel V26 emulation server (Rust port).
//!
//! Module tree mirrors the Java package `net.h4bbo.lisbon.*`.

pub mod log;
pub mod lisbon;
pub mod util;
pub mod crypto;
pub mod dao;
pub mod messages;
pub mod server;
pub mod game;

/// Mirrors `Lisbon.SERVER_VERSION`.
pub const SERVER_VERSION: &str = "v1.6";
