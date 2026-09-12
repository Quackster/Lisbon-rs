//! Mirrors `org.alexdev.http.game.groups.DiscussionTopic`.

use lisbon_server::game::groups::group::Group;
use lisbon_server::game::groups::group_forum_type::GroupForumType;
use lisbon_server::game::groups::group_member::GroupMember;
use lisbon_server::game::groups::group_member_rank::GroupMemberRank;
use lisbon_server::game::groups::group_permission_type::GroupPermissionType;
use lisbon_server::game::wordfilter::wordfilter_manager::WordfilterManager;
use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::date_util::DateUtil;

#[derive(Clone, Debug, serde::Serialize)]
pub struct DiscussionTopic {
    pub id: i32,
    pub creator_name: String,
    pub creator_id: i32,
    pub last_reply_name: String,
    pub reply_count: i32,
    pub group_id: i32,
    pub topic_title: String,
    pub is_open: bool,
    pub is_stickied: bool,
    pub views: i32,
    pub created_at: i64,
    pub last_message_at: i64,
    pub is_new: bool,
}

impl DiscussionTopic {
    /// Mirrors the `DiscussionTopic(int, int, String, int, boolean, boolean, int, Time, Time, int, String, String, boolean)` constructor
    /// (the `Time` arguments are passed as unix seconds).
    pub fn new(
        id: i32,
        group_id: i32,
        topic_title: &str,
        reply_count: i32,
        is_open: bool,
        is_stickied: bool,
        views: i32,
        created_at: i64,
        last_message_at: i64,
        creator_id: i32,
        creator_name: &str,
        last_reply_name: &str,
        has_read: bool,
    ) -> Self {
        Self {
            id,
            group_id,
            topic_title: topic_title.to_string(),
            reply_count,
            is_open,
            is_stickied,
            views,
            created_at,
            last_message_at,
            creator_id,
            creator_name: creator_name.to_string(),
            last_reply_name: last_reply_name.to_string(),
            is_new: !has_read,
        }
    }

    /// Mirrors `canPostReply(Group, GroupMember)`.
    pub fn can_post_reply(&self, group: &Group, member: Option<&GroupMember>) -> bool {
        if !self.is_open {
            return false;
        }

        if group.get_forum_type() == GroupForumType::Private
            || group.get_forum_permission() == GroupPermissionType::MemberOnly
            || group.get_forum_permission() == GroupPermissionType::AdminOnly
        {
            let Some(member) = member else {
                return false;
            };

            if group.get_forum_permission() == GroupPermissionType::AdminOnly {
                let Some(member_rank) = member.get_member_rank() else {
                    return false;
                };

                return member_rank == GroupMemberRank::Administrator
                    || member_rank == GroupMemberRank::Owner;
            }
        }

        true
    }

    /// Mirrors `getRecentPages()`.
    pub fn get_recent_pages(&self) -> Vec<i32> {
        let mut page_list = Vec::new();

        let limit = GameConfiguration::get_instance().get_integer("discussions.replies.per.page");

        if self.reply_count > limit {
            for i in 0..3 {
                let new_number = self.get_reply_pages() - i;

                if new_number > 1 {
                    page_list.push(new_number);
                }
            }
        }

        page_list.sort_unstable();
        page_list
    }

    /// Mirrors `getTopicTitle()`.
    pub fn get_topic_title(&self) -> String {
        WordfilterManager::filter_sentence(&self.topic_title)
    }

    /// Mirrors `getReplyPages()`.
    pub fn get_reply_pages(&self) -> i32 {
        let limit = GameConfiguration::get_instance().get_integer("discussions.replies.per.page");

        if self.reply_count > 0 {
            (self.reply_count as f64 / limit as f64).ceil() as i32
        } else {
            1
        }
    }

    /// Mirrors `setTopicTitle(String)`.
    pub fn set_topic_title(&mut self, topic_title: &str) {
        self.topic_title = topic_title.to_string();
    }

    /// Mirrors `setOpen(boolean)`.
    pub fn set_open(&mut self, open: bool) {
        self.is_open = open;
    }

    /// Mirrors `setStickied(boolean)`.
    pub fn set_stickied(&mut self, stickied: bool) {
        self.is_stickied = stickied;
    }

    /// Mirrors `setViews(int)`.
    pub fn set_views(&mut self, views: i32) {
        self.views = views;
    }

    /// Mirrors `getCreatedDate(String)`.
    pub fn get_created_date(&self, date_format: &str) -> String {
        DateUtil::get_date(self.created_at, date_format)
            .replace("am", "AM")
            .replace("pm", "PM")
            .replace('.', "")
    }

    /// Mirrors `getLastMessage(String)`.
    pub fn get_last_message(&self, date_format: &str) -> String {
        DateUtil::get_date(self.last_message_at, date_format)
            .replace("am", "AM")
            .replace("pm", "PM")
            .replace('.', "")
    }
}
