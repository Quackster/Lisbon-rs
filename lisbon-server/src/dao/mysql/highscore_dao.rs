//! Mirrors `net.h4bbo.lisbon.dao.mysql.HighscoreDao`.

use crate::dao::storage::{RowGetters, Storage};
use crate::game::games::enums::game_type::GameType;
use crate::game::games::history::score_entry::ScoreEntry;

pub struct HighscoreDao;

impl HighscoreDao {
    /// Mirrors `getScores(int, GameType, int, boolean)`.
    pub fn get_scores(limit: i32, game_type: GameType, page: i32, view_monthly: bool) -> Vec<ScoreEntry> {
        let suffix = if view_monthly { "month" } else { "all_time" };
        let column = match game_type {
            GameType::Battleball => format!("battleball_score_{suffix}"),
            GameType::Snowstorm => format!("snowstorm_score_{suffix}"),
            GameType::WobbleSquabble => format!("wobble_squabble_score_{suffix}"),
        };

        let offset = (page - 1) * limit;
        let sql = format!(
            "SELECT users.username AS username, users_statistics.* FROM users_statistics INNER JOIN users ON users.id = users_statistics.user_id WHERE {column} > 0 AND ((SELECT COUNT(*) FROM users_bans WHERE banned_value = users.id AND ban_type = 'USER_ID' AND NOW() > banned_until AND is_active = 1) = 0) ORDER BY {column} DESC LIMIT {offset}, {limit}"
        );

        let mut score_entry_list = Vec::new();
        let mut position = offset + 1;

        for row in Storage::get_storage().query_all(&sql) {
            if let (Some(username), Some(score)) = (row.str("username"), row.i64(&column)) {
                score_entry_list.push(ScoreEntry::new(&username, score, position));
            }
            position += 1;
        }

        score_entry_list
    }
}
