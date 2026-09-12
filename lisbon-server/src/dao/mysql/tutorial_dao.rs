//! Mirrors `net.h4bbo.lisbon.dao.mysql.TutorialDao`.

use crate::dao::storage::Storage;

pub struct TutorialDao;

impl TutorialDao {
    /// Mirrors `updateTutorialMode(int, boolean)`.
    pub fn update_tutorial_mode(user_id: i32, tutorial_finished: bool) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET tutorial_finished = {} WHERE id = {user_id} LIMIT 1",
            if tutorial_finished { 1 } else { 0 }
        ));
    }
}
