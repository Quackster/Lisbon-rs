//! Mirrors `net.h4bbo.lisbon.dao.mysql.InfobusDao`.

use std::collections::HashMap;

use sqlx::mysql::MySqlRow;

use crate::dao::storage::{RowGetters, Storage};
use crate::game::infobus::infobus_poll::InfobusPoll;
use crate::game::infobus::infobus_poll_data::InfobusPollData;
use crate::lisbon::Lisbon;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct InfobusDao;

impl InfobusDao {
    /// Mirrors `getInfobusPolls()`.
    pub fn get_infobus_polls() -> Vec<InfobusPoll> {
        let mut polls = Vec::new();

        for row in Storage::get_storage()
            .query_all("SELECT * FROM infobus_polls ORDER BY created_at DESC")
        {
            if let Some(poll) = Self::fill(&row) {
                polls.push(poll);
            }
        }

        polls
    }

    /// Mirrors `createInfobusPoll(int, InfobusPollData)`.
    pub fn create_infobus_poll(initiated_by: i32, poll_data: &InfobusPollData) -> i32 {
        let json = Lisbon::get_gson().to_json(poll_data);

        match Storage::get_storage().execute_insert(&format!(
            "INSERT INTO infobus_polls (initiated_by, poll_data) VALUES ({initiated_by}, '{}')",
            escape(&json)
        )) {
            Some(id) => id as i32,
            None => -1,
        }
    }

    /// Mirrors `delete(int)`.
    pub fn delete(id: i32) {
        Storage::get_storage().execute(&format!("DELETE FROM infobus_polls WHERE id = {id}"));
    }

    /// Mirrors `get(int)`.
    pub fn get(id: i32) -> Option<InfobusPoll> {
        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM infobus_polls WHERE id = {id} LIMIT 1"))
        {
            if let Some(poll) = Self::fill(&row) {
                return Some(poll);
            }
        }

        None
    }

    /// Mirrors `saveInfobusPoll(int, InfobusPollData)`.
    pub fn save_infobus_poll(id: i32, poll_data: &InfobusPollData) {
        let json = Lisbon::get_gson().to_json(poll_data);

        Storage::get_storage().execute(&format!(
            "UPDATE infobus_polls SET poll_data = '{}' WHERE id = {id}",
            escape(&json)
        ));
    }

    /// Mirrors `addAnswer(int, int, int)`.
    pub fn add_answer(poll_id: i32, answer: i32, user_id: i32) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO infobus_polls_answers (user_id, poll_id, answer) VALUES ({user_id}, {poll_id}, {answer})"
        ));
    }

    /// Mirrors `hasAnswer(int, int)`.
    pub fn has_answer(poll_id: i32, user_id: i32) -> bool {
        for _row in Storage::get_storage()
            .query_all(
                &format!(
                    "SELECT * FROM infobus_polls_answers WHERE user_id = {user_id} AND poll_id = {poll_id}"
                ),
            )
        {
            return true;
        }

        false
    }

    /// Mirrors `getAnswers(int)`.
    pub fn get_answers(poll_id: i32) -> HashMap<i32, i32> {
        let mut answers = HashMap::new();

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT COUNT(*) AS votes, answer FROM infobus_polls_answers WHERE poll_id = {poll_id} GROUP BY answer"
            ),
        ) {
            if let (Some(answer), Some(votes)) = (row.i32("answer"), row.i32("votes")) {
                answers.insert(answer, votes);
            }
        }

        answers
    }

    /// Mirrors `clearAnswers(int)`.
    pub fn clear_answers(poll_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM infobus_polls_answers WHERE poll_id = {poll_id}"
        ));
    }

    /// Mirrors the private `fill(ResultSet)`.
    fn fill(row: &MySqlRow) -> Option<InfobusPoll> {
        let (id, initiated_by, created_at) = (row.i32("id")?, row.i32("initiated_by")?, row.i64("created_at")?);
        let poll_data = match row.str("poll_data") {
            Some(json) => Lisbon::get_gson()
                .from_json::<InfobusPollData>(&json)
                .unwrap_or_else(|| InfobusPollData::new("")),
            None => InfobusPollData::new(""),
        };

        Some(InfobusPoll::new(id, initiated_by, poll_data, created_at))
    }
}
