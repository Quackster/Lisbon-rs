//! Mirrors `org.alexdev.http.dao.ReplyDao`.

use lisbon_server::dao::storage::Storage;

use super::group_discussion_dao::DiscussionReply;

pub struct ReplyDao;

impl ReplyDao {
    /// Mirrors `read(int, List<DiscussionReply>)`.
    // Port note: the JDBC batch (`addBatch`/`executeBatch`) is executed as
    // one insert per reply; the `setAutoCommit(false)` transaction wrapper
    // has no equivalent in the synchronous `Storage` API.
    pub fn read(user_id: i32, replies: &[DiscussionReply]) {
        for reply in replies {
            Storage::get_storage().execute(&format!(
                "INSERT IGNORE INTO `cms_forums_read_replies` (user_id, reply_id) VALUES ({user_id}, {rid})",
                rid = reply.id
            ));
        }
    }

    /// Mirrors `hasRead(int, int)`.
    pub fn has_read(user_id: i32, reply_id: i32) -> bool {
        for _row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM `cms_forums_read_replies` WHERE user_id = {user_id} AND reply_id = {reply_id}"))
        {
            return true;
        }

        false
    }
}
