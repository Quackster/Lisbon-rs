//! Mirrors `org.alexdev.http.dao.CommunityDao`.

use lisbon_server::dao::storage::{RowGetters, Storage};
use lisbon_server::game::groups::group::Group;

use super::group_discussion_dao::{DiscussionTopic, GroupDiscussionDao};

pub struct CommunityDao;

impl CommunityDao {
    /// Mirrors `getHotGroups(int, int)`.
    // Returns an ordered vec of (Group, popularity) pairs, mirroring the
    // Java `Map<Group, Integer>`.
    pub fn get_hot_groups(limit: i32, offset: i32) -> Vec<(Group, i32)> {
        let mut hot_groups = Vec::new();

        for row in Storage::get_storage().query_all(&format!(
            "SELECT *, (SELECT COUNT(*) FROM groups_memberships WHERE group_id = id AND (groups_memberships.created_at between (CURDATE() - INTERVAL 1 MONTH ) and CURDATE())) AS popularity FROM groups_details WHERE groups_details.created_at between (CURDATE() - INTERVAL 1 MONTH) and CURDATE() LIMIT {limit} OFFSET {offset}"
        )) {
            hot_groups.push((
                Group::new(
                    row.i32("id").unwrap_or(0),
                    row.str("name").unwrap_or_default().as_str(),
                    row.str("description").unwrap_or_default().as_str(),
                    row.i32("owner_id").unwrap_or(0),
                    row.i32("room_id").unwrap_or(0),
                    row.str("badge").unwrap_or_default().as_str(),
                    row.bool("recommended").unwrap_or(false),
                    row.str("background").unwrap_or_default().as_str(),
                    row.i32("views").unwrap_or(0),
                    row.i32("topics").unwrap_or(0),
                    row.i32("group_type").unwrap_or(0),
                    row.i32("forum_type").unwrap_or(0),
                    row.i32("forum_premission").unwrap_or(0),
                    row.str("alias").unwrap_or_default().as_str(),
                    row.i64("created_at").unwrap_or(0),
                ),
                row.i32("popularity").unwrap_or(0),
            ));
        }

        hot_groups
    }

    /// Mirrors `getRecentDiscussions(int, int)`.
    pub fn get_recent_discussions(limit: i32, offset: i32) -> Vec<DiscussionTopic> {
        let mut discussion_list = Vec::new();

        for row in Storage::get_storage().query_all(&format!(
            "SELECT DISTINCT cms_forum_threads.*, cms_forum_replies.created_at AS last_message_at, cms_forum_replies.id AS last_reply_id, '' AS creator_name, 0 AS creator_id, '' AS last_reply_name, (SELECT COUNT(*) FROM cms_forum_replies WHERE cms_forum_replies.thread_id = cms_forum_threads.id) AS reply_count, 0 as has_read FROM cms_forum_replies INNER JOIN cms_forum_threads ON cms_forum_threads.id = cms_forum_replies.thread_id WHERE cms_forum_replies.id = (SELECT MAX(id) FROM cms_forum_replies WHERE cms_forum_replies.thread_id = cms_forum_threads.id) ORDER BY cms_forum_replies.created_at DESC LIMIT {limit} OFFSET {offset}"
        )) {
            discussion_list.push(GroupDiscussionDao::fill(&row));
        }

        discussion_list
    }
}
