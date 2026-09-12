//! Mirrors `net.h4bbo.lisbon.dao.mysql.TagDao`.

use rand::prelude::SliceRandom;

use crate::dao::storage::{RowGetters, Storage};
use crate::game::tags::habbo_tag::HabboTag;

pub struct TagDao;

impl TagDao {
    /// Mirrors `getTagInfoList(String)`.
    pub fn get_tag_info_list(tag: &str) -> Vec<HabboTag> {
        let mut search = Vec::new();

        for row in Storage::get_storage().query_all(&format!(
            "SELECT * FROM users_tags WHERE LOWER(tag) = '{}' AND (user_id > 0 OR group_id > 0) AND room_id = 0",
            tag.replace('\'', "''"),
        )) {
            if let Some(tag_value) = row.str("tag") {
                search.push(HabboTag::new(
                    tag_value,
                    row.i32("room_id").unwrap_or(0),
                    row.i32("user_id").unwrap_or(0),
                    row.i32("group_id").unwrap_or(0),
                ));
            }
        }

        search
    }

    /// Mirrors `countTag(String)`.
    pub fn count_tag(tag: &str) -> i32 {
        for row in Storage::get_storage().query_all(&format!(
            "SELECT COUNT(user_id) as tag_count FROM users_tags WHERE tag = '{}'",
            tag.replace('\'', "''"),
        )) {
            if let Some(count) = row.i32("tag_count") {
                return count;
            }
        }

        0
    }

    /// Mirrors `removeTags(int, int, int)`.
    pub fn remove_tags(user_id: i32, room_id: i32, group_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM users_tags WHERE user_id = {user_id} AND room_id = {room_id} AND group_id = {group_id}"
        ));
    }

    /// Mirrors `getUserTags(int)`.
    pub fn get_user_tags(user_id: i32) -> Vec<String> {
        let mut tags = Vec::new();

        for row in Storage::get_storage()
            .query_all(&format!("SELECT tag FROM users_tags WHERE user_id = {user_id}"))
        {
            if let Some(tag) = row.str("tag") {
                tags.push(tag);
            }
        }

        tags
    }

    /// Mirrors `getGroupTags(int)`.
    pub fn get_group_tags(group_id: i32) -> Vec<String> {
        let mut tags = Vec::new();

        for row in Storage::get_storage()
            .query_all(&format!("SELECT tag FROM users_tags WHERE group_id = {group_id}"))
        {
            if let Some(tag) = row.str("tag") {
                tags.push(tag);
            }
        }

        tags
    }

    /// Mirrors `removeTag(int, int, int, String)`.
    pub fn remove_tag(user_id: i32, room_id: i32, group_id: i32, tag: &str) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM users_tags WHERE user_id = {user_id} AND room_id = {room_id} AND group_id = {group_id} AND tag = '{}'",
            tag.replace('\'', "''"),
        ));
    }

    /// Mirrors `hasTag(int, int, int, String)`.
    pub fn has_tag(user_id: i32, room_id: i32, group_id: i32, tag: &str) -> bool {
        for _ in Storage::get_storage().query_all(&format!(
            "SELECT tag FROM users_tags WHERE user_id = {user_id} AND room_id = {room_id} AND group_id = {group_id} AND LOWER(tag) = LOWER('{}')",
            tag.replace('\'', "''"),
        )) {
            return true;
        }

        false
    }

    /// Mirrors `addTag(int, int, int, String)`.
    pub fn add_tag(user_id: i32, room_id: i32, group_id: i32, tag: &str) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO users_tags (user_id, room_id, group_id, tag) VALUES ({user_id}, {room_id}, {group_id}, '{}')",
            tag.replace('\'', "''"),
        ));
    }

    /// Mirrors `getPopularTags()` (a fixed limit of 20).
    pub fn get_popular_tags_default() -> Vec<(String, i32)> {
        Self::get_popular_tags(20)
    }

    /// Mirrors `getPopularTags(int)`.
    pub fn get_popular_tags(num: i32) -> Vec<(String, i32)> {
        let mut tag_list: Vec<(String, i32)> = Vec::new();

        let rows = Storage::get_storage().query_all(&format!(
            "SELECT tag, COUNT(*) AS quantity FROM users_tags GROUP BY tag ORDER BY quantity DESC LIMIT {}",
            num
        ));

        let mut total = 0;
        let mut temp: std::collections::HashMap<String, i32> = std::collections::HashMap::new();

        for row in rows {
            if let (Some(tag), Some(count)) = (row.str("tag"), row.i32("quantity")) {
                let count = temp
                    .get(&tag)
                    .map(|previous| previous + count)
                    .unwrap_or(count);
                total += count;
                temp.insert(tag, count);
            }
        }

        let entry_count = temp.len() as i32;
        let mut list: Vec<(String, i32)> = temp.into_iter().collect();
        list.sort_by(|a, b| a.1.cmp(&b.1));

        let fonts = [10, 12, 14, 20];

        if entry_count > 0 {
            let bits = (entry_count as f64 / 4.0).ceil() as i32;

            if total > 0 {
                let mut counter = entry_count;

                for entry in list {
                    let mut weight = 0;

                    if counter == bits {
                        weight = 3;
                    }

                    if counter == bits * 3 {
                        weight = 2;
                    }

                    if counter == bits * 2 {
                        weight = 1;
                    }

                    tag_list.push((entry.0, fonts[weight]));
                    counter -= 1;
                }
            }
        }

        tag_list.shuffle(&mut rand::thread_rng());
        tag_list
    }
}
