//! Mirrors `org.alexdev.http.dao.GroupDiscussionDao`.

use std::collections::HashMap;

use sqlx::mysql::MySqlRow;

use lisbon_server::dao::storage::{RowGetters, Storage};

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub use crate::game::groups::discussion_reply::DiscussionReply;
pub use crate::game::groups::discussion_topic::DiscussionTopic;

pub struct GroupDiscussionDao;

impl GroupDiscussionDao {
    /// Mirrors `MAX_UNREAD_DAYS`.
    pub const MAX_UNREAD_DAYS: i32 = 7;

    /// Mirrors `getNewGroupMessages(int, long)`.
    pub fn get_new_group_messages(user_id: i32, last_online: i64) -> (i32, HashMap<String, String>) {
        let _ = last_online;
        let mut group_data: HashMap<String, String> = HashMap::new();
        let mut groups: HashMap<String, String> = HashMap::new();
        let mut pending_members = 0;

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT groups_details.id AS group_id, groups_details.name AS group_name FROM groups_memberships RIGHT JOIN groups_details ON groups_memberships.group_id = groups_details.id WHERE (owner_id = {user_id} OR (groups_memberships.user_id = {user_id} AND groups_memberships.is_pending = 0 AND (groups_memberships.member_rank = '1' OR groups_memberships.member_rank = '2' OR groups_memberships.member_rank = '3')))"
            ),
        ) {
            if let (Some(group_id), Some(group_name)) = (row.i32("group_id"), row.str("group_name")) {
                group_data.insert(group_id.to_string(), group_name);
            }
        }

        if !group_data.is_empty() {
            let ids = group_data.keys().cloned().collect::<Vec<String>>().join(",");

            for row in Storage::get_storage().query_all(&format!(
                "SELECT group_id FROM cms_forum_replies INNER JOIN cms_forum_threads ON cms_forum_threads.id = cms_forum_replies.thread_id WHERE cms_forum_threads.group_id IN ({ids}) AND (DATEDIFF(NOW(), cms_forum_replies.created_at) <= {}) AND NOT EXISTS (SELECT * FROM cms_forums_read_replies WHERE cms_forums_read_replies.reply_id = (SELECT MAX(id) FROM cms_forum_replies WHERE cms_forum_replies.thread_id = cms_forum_threads.id) AND cms_forums_read_replies.user_id = {user_id}) GROUP BY group_id",
                Self::MAX_UNREAD_DAYS
            )) {
                if let Some(group_id) = row.i32("group_id") {
                    let key = group_id.to_string();

                    if !groups.contains_key(&key) {
                        let name = group_data
                            .get(&key)
                            .cloned()
                            .unwrap_or_default();
                        groups.insert(key.clone(), name);
                    }

                    pending_members += 1;
                }
            }
        }

        (pending_members, groups)
    }

    /// Mirrors `getDiscussions(int, int, int, int)`.
    pub fn get_discussions(group_id: i32, page: i32, items_per_page: i32, user_id: i32) -> Vec<DiscussionTopic> {
        let mut discussion_list = Vec::new();

        for row in Storage::get_storage().query_all(&format!(
            "SELECT cms_forum_threads.*, cms_forum_replies.created_at AS last_message_at, cms_forum_replies.id AS last_reply_id, creator.username AS creator_name, creator.id AS creator_id, replier.username AS last_reply_name, (SELECT COUNT(*) FROM cms_forum_replies WHERE cms_forum_replies.thread_id = cms_forum_threads.id) AS reply_count, {} FROM cms_forum_replies INNER JOIN cms_forum_threads ON cms_forum_threads.id = cms_forum_replies.thread_id INNER JOIN users replier ON cms_forum_replies.poster_id = replier.id INNER JOIN users creator ON cms_forum_threads.poster_id = creator.id WHERE cms_forum_replies.id = (SELECT MAX(id) FROM cms_forum_replies WHERE cms_forum_replies.thread_id = cms_forum_threads.id) AND group_id = {group_id} ORDER BY is_stickied DESC, cms_forum_replies.created_at DESC LIMIT {}, {}",
            Self::has_read_clause("last_reply_id", user_id),
            (page - 1) * items_per_page,
            items_per_page
        )) {
            discussion_list.push(Self::fill(&row));
        }

        discussion_list
    }

    /// Mirrors `countDiscussions(int)`.
    pub fn count_discussions(group_id: i32) -> i32 {
        let mut discussions = 0;

        for row in Storage::get_storage().query_all(
            &format!("SELECT COUNT(*) AS discussion_count FROM cms_forum_threads WHERE cms_forum_threads.group_id = {group_id}"),
        ) {
            if let Some(value) = row.i32("discussion_count") {
                discussions = value;
            }
        }

        discussions
    }

    /// Mirrors `getReplies(int, int, int, int)`.
    pub fn get_replies(group_id: i32, page: i32, items_per_page: i32, user_id: i32) -> Vec<DiscussionReply> {
        let mut reply_list = Vec::new();

        for row in Storage::get_storage().query_all(&format!(
            "SELECT cms_forum_replies.*, users.id AS user_id, users.figure AS figure, users.username AS username, users.is_online AS is_online, IFNULL(users.favourite_group, 0) as group_id, (SELECT users_badges.badge FROM users_badges WHERE users_badges.user_id = users.id AND users_badges.equipped ORDER BY slot_id ASC LIMIT 1) AS equipped_badge, (SELECT groups_details.badge FROM groups_details WHERE groups_details.id = users.favourite_group) AS group_badge, (SELECT COUNT(*) FROM cms_forum_replies WHERE cms_forum_replies.poster_id = users.id) AS forum_messages, {} FROM cms_forum_replies INNER JOIN users ON users.id = cms_forum_replies.poster_id WHERE thread_id = {group_id} ORDER BY cms_forum_replies.created_at ASC LIMIT {}, {}",
            Self::has_read_clause("cms_forum_replies.id", user_id),
            (page - 1) * items_per_page,
            items_per_page
        )) {
            reply_list.push(Self::fill_reply(&row));
        }

        reply_list
    }

    /// Mirrors `getFirstReply(int)`.
    pub fn get_first_reply(discussion_id: i32) -> i32 {
        let mut id = 0;

        for row in Storage::get_storage()
            .query_all(&format!("SELECT id FROM cms_forum_replies WHERE thread_id = {discussion_id} ORDER BY created_at ASC LIMIT 1"))
        {
            if let Some(value) = row.i32("id") {
                id = value;
            }
        }

        id
    }

    /// Mirrors `countReplies(int)`.
    pub fn count_replies(group_id: i32) -> i32 {
        let mut replies = 0;

        for row in Storage::get_storage().query_all(
            &format!("SELECT COUNT(*) AS replies FROM cms_forum_replies INNER JOIN users ON users.id = cms_forum_replies.poster_id WHERE thread_id = {group_id} ORDER BY cms_forum_replies.created_at ASC"),
        ) {
            if let Some(value) = row.i32("replies") {
                replies = value;
            }
        }

        replies
    }

    /// Mirrors `createDiscussion(int, int, String)`.
    pub fn create_discussion(group_id: i32, poster_id: i32, topic_title: &str) -> i32 {
        Storage::get_storage()
            .execute_insert(&format!(
                "INSERT INTO cms_forum_threads (topic_title, poster_id, group_id) VALUES ('{title}', {poster_id}, {group_id})",
                title = escape(topic_title)
            ))
            .map(|id| id as i32)
            .unwrap_or(0)
    }

    /// Mirrors `getDiscussion(int, int, int)`.
    pub fn get_discussion(group_id: i32, discussion_id: i32, user_id: i32) -> Option<DiscussionTopic> {
        for row in Storage::get_storage().query_all(&format!(
            "SELECT cms_forum_threads.*, cms_forum_replies.created_at AS last_message_at, cms_forum_replies.id AS last_reply_id, creator.username AS creator_name, creator.id AS creator_id, replier.username AS last_reply_name, (SELECT COUNT(*) FROM cms_forum_replies WHERE cms_forum_replies.thread_id = cms_forum_threads.id) AS reply_count, {} FROM cms_forum_replies INNER JOIN cms_forum_threads ON cms_forum_threads.id = cms_forum_replies.thread_id INNER JOIN users replier ON cms_forum_replies.poster_id = replier.id INNER JOIN users creator ON cms_forum_threads.poster_id = creator.id WHERE cms_forum_threads.group_id = {group_id} AND cms_forum_threads.id = {discussion_id} ORDER BY cms_forum_replies.created_at DESC LIMIT 1",
            Self::has_read_clause("last_reply_id", user_id)
        )) {
            return Some(Self::fill(&row));
        }

        None
    }

    /// Mirrors `getReply(int, int, int)`.
    pub fn get_reply(discussion_id: i32, reply_id: i32, user_id: i32) -> Option<DiscussionReply> {
        for row in Storage::get_storage().query_all(&format!(
            "SELECT cms_forum_replies.*, users.id AS user_id, users.figure AS figure, users.username AS username, users.is_online AS is_online, IFNULL(users.favourite_group, 0) as group_id, (SELECT users_badges.badge FROM users_badges WHERE users_badges.user_id = users.id AND users_badges.equipped ORDER BY slot_id ASC LIMIT 1) AS equipped_badge, (SELECT groups_details.badge FROM groups_details WHERE groups_details.id = users.favourite_group) AS group_badge, (SELECT COUNT(*) FROM cms_forum_replies WHERE cms_forum_replies.poster_id = users.id) AS forum_messages, {} FROM cms_forum_replies INNER JOIN users ON users.id = cms_forum_replies.poster_id WHERE thread_id = {discussion_id} AND cms_forum_replies.id = {reply_id} LIMIT 1",
            Self::has_read_clause("cms_forum_replies.id", user_id)
        )) {
            return Some(Self::fill_reply(&row));
        }

        None
    }

    /// Mirrors `getLatestReply(int)`.
    pub fn get_latest_reply(user_id: i32) -> Option<DiscussionReply> {
        for row in Storage::get_storage().query_all(&format!(
            "SELECT cms_forum_replies.*, users.id AS user_id, users.figure AS figure, users.username AS username, users.is_online AS is_online, IFNULL(users.favourite_group, 0) as group_id, (SELECT users_badges.badge FROM users_badges WHERE users_badges.user_id = users.id AND users_badges.equipped ORDER BY slot_id ASC LIMIT 1) AS equipped_badge, (SELECT groups_details.badge FROM groups_details WHERE groups_details.id = users.favourite_group) AS group_badge, (SELECT COUNT(*) FROM cms_forum_replies WHERE cms_forum_replies.poster_id = users.id) AS forum_messages, {} FROM cms_forum_replies INNER JOIN users ON users.id = cms_forum_replies.poster_id WHERE poster_id = {user_id} ORDER BY created_at DESC LIMIT 1",
            Self::has_read_clause("cms_forum_replies.id", user_id)
        )) {
            return Some(Self::fill_reply(&row));
        }

        None
    }

    /// Mirrors `countUserReplies(int)`.
    pub fn count_user_replies(user_id: i32) -> i32 {
        let mut replies = 0;

        for row in Storage::get_storage()
            .query_all(&format!("SELECT COUNT(*) FROM cms_forum_replies WHERE cms_forum_replies.poster_id = {user_id}"))
        {
            replies = row.i64("COUNT(*)").unwrap_or(0) as i32;
        }

        replies
    }

    /// Mirrors `getDisplayBadges(int)`.
    pub fn get_display_badges(user_id: i32) -> (Option<String>, Option<String>) {
        let mut badges: (Option<String>, Option<String>) = (None, None);

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT (SELECT users_badges.badge FROM users_badges WHERE users_badges.user_id = users.id AND users_badges.equipped ORDER BY slot_id ASC LIMIT 1) AS equipped_badge, (SELECT groups_details.badge FROM groups_details WHERE groups_details.id = users.favourite_group) AS group_badge FROM users WHERE users.id = {user_id}"
            ),
        ) {
            badges = (row.str("equipped_badge"), row.str("group_badge"));
        }

        badges
    }

    /// Mirrors `createReplies(int, int, String)`.
    pub fn create_replies(thread_id: i32, poster_id: i32, message: &str) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO cms_forum_replies (thread_id, message, poster_id) VALUES ({thread_id}, '{m}', {poster_id})",
            m = escape(message)
        ));
    }

    /// Mirrors `deleteDiscussion(int, int)`.
    pub fn delete_discussion(group_id: i32, topic_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM cms_forum_threads WHERE id = {topic_id} AND group_id = {group_id}"
        ));
        Storage::get_storage().execute(&format!(
            "DELETE FROM cms_forum_replies WHERE thread_id = {topic_id}"
        ));
    }

    /// Mirrors `saveDiscussion(DiscussionTopic)`.
    pub fn save_discussion(discussion_topic: &DiscussionTopic) {
        Storage::get_storage().execute(&format!(
            "UPDATE cms_forum_threads SET topic_title = '{title}', is_open = {open}, is_stickied = {stickied} WHERE id = {id}",
            title = escape(&discussion_topic.topic_title),
            open = discussion_topic.is_open as i32,
            stickied = discussion_topic.is_stickied as i32,
            id = discussion_topic.id
        ));
    }

    /// Mirrors `saveReply(DiscussionReply)`.
    pub fn save_reply(discussion_reply: &DiscussionReply) {
        Storage::get_storage().execute(&format!(
            "UPDATE cms_forum_replies SET is_deleted = {deleted}, is_edited = {edited}, message = '{message}' WHERE id = {id}",
            deleted = discussion_reply.is_deleted as i32,
            edited = discussion_reply.is_edited as i32,
            message = escape(&discussion_reply.message),
            id = discussion_reply.id
        ));
    }

    /// Mirrors `deleteReply(DiscussionReply)`.
    pub fn delete_reply(discussion_reply: &DiscussionReply) {
        let id = discussion_reply.id;
        Storage::get_storage().execute(&format!("DELETE FROM cms_forum_replies WHERE id = {id}"));
    }

    /// Mirrors `incrementViews(int)`.
    pub fn increment_views(discussion_id: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE cms_forum_threads SET views = views + 1 WHERE id = {discussion_id}"
        ));
    }

    /// Mirrors `fill(ResultSet)`.
    pub fn fill(row: &MySqlRow) -> DiscussionTopic {
        let has_read = row.i32("has_read").unwrap_or(0) != 0;

        DiscussionTopic::new(
            row.i32("id").unwrap_or(0),
            row.i32("group_id").unwrap_or(0),
            &row.str("topic_title").unwrap_or_default(),
            row.i32("reply_count").unwrap_or(0),
            row.bool("is_open").unwrap_or(false),
            row.bool("is_stickied").unwrap_or(false),
            row.i32("views").unwrap_or(0),
            row.i64("created_at").unwrap_or(0),
            row.i64("last_message_at").unwrap_or(0),
            row.i32("creator_id").unwrap_or(0),
            &row.str("creator_name").unwrap_or_default(),
            &row.str("last_reply_name").unwrap_or_default(),
            has_read,
        )
    }

    /// Mirrors `fillReply(ResultSet)`.
    fn fill_reply(row: &MySqlRow) -> DiscussionReply {
        let has_read = row.i32("has_read").unwrap_or(0) != 0;

        DiscussionReply::new(
            row.i32("id").unwrap_or(0),
            row.i32("user_id").unwrap_or(0),
            &row.str("message").unwrap_or_default(),
            &row.str("figure").unwrap_or_default(),
            &row.str("username").unwrap_or_default(),
            row.bool("is_online").unwrap_or(false),
            row.str("equipped_badge"),
            row.i32("group_id").unwrap_or(0),
            row.str("group_badge"),
            row.i32("forum_messages").unwrap_or(0),
            row.bool("is_edited").unwrap_or(false),
            row.bool("is_deleted").unwrap_or(false),
            row.i64("created_at").unwrap_or(0),
            row.i64("modified_at").unwrap_or(0),
            has_read,
        )
    }

    /// The `has_read` subquery clause, shared by the topic/reply SELECTs.
    fn has_read_clause(reply_id_column: &str, user_id: i32) -> String {
        if user_id > 0 {
            format!(
                "(SELECT COUNT(*) FROM cms_forums_read_replies WHERE (cms_forums_read_replies.reply_id = {reply_id_column} AND cms_forums_read_replies.user_id = {user_id}) OR (DATEDIFF(NOW(), cms_forum_replies.created_at) > {days}) ) AS has_read",
                days = Self::MAX_UNREAD_DAYS
            )
        } else {
            "0 as has_read".to_string()
        }
    }
}
