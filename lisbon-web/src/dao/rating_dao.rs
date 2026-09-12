//! Mirrors `org.alexdev.http.dao.RatingDao`.

use lisbon_server::dao::storage::{RowGetters, Storage};

pub struct RatingDao;

impl RatingDao {
    /// Mirrors `rate(int, int, int)`.
    pub fn rate(user_id: i32, home_id: i32, rating: i32) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO `homes_ratings` (user_id, home_id, rating) VALUES ({user_id}, {home_id}, {rating})"
        ));
    }

    /// Mirrors `deleteRating(int)`.
    pub fn delete_rating(home_id: i32) {
        Storage::get_storage()
            .execute(&format!("DELETE FROM `homes_ratings` WHERE home_id = {home_id}"));
    }

    /// Mirrors `hasRated(int, int)`.
    pub fn has_rated(user_id: i32, home_id: i32) -> bool {
        for _row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM `homes_ratings` WHERE user_id = {user_id} AND home_id = {home_id}"))
        {
            return true;
        }

        false
    }

    /// Mirrors `getAverageRating(int)`.
    pub fn get_average_rating(home_id: i32) -> f64 {
        let mut rating = 0.0;

        for row in Storage::get_storage()
            .query_all(&format!("SELECT AVG(rating) AS 'average_rating' FROM homes_ratings WHERE home_id = {home_id}"))
        {
            if let Some(value) = row.f64("average_rating") {
                rating = value;
            }
        }

        rating
    }

    /// Mirrors `getUserRated(int, int)`.
    pub fn get_user_rated(user_id: i32, home_id: i32) -> i32 {
        let mut rating = 0;

        for row in Storage::get_storage()
            .query_all(&format!("SELECT rating FROM homes_ratings WHERE user_id = {user_id} AND home_id = {home_id}"))
        {
            if let Some(value) = row.i32("rating") {
                rating = value;
            }
        }

        rating
    }

    /// Mirrors `getHighVoteCount(int)`.
    pub fn get_high_vote_count(home_id: i32) -> i32 {
        let mut count = 0;

        for row in Storage::get_storage().query_all(
            &format!("SELECT COUNT(rating) as 'votes' FROM homes_ratings WHERE home_id = {home_id} AND rating >= 4"),
        ) {
            if let Some(value) = row.i32("votes") {
                count = value;
            }
        }

        count
    }

    /// Mirrors `getVoteCount(int)`.
    pub fn get_vote_count(home_id: i32) -> i32 {
        let mut count = 0;

        for row in Storage::get_storage()
            .query_all(&format!("SELECT COUNT(rating) as 'votes' FROM homes_ratings WHERE home_id = {home_id}"))
        {
            if let Some(value) = row.i32("votes") {
                count = value;
            }
        }

        count
    }
}
