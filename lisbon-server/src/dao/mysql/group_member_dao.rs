//! Mirrors `net.h4bbo.lisbon.dao.mysql.GroupMemberDao`.

use std::collections::HashMap;

use crate::dao::mysql::player_dao::PlayerDao;
use crate::dao::storage::{RowGetters, Storage};
use crate::game::groups::group_member::GroupMember;
use crate::game::groups::group_member_rank::GroupMemberRank;
use crate::game::player::player_details::PlayerDetails;

pub struct GroupMemberDao;

impl GroupMemberDao {
    /// Mirrors `getMembers(int, boolean, String, int, int)`.
    pub fn get_members(
        group_id: i32,
        check_pending: bool,
        query: &str,
        page: i32,
        items_per_page: i32,
    ) -> Vec<GroupMember> {
        let offset = (page - 1) * items_per_page;
        let sql = if query.trim().is_empty() {
            format!(
                "SELECT * FROM groups_memberships INNER JOIN users ON groups_memberships.user_id = users.id WHERE group_id = {group_id} AND is_pending = {pending} LIMIT {offset}, {items_per_page}",
                pending = if check_pending { 1 } else { 0 }
            )
        } else {
            format!(
                "SELECT * FROM groups_memberships INNER JOIN users ON groups_memberships.user_id = users.id WHERE group_id = {group_id} AND is_pending = {pending} AND username LIKE '{}%' LIMIT {offset}, {items_per_page}",
                query.replace('\'', "''"),
                pending = if check_pending { 1 } else { 0 }
            )
        };

        let mut members = Vec::new();

        for row in Storage::get_storage().query_all(&sql) {
            members.push(Self::fill(&row));
        }

        members
    }

    /// Mirrors `getMember(int, int)`.
    pub fn get_member(group_id: i32, user_id: i32) -> Option<GroupMember> {
        for row in Storage::get_storage()
            .query_all(
                &format!("SELECT * FROM groups_memberships WHERE group_id = {group_id} AND user_id = {user_id}"),
            )
        {
            return Some(Self::fill(&row));
        }

        None
    }

    /// Mirrors `fill(ResultSet)`.
    fn fill(row: &sqlx::mysql::MySqlRow) -> GroupMember {
        let user_id = row.i32("user_id").unwrap_or(0);
        let group_id = row.i32("group_id").unwrap_or(0);
        let is_pending = row.bool("is_pending").unwrap_or(false);
        let member_rank = row
            .i32("member_rank")
            .or_else(|| row.str("member_rank").and_then(|value| value.parse().ok()))
            .unwrap_or(1);

        GroupMember::new(user_id, group_id, is_pending, member_rank)
    }

    /// Mirrors `addMember(int, int, boolean)`.
    pub fn add_member(user_id: i32, group_id: i32, insert_pending: bool) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO groups_memberships (user_id, group_id, is_pending) VALUES ({user_id}, {group_id}, {})",
            if insert_pending { 1 } else { 0 }
        ));
    }

    /// Mirrors `updateMember(int, int, GroupMemberRank, boolean)`.
    pub fn update_member(
        user_id: i32,
        group_id: i32,
        member_rank: GroupMemberRank,
        pending_status: bool,
    ) {
        Storage::get_storage().execute(&format!(
            "UPDATE groups_memberships SET is_pending = {}, member_rank = {} WHERE user_id = {user_id} AND group_id = {group_id}",
            if pending_status { 1 } else { 0 },
            member_rank.get_rank_id()
        ));
    }

    /// Mirrors `deleteMember(int, int)`.
    pub fn delete_member(user_id: i32, group_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM groups_memberships WHERE user_id = {user_id} AND group_id = {group_id}"
        ));
    }

    /// Mirrors `getPendingMembers(int)` (the Java `Pair<Integer,
    /// Map<String, String>>`).
    pub fn get_pending_members(user_id: i32) -> (i32, HashMap<String, String>) {
        let mut group_data: HashMap<String, String> = HashMap::new();
        let mut groups: HashMap<String, String> = HashMap::new();
        let mut pending_members = 0;

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT groups_details.id AS group_id, groups_details.name AS group_name FROM groups_memberships RIGHT JOIN groups_details ON groups_memberships.group_id = groups_details.id WHERE owner_id = {user_id} OR (groups_memberships.user_id = {user_id} AND (groups_memberships.member_rank = '2' OR groups_memberships.member_rank = '3'))"
            ),
        ) {
            if let (Some(group_id), Some(group_name)) = (row.i32("group_id"), row.str("group_name")) {
                group_data.insert(group_id.to_string(), group_name);
            }
        }

        if !group_data.is_empty() {
            let ids = group_data.keys().cloned().collect::<Vec<_>>().join(",");

            for row in Storage::get_storage().query_all(
                &format!(
                    "SELECT user_id, group_id FROM groups_memberships WHERE group_id IN ({ids}) AND is_pending = 1 GROUP BY group_id"
                ),
            ) {
                if let Some(group_id) = row.i32("group_id") {
                    let group_id_str = group_id.to_string();

                    if !groups.contains_key(&group_id_str) {
                        if let Some(name) = group_data.get(&group_id_str) {
                            groups.insert(group_id_str.clone(), name.clone());
                        }
                    }

                    pending_members += 1;
                }
            }
        }

        (pending_members, groups)
    }

    /// Mirrors `countMembers(int, boolean)`.
    pub fn count_members(group_id: i32, is_pending: bool) -> i32 {
        let mut count = 0;

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT COUNT(*) AS member_count FROM groups_memberships WHERE group_id = {group_id} AND is_pending = {}",
                if is_pending { 1 } else { 0 }
            ),
        ) {
            if let Some(value) = row.i32("member_count") {
                count = value;
            }
        }

        count
    }

    /// Mirrors `getOnlineMembersByFavourite(int)`.
    pub fn get_online_members_by_favourite(group_id: i32) -> Vec<PlayerDetails> {
        let mut details_list = Vec::new();

        for row in Storage::get_storage()
            .query_all(
                &format!(
                    "SELECT * FROM users WHERE favourite_group = {group_id} AND is_online = 1 LIMIT 1"
                ),
            )
        {
            let mut details = PlayerDetails::new();
            PlayerDao::fill(&mut details, &row);
            details_list.push(details);
            break;
        }

        details_list
    }

    /// Mirrors `resetFavourites(int)`.
    pub fn reset_favourites(group_id: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE users SET favourite_group = 0 WHERE favourite_group = {group_id}"
        ));
    }

    /// Mirrors `deleteMembers(int)`.
    pub fn delete_members(group_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM groups_memberships WHERE group_id = {group_id}"
        ));
    }
}
