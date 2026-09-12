//! Mirrors `net.h4bbo.lisbon.dao.mysql.GroupDao`.

use sqlx::mysql::MySqlRow;

use crate::dao::mysql::group_member_dao::GroupMemberDao;
use crate::dao::storage::{RowGetters, Storage};
use crate::game::groups::group::Group;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct GroupDao;

impl GroupDao {
    /// Mirrors `getJoinedGroups(int)`.
    pub fn get_joined_groups(user_id: i32) -> Vec<Group> {
        let mut group_list = Vec::new();

        for row in Storage::get_storage().query_all(&format!(
            "SELECT groups_details.* FROM groups_memberships RIGHT JOIN groups_details ON groups_memberships.group_id = groups_details.id WHERE owner_id = {user_id} OR (groups_memberships.user_id = {user_id} AND groups_memberships.is_pending = 0)"
        )) {
            let group_id = row.i32("id").unwrap_or(0);

            if !group_list.iter().any(|group: &Group| group.get_id() == group_id) {
                group_list.push(Self::fill(&row));
            }
        }

        group_list.sort_by(|a, b| {
            GroupMemberDao::count_members(b.get_id(), false)
                .cmp(&GroupMemberDao::count_members(a.get_id(), false))
        });

        group_list
    }

    /// Mirrors `getGroupOwner(int)`.
    pub fn get_group_owner(group_id: i32) -> i32 {
        let mut owner_id = 0;

        for row in Storage::get_storage()
            .query_all(&format!("SELECT owner_id FROM groups_details WHERE id = {group_id}"))
        {
            if let Some(value) = row.i32("owner_id") {
                owner_id = value;
            }
        }

        owner_id
    }

    /// Mirrors `querySearch(String)`.
    pub fn query_search(query: &str) -> Vec<Group> {
        let mut groups = Vec::new();

        for row in Storage::get_storage()
            .query_all(&format!(
                "SELECT * FROM groups_details WHERE name LIKE '%{}%' LIMIT 30",
                escape(query)
            ))
        {
            groups.push(Self::fill(&row));
        }

        groups
    }

    /// Mirrors `getGroup(int)`.
    pub fn get_group(group_id: i32) -> Option<Group> {
        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM groups_details WHERE id = {group_id}"))
        {
            return Some(Self::fill(&row));
        }

        None
    }

    /// Mirrors `getGroupByAlias(String)`.
    pub fn get_group_by_alias(group_alias: &str) -> Option<Group> {
        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM groups_details WHERE alias = '{}'", escape(group_alias)))
        {
            return Some(Self::fill(&row));
        }

        None
    }

    /// Mirrors `fill(ResultSet)`.
    fn fill(row: &MySqlRow) -> Group {
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
        )
    }

    /// Mirrors `addGroup(String, String, int)`.
    pub fn add_group(name: &str, description: &str, owner_id: i32) -> i32 {
        match Storage::get_storage().execute_insert(&format!(
            "INSERT INTO groups_details (name, description, owner_id) VALUES ('{}', '{}', {owner_id})",
            escape(name),
            escape(description)
        )) {
            Some(id) => id as i32,
            None => 0,
        }
    }

    /// Mirrors `saveGroup(Group)` (the Java `getGeneratedKeys()` on the
    /// `UPDATE` yields no row, so `0` is returned, mirroring Java).
    pub fn save_group(group: &Group) -> i32 {
        let alias = if group.get_alias().trim().is_empty() {
            "NULL".to_string()
        } else {
            format!("'{}'", escape(group.get_alias()))
        };

        Storage::get_storage().execute(&format!(
            "UPDATE groups_details SET name = '{}', description = '{}', room_id = {}, badge = '{}', recommended = {}, group_type = {}, forum_type = {}, forum_premission = {}, alias = {alias} WHERE id = {}",
            escape(group.get_name()),
            escape(group.get_description()),
            group.get_room_id(),
            escape(&group.get_badge()),
            if group.is_recommended() { 1 } else { 0 },
            group.get_group_type(),
            group.get_forum_type().get_id(),
            group.get_forum_permission().get_id(),
            group.get_id()
        ));

        0
    }

    /// Mirrors `saveBackground(Group)`.
    pub fn save_background(group: &Group) {
        Storage::get_storage().execute(&format!(
            "UPDATE groups_details SET background = '{}' WHERE id = {}",
            escape(group.get_background()),
            group.get_id()
        ));
    }

    /// Mirrors `saveBadge(Group)`.
    pub fn save_badge(group: &Group) {
        Storage::get_storage().execute(&format!(
            "UPDATE groups_details SET badge = '{}' WHERE id = {}",
            escape(&group.get_badge()),
            group.get_id()
        ));
    }

    /// Mirrors `delete(int)`.
    pub fn delete(group_id: i32) {
        Storage::get_storage()
            .execute(&format!("DELETE FROM groups_details WHERE id = {group_id}"));
    }

    /// Mirrors `hasGroupByAlias(String)`.
    pub fn has_group_by_alias(url: &str) -> bool {
        !Storage::get_storage()
            .query_all(&format!("SELECT * FROM groups_details WHERE alias = '{}'", escape(url)))
            .is_empty()
    }

    /// Mirrors `getGroupName(int)`.
    pub fn get_group_name(group_id: i32) -> Option<String> {
        Storage::get_storage()
            .get_string(&format!("SELECT name FROM groups_details WHERE id = {group_id}"), "name")
    }
}
