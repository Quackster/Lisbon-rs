//! Mirrors `org.alexdev.http.dao.housekeeping.HousekeepingPlayerDao`.

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::dao::storage::Storage;
use lisbon_server::game::player::player_details::PlayerDetails;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct HousekeepingPlayerDao;

impl HousekeepingPlayerDao {
    /// Mirrors `getPlayers(int, boolean, String)`.
    pub fn get_players(page: i32, zero_coins_flag: bool, sort_by: &str) -> Vec<PlayerDetails> {
        let mut players = Vec::new();
        let rows = 25;
        let next_offset = page * rows;

        if next_offset >= 0 {
            let zero_coins_clause = if zero_coins_flag {
                " AND credits = 0 "
            } else {
                ""
            };

            for row in Storage::get_storage().query_all(
                &format!("SELECT * FROM users WHERE username <> '' {zero_coins_clause}ORDER BY {sort_by} DESC LIMIT {rows} OFFSET {next_offset}"),
            ) {
                let mut details = PlayerDetails::new();
                PlayerDao::fill(&mut details, &row);
                players.push(details);
            }
        }

        players
    }

    /// Mirrors `search(String, String, String)`.
    pub fn search(search_type: &str, field: &str, input: &str) -> Vec<PlayerDetails> {
        let mut players = Vec::new();

        let (sql, value) = match search_type {
            "contains" => (
                "SELECT * FROM users WHERE ".to_string() + field + " LIKE",
                format!("%{input}%"),
            ),
            "starts_with" => (
                "SELECT * FROM users WHERE ".to_string() + field + " LIKE",
                format!("{input}%"),
            ),
            "ends_with" => (
                "SELECT * FROM users WHERE ".to_string() + field + " LIKE",
                format!("%{input}"),
            ),
            _ => (
                "SELECT * FROM users WHERE ".to_string() + field + " =",
                input.to_string(),
            ),
        };

        for row in Storage::get_storage().query_all(&format!("{sql} '{}'", escape(&value))) {
            let mut details = PlayerDetails::new();
            PlayerDao::fill(&mut details, &row);
            players.push(details);
        }

        players
    }
}
