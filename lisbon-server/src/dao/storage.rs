//! Mirrors `net.h4bbo.lisbon.dao.Storage`.
//!
//! Java wraps a HikariCP / MariaDB connection pool behind a small API
//! (`connect`, `execute`, `getString`, `getConnection`, `closeSilently`).
//!
//! Rust architecture decision: the game logic (and the DAOs) stay **synchronous**
//! to mirror the blocking JDBC calls. The `sqlx` pool is async, so `Storage`
//! owns a dedicated `tokio::runtime::Runtime` and drives the pool with
//! `block_on`. IMPORTANT: the game logic must run on blocking threads (e.g. via
//! `tokio::task::spawn_blocking` from the async network layer), never inside an
//! async context, or `block_on` will panic.
//!
//! Note: the runtime must be created **before** the pool — sqlx spawns
//! background maintenance tasks at pool construction, which also require an
//! active runtime context.

use std::sync::OnceLock;

use sqlx::mysql::{MySqlPool, MySqlPoolOptions, MySqlRow};
use sqlx::Row;
use tokio::runtime::Runtime;

use crate::log::Log;
use crate::util::config::server_configuration::ServerConfiguration;

pub struct Storage {
    pool: MySqlPool,
    runtime: Runtime,
    connected: bool,
}

static STORAGE: OnceLock<Storage> = OnceLock::new();

impl Storage {
    /// Mirrors `Storage.connect()`.
    pub fn connect() -> bool {
        tracing::info!("Connecting to MySQL server");

        let host = ServerConfiguration::get_string("mysql.hostname");
        let port = ServerConfiguration::get_integer("mysql.port");
        let username = ServerConfiguration::get_string("mysql.username");
        let password = ServerConfiguration::get_string("mysql.password");
        let database = ServerConfiguration::get_string("mysql.database");

        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            username, password, host, port, database
        );

        // The runtime must exist before pool creation: sqlx's pool spawns
        // background maintenance tasks at construction time, which require an
        // active runtime context.
        let runtime = match tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
        {
            Ok(r) => r,
            Err(e) => {
                Log::get_error_logger()
                    .error_with("Could not build DB runtime", e.to_string());
                return false;
            }
        };

        let pool = match runtime
            .block_on(MySqlPoolOptions::new().max_connections(4).connect(&url))
        {
            Ok(p) => p,
            Err(e) => {
                Log::get_error_logger().error_with("Could not connect", e.to_string());
                return false;
            }
        };

        tracing::info!("Connection to MySQL was a success");
        if STORAGE
            .set(Storage {
                pool,
                runtime,
                connected: true,
            })
            .is_err()
        {
            return false;
        }
        true
    }

    /// Mirrors `Storage.logError(Exception)`.
    pub fn log_error(err: impl std::fmt::Display) {
        Log::get_error_logger()
            .error_with("Error when executing MySQL query: ", err.to_string());
    }

    /// Returns the singleton `Storage` (mirrors `getStorage()`).
    pub fn get_storage() -> &'static Storage {
        STORAGE.get().expect("Storage::connect() must be called before use")
    }

    /// Exposes the underlying pool (mirrors `getConnection()`).
    pub fn pool(&self) -> &MySqlPool {
        &self.pool
    }

    /// Exposes the dedicated runtime so the DAOs can drive
    /// multi-statement transactions through `block_on`.
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    /// Mirrors `isConnected()`.
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Convenience: run a query and return all rows (mirrors the
    /// prepare/execute/query loop used across the DAOs).
    pub fn query_all(&self, sql: &str) -> Vec<MySqlRow> {
        match self
            .runtime
            .block_on(sqlx::query(sql).fetch_all(&self.pool))
        {
            Ok(rows) => rows,
            Err(e) => {
                Self::log_error(e.to_string());
                Vec::new()
            }
        }
    }

    /// Convenience: execute a statement (INSERT/UPDATE/DELETE) — mirrors
    /// `execute(String)`.
    pub fn execute(&self, sql: &str) {
        if let Err(e) = self.runtime.block_on(sqlx::query(sql).execute(&self.pool)) {
            Self::log_error(e.to_string());
        }
    }

    /// Convenience: execute an INSERT and return the generated key — mirrors
    /// the `getGeneratedKeys()` pattern used across the DAOs.
    pub fn execute_insert(&self, sql: &str) -> Option<i64> {
        match self.runtime.block_on(sqlx::query(sql).execute(&self.pool)) {
            Ok(result) => Some(result.last_insert_id() as i64),
            Err(e) => {
                Self::log_error(e.to_string());
                None
            }
        }
    }

    /// Convenience: `SELECT <col> ...` single value — mirrors `getString`.
    pub fn get_string(&self, sql: &str, column: &str) -> Option<String> {
        self.query_all(sql)
            .into_iter()
            .next()
            .and_then(|row| row.try_get::<String, _>(column).ok())
    }
}

/// Typed column access helpers (mirrors `ResultSet.getX`).
pub trait RowGetters {
    fn str(&self, column: &str) -> Option<String>;
    fn i32(&self, column: &str) -> Option<i32>;
    fn i64(&self, column: &str) -> Option<i64>;
    fn f64(&self, column: &str) -> Option<f64>;
    fn bool(&self, column: &str) -> Option<bool>;
}

impl RowGetters for MySqlRow {
    fn str(&self, column: &str) -> Option<String> {
        self.try_get::<String, _>(column).ok()
    }
    fn i32(&self, column: &str) -> Option<i32> {
        self.try_get::<i32, _>(column).ok()
    }
    fn i64(&self, column: &str) -> Option<i64> {
        self.try_get::<i64, _>(column).ok()
    }
    fn f64(&self, column: &str) -> Option<f64> {
        self.try_get::<f64, _>(column).ok()
    }
    fn bool(&self, column: &str) -> Option<bool> {
        self.try_get::<bool, _>(column).ok()
    }
}
