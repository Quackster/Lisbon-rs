//! Mirrors `net.h4bbo.lisbon.game.groups.Group`.

use crate::dao::mysql::group_dao::GroupDao;
use crate::dao::mysql::group_member_dao::GroupMemberDao;
use crate::game::groups::group_forum_type::GroupForumType;
use crate::game::groups::group_member::GroupMember;
use crate::game::groups::group_member_rank::GroupMemberRank;
use crate::game::groups::group_permission_type::GroupPermissionType;
use crate::game::player::player_rank::PlayerRank;
use crate::game::wordfilter::wordfilter_manager::WordfilterManager;
use crate::util::config::game_configuration::GameConfiguration;
use crate::util::date_util::DateUtil;

#[derive(Clone, Debug, serde::Serialize)]
pub struct Group {
    id: i32,
    name: String,
    description: String,
    owner_id: i32,
    room_id: i32,
    badge: String,
    recommended: bool,
    background: String,
    views: i32,
    topics: i32,
    group_type: i32,
    forum_type: GroupForumType,
    forum_permission: GroupPermissionType,
    alias: String,
    created_date: i64,
}

impl Group {
    /// Mirrors the 15-arg `Group` constructor (the Java `GroupForumType` /
    /// `GroupPermissionType` ids default to `PUBLIC` / `EVERYONE` when unknown,
    /// where Java would hold `null`).
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i32,
        name: &str,
        description: &str,
        owner_id: i32,
        room_id: i32,
        badge: &str,
        recommended: bool,
        background: &str,
        views: i32,
        topics: i32,
        group_type: i32,
        forum_type: i32,
        forum_permission: i32,
        alias: &str,
        created_date: i64,
    ) -> Self {
        Self {
            id,
            name: WordfilterManager::filter_sentence(name),
            description: WordfilterManager::filter_sentence(description),
            owner_id,
            room_id,
            badge: badge.to_string(),
            recommended,
            background: background.to_string(),
            views,
            topics,
            group_type,
            forum_type: GroupForumType::get_by_id(forum_type).unwrap_or(GroupForumType::Public),
            forum_permission: GroupPermissionType::get_by_id(forum_permission)
                .unwrap_or(GroupPermissionType::Everyone),
            alias: alias.to_string(),
            created_date,
        }
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `getName()`.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Mirrors `getOwnerId()`.
    pub fn get_owner_id(&self) -> i32 {
        self.owner_id
    }

    /// Mirrors `getBadge()` (Java strips non-alphanumeric characters).
    pub fn get_badge(&self) -> String {
        self.badge
            .chars()
            .filter(|character| character.is_ascii_alphanumeric())
            .collect()
    }

    /// Mirrors `getMember(int)`.
    pub fn get_member(&self, user_id: i32) -> Option<GroupMember> {
        if self.owner_id == user_id {
            return Some(GroupMember::new(self.owner_id, self.id, false, 3));
        }

        let member = GroupMemberDao::get_member(self.id, user_id)?;

        if member.is_pending() {
            return None;
        }

        Some(member)
    }

    /// Mirrors `isMember(int)`.
    pub fn is_member(&self, user_id: i32) -> bool {
        if self.owner_id == user_id {
            return true;
        }

        match GroupMemberDao::get_member(self.id, user_id) {
            Some(member) => !member.is_pending(),
            None => false,
        }
    }

    /// Mirrors `generateClickLink()`.
    pub fn generate_click_link(&self) -> String {
        let site_path = GameConfiguration::get_instance().get_string("site.path");

        if !self.alias.trim().is_empty() {
            format!("{site_path}/groups/{}", self.alias)
        } else {
            format!("{site_path}/groups/{}/id", self.id)
        }
    }

    /// Mirrors `hasTopicAdmin(PlayerRank)`.
    pub fn has_topic_admin(rank: PlayerRank) -> bool {
        rank.rank_id() >= 5
    }

    /// Mirrors `hasAdministrator(int)`.
    pub fn has_administrator(&self, user_id: i32) -> bool {
        let Some(member) = self.get_member(user_id) else {
            return false;
        };

        matches!(
            member.get_member_rank(),
            Some(GroupMemberRank::Administrator) | Some(GroupMemberRank::Owner)
        )
    }

    /// Mirrors `canViewForum(GroupMember)`.
    pub fn can_view_forum(&self, group_member: Option<&GroupMember>) -> bool {
        if self.forum_type == GroupForumType::Public {
            return true;
        }

        group_member.is_some()
    }

    /// Mirrors `canReplyForum(GroupMember)`.
    pub fn can_reply_forum(&self, group_member: Option<&GroupMember>) -> bool {
        if self.forum_type == GroupForumType::Public {
            return true;
        }

        group_member.is_some()
    }

    /// Mirrors `canForumPost(GroupMember)`.
    pub fn can_forum_post(&self, group_member: Option<&GroupMember>) -> bool {
        if self.forum_permission == GroupPermissionType::Everyone {
            return true;
        }

        if let Some(member) = group_member {
            if self.forum_permission == GroupPermissionType::AdminOnly {
                return matches!(
                    member.get_member_rank(),
                    Some(GroupMemberRank::Owner) | Some(GroupMemberRank::Administrator)
                );
            }

            if self.forum_permission == GroupPermissionType::MemberOnly {
                return true;
            }
        }

        false
    }

    /// Mirrors `getPendingMember(int)`.
    pub fn get_pending_member(&self, user_id: i32) -> Option<GroupMember> {
        if self.owner_id == user_id {
            return Some(GroupMember::new(self.owner_id, self.id, false, 3));
        }

        let member = GroupMemberDao::get_member(self.id, user_id)?;

        if member.is_pending() {
            Some(member)
        } else {
            None
        }
    }

    /// Mirrors `isPendingMember(int)`.
    pub fn is_pending_member(&self, user_id: i32) -> bool {
        if self.owner_id == user_id {
            return false;
        }

        match GroupMemberDao::get_member(self.id, user_id) {
            Some(member) => member.is_pending(),
            None => false,
        }
    }

    /// Mirrors `getCreatedDate()` (e.g. `May 5, 2019`).
    pub fn get_created_date(&self) -> String {
        DateUtil::get_date(self.created_date, "MMM dd, yyyy")
    }

    /// Mirrors `setName(String)`.
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Mirrors `getDescription()`.
    pub fn get_description(&self) -> &str {
        &self.description
    }

    /// Mirrors `setDescription(String)`.
    pub fn set_description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    /// Mirrors `getRoomId()`.
    pub fn get_room_id(&self) -> i32 {
        self.room_id
    }

    /// Mirrors `setRoomId(int)`.
    pub fn set_room_id(&mut self, room_id: i32) {
        self.room_id = room_id;
    }

    /// Mirrors `setBadge(String)`.
    pub fn set_badge(&mut self, badge: &str) {
        self.badge = badge.to_string();
    }

    /// Mirrors `isRecommended()`.
    pub fn is_recommended(&self) -> bool {
        self.recommended
    }

    /// Mirrors `setRecommended(boolean)`.
    pub fn set_recommended(&mut self, recommended: bool) {
        self.recommended = recommended;
    }

    /// Mirrors `getBackground()`.
    pub fn get_background(&self) -> &str {
        &self.background
    }

    /// Mirrors `setBackground(String)`.
    pub fn set_background(&mut self, background: &str) {
        self.background = background.to_string();
    }

    /// Mirrors `getViews()`.
    pub fn get_views(&self) -> i32 {
        self.views
    }

    /// Mirrors `setViews(int)`.
    pub fn set_views(&mut self, views: i32) {
        self.views = views;
    }

    /// Mirrors `getTopics()`.
    pub fn get_topics(&self) -> i32 {
        self.topics
    }

    /// Mirrors `setTopics(int)`.
    pub fn set_topics(&mut self, topics: i32) {
        self.topics = topics;
    }

    /// Mirrors `getGroupType()`.
    pub fn get_group_type(&self) -> i32 {
        self.group_type
    }

    /// Mirrors `setGroupType(int)`.
    pub fn set_group_type(&mut self, group_type: i32) {
        self.group_type = group_type;
    }

    /// Mirrors `getForumType()`.
    pub fn get_forum_type(&self) -> GroupForumType {
        self.forum_type
    }

    /// Mirrors `setForumType(GroupForumType)`.
    pub fn set_forum_type(&mut self, forum_type: GroupForumType) {
        self.forum_type = forum_type;
    }

    /// Mirrors `getForumPermission()`.
    pub fn get_forum_permission(&self) -> GroupPermissionType {
        self.forum_permission
    }

    /// Mirrors `setForumPermission(GroupPermissionType)`.
    pub fn set_forum_permission(&mut self, forum_permission: GroupPermissionType) {
        self.forum_permission = forum_permission;
    }

    /// Mirrors `getAlias()`.
    pub fn get_alias(&self) -> &str {
        &self.alias
    }

    /// Mirrors `setAlias(String)`.
    pub fn set_alias(&mut self, alias: &str) {
        self.alias = alias.to_string();
    }

    /// Mirrors `save()`.
    pub fn save(&self) {
        GroupDao::save_group(self);
    }

    /// Mirrors `saveBackground()`.
    pub fn save_background(&self) {
        GroupDao::save_background(self);
    }

    /// Mirrors `saveBadge()`.
    pub fn save_badge(&self) {
        GroupDao::save_badge(self);
    }

    /// Mirrors `getMemberCount(boolean)`.
    pub fn get_member_count(&self, is_pending: bool) -> i32 {
        GroupMemberDao::count_members(self.id, is_pending)
    }
}
