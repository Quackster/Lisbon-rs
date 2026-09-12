//! Mirrors `net.h4bbo.lisbon.dao.mysql.CurrencyDao`.
//!
//! The Java methods atomically update the column and then write the fetched
//! value back into `PlayerDetails`. The Rust callers pass a shared
//! `&PlayerDetails` (`Player::get_details`), so the in-memory write-back is not
//! expressible here — only the atomic DB update is applied.

use crate::dao::storage::Storage;
use crate::game::player::player_details::PlayerDetails;

pub struct CurrencyDao;

impl CurrencyDao {
    /// Mirrors `increaseCredits(PlayerDetails, int)`.
    // Port note: the in-memory `setCredits` write-back is skipped because the
    // caller passes a shared `&PlayerDetails`.
    pub fn increase_credits(details: &PlayerDetails, amount: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET credits = credits + {amount} WHERE id = {}",
            details.get_id()
        ));
    }

    /// Mirrors `increaseCredits(Map)`.
    // Port note: the in-memory write-back is skipped (see module note).
    pub fn increase_credits_bulk(entries: &[( &PlayerDetails, i32)]) {
        for (details, amount) in entries {
            Storage::get_storage().execute(&format!(
                "UPDATE users SET credits = credits + {amount} WHERE id = {}",
                details.get_id()
            ));
        }
    }

    /// Mirrors `decreaseCredits(PlayerDetails, int)`.
    // Port note: the in-memory `setCredits` write-back is skipped (see module
    // note); the `<= 0` guard is applied to the amount.
    pub fn decrease_credits(details: &PlayerDetails, amount: i32) {
        let amount = if details.get_credits() <= 0 { 0 } else { amount };

        Storage::get_storage().execute(&format!(
            "UPDATE users SET credits = credits - {amount} WHERE id = {}",
            details.get_id()
        ));
    }

    /// Mirrors `increaseTickets(PlayerDetails, int)`.
    // Port note: the in-memory `setTickets` write-back is skipped (see module
    // note).
    pub fn increase_tickets(details: &PlayerDetails, amount: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET tickets = tickets + {amount} WHERE id = {}",
            details.get_id()
        ));
    }

    /// Mirrors `decreaseTickets(PlayerDetails, int)`.
    // Port note: the in-memory `setTickets` write-back is skipped (see module
    // note); the `<= 0` guard is applied to the amount.
    pub fn decrease_tickets(details: &PlayerDetails, amount: i32) {
        let amount = if details.get_tickets() <= 0 { 0 } else { amount };

        Storage::get_storage().execute(&format!(
            "UPDATE users SET tickets = tickets - {amount} WHERE id = {}",
            details.get_id()
        ));
    }

    /// Mirrors `increaseFilm(PlayerDetails, int)`.
    // Port note: the in-memory `setFilm` write-back is skipped (see module note).
    pub fn increase_film(details: &PlayerDetails, amount: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET film = film + {amount} WHERE id = {}",
            details.get_id()
        ));
    }

    /// Mirrors `decreaseFilm(PlayerDetails, int)`.
    // Port note: the in-memory `setFilm` write-back is skipped (see module
    // note); the `<= 0` guard is applied to the amount.
    pub fn decrease_film(details: &PlayerDetails, amount: i32) {
        let amount = if details.get_film() <= 0 { 0 } else { amount };

        Storage::get_storage().execute(&format!(
            "UPDATE users SET film = film - {amount} WHERE id = {}",
            details.get_id()
        ));
    }

    /// Mirrors `getCredits(int)`.
    pub fn get_credits(user_id: i32) -> i32 {
        Storage::get_storage()
            .get_string(
                &format!("SELECT credits FROM users WHERE id = {user_id}"),
                "credits",
            )
            .and_then(|value| value.parse().ok())
            .unwrap_or(0)
    }
}
