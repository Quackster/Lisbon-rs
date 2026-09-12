//! Mirrors `net.h4bbo.lisbon.dao.mysql.MessengerDao`.

use std::collections::HashMap;

use crate::dao::storage::{RowGetters, Storage};
use crate::game::messenger::messenger_category::MessengerCategory;
use crate::game::messenger::messenger_message::MessengerMessage;
use crate::game::messenger::messenger_user::MessengerUser;
use crate::util::date_util::DateUtil;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct MessengerDao;

impl MessengerDao {
    /// Mirrors `getFriends(int)`.
    pub fn get_friends(user_id: i32) -> HashMap<i32, MessengerUser> {
        Self::query_users(
            &format!(
                "SELECT id,username,figure,motto,last_online,sex,allow_stalking,is_online,category_id,online_status_visible FROM messenger_friends INNER JOIN users ON messenger_friends.from_id = users.id WHERE to_id = {user_id}"
            ),
        )
    }

    /// Mirrors `getFriendsPage(int, int, int)`.
    pub fn get_friends_page(user_id: i32, range: i32, page_size: i32) -> HashMap<i32, MessengerUser> {
        let offset = range * page_size;
        let limit = (range * page_size) + page_size;

        Self::query_users(
            &format!(
                "SELECT id,username,figure,motto,last_online,sex,allow_stalking,is_online,category_id,online_status_visible FROM messenger_friends INNER JOIN users ON messenger_friends.from_id = users.id WHERE to_id = {user_id} LIMIT {offset}, {limit}"
            ),
        )
    }

    /// Mirrors `getFriendsCount(int)`.
    pub fn get_friends_count(user_id: i32) -> i32 {
        let mut count = 0;

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT COUNT(*) AS friend_count FROM messenger_friends INNER JOIN users ON messenger_friends.from_id = users.id WHERE to_id = {user_id}"
            ),
        ) {
            if let Some(value) = row.i32("friend_count") {
                count = value;
            }
        }

        count
    }

    /// Mirrors `getRequests(int)`.
    pub fn get_requests(user_id: i32) -> HashMap<i32, MessengerUser> {
        let mut users = HashMap::new();

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT from_id,username,figure,sex,motto,last_online,allow_stalking,is_online,online_status_visible FROM messenger_requests INNER JOIN users ON messenger_requests.from_id = users.id WHERE to_id = {user_id}"
            ),
        ) {
            if let (Some(from_id), Some(username), Some(figure), Some(sex), Some(motto), Some(last_online), Some(allow_stalking), Some(is_online), Some(online_status_visible)) = (
                row.i32("from_id"),
                row.str("username"),
                row.str("figure"),
                row.str("sex"),
                row.str("motto"),
                row.i64("last_online"),
                row.bool("allow_stalking"),
                row.bool("is_online"),
                row.bool("online_status_visible"),
            ) {
                users.insert(
                    from_id,
                    MessengerUser::new(
                        from_id,
                        &username,
                        &figure,
                        &sex,
                        &motto,
                        last_online,
                        allow_stalking,
                        0,
                        is_online,
                        online_status_visible,
                    ),
                );
            }
        }

        users
    }

    /// Mirrors `search(String)`.
    pub fn search(query: &str) -> Vec<i32> {
        let mut user_list = Vec::new();

        for row in Storage::get_storage()
            .query_all(&format!("SELECT id FROM users WHERE LOWER(username) LIKE '{}%' LIMIT 30", escape(query)))
        {
            if let Some(id) = row.i32("id") {
                user_list.push(id);
            }
        }

        user_list
    }

    /// Mirrors `newRequest(int, int)`.
    pub fn new_request(from_id: i32, to_id: i32) {
        if to_id == from_id {
            return;
        }

        if Self::request_exists(from_id, to_id) {
            return;
        }

        Storage::get_storage().execute(&format!(
            "INSERT INTO messenger_requests (to_id, from_id) VALUES ({to_id}, {from_id})"
        ));
    }

    /// Mirrors `requestExists(int, int)`.
    pub fn request_exists(from_id: i32, to_id: i32) -> bool {
        for _row in Storage::get_storage()
            .query_all(
                &format!("SELECT * FROM messenger_requests WHERE from_id = {from_id} AND to_id = {to_id}"),
            )
        {
            return true;
        }

        false
    }

    /// Mirrors `friendExists(int, int)`.
    pub fn friend_exists(from_id: i32, to_id: i32) -> bool {
        for _row in Storage::get_storage()
            .query_all(
                &format!("SELECT * FROM messenger_friends WHERE from_id = {from_id} AND to_id = {to_id}"),
            )
        {
            return true;
        }

        false
    }

    /// Mirrors `removeRequest(int, int)`.
    pub fn remove_request(from_id: i32, to_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM messenger_requests WHERE from_id = {from_id} AND to_id = {to_id}"
        ));
        Storage::get_storage().execute(&format!(
            "DELETE FROM messenger_requests WHERE from_id = {to_id} AND to_id = {from_id}"
        ));
    }

    /// Mirrors `removeAllRequests(int)`.
    pub fn remove_all_requests(to_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM messenger_requests WHERE to_id = {to_id}"
        ));
    }

    /// Mirrors `removeFriend(int, int)`.
    pub fn remove_friend(to_id: i32, from_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM messenger_friends WHERE from_id = {from_id} AND to_id = {to_id}"
        ));
    }

    /// Mirrors `newFriend(int, int)`.
    pub fn new_friend(to_id: i32, from_id: i32) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO messenger_friends (from_id, to_id) VALUES ({from_id}, {to_id})"
        ));
    }

    /// Mirrors `updateFriendCategory(int, int, int)`.
    pub fn update_friend_category(user_id: i32, friend_id: i32, category_id: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE messenger_friends SET category_id = {category_id} WHERE from_id = {friend_id} AND to_id = {user_id}"
        ));
    }

    /// Mirrors `resetFriendCategories(int, int)`.
    pub fn reset_friend_categories(user_id: i32, category_id: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE messenger_friends SET category_id = 0 WHERE to_id = {user_id} AND category_id = {category_id}"
        ));
    }

    /// Mirrors `newMessage(int, int, String)`.
    pub fn new_message(from_id: i32, to_id: i32, message: &str) -> i32 {
        let date = DateUtil::get_current_time_seconds() as i64;

        match Storage::get_storage().execute_insert(&format!(
            "INSERT INTO messenger_messages (receiver_id, sender_id, unread, body, date) VALUES ({to_id}, {from_id}, 1, '{}', {date})",
            escape(message)
        )) {
            Some(id) => id as i32,
            None => 0,
        }
    }

    /// Mirrors `getUnreadMessages(int)`.
    pub fn get_unread_messages(user_id: i32) -> HashMap<i32, MessengerMessage> {
        let mut messages = HashMap::new();

        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM messenger_messages WHERE receiver_id = {user_id} AND unread = 1"))
        {
            if let (Some(id), Some(receiver_id), Some(sender_id), Some(date), Some(body)) = (
                row.i32("id"),
                row.i32("receiver_id"),
                row.i32("sender_id"),
                row.i64("date"),
                row.str("body"),
            ) {
                messages.insert(
                    id,
                    MessengerMessage::new(id, receiver_id, sender_id, date, &body),
                );
            }
        }

        messages
    }

    /// Mirrors `markMessageRead(int)`.
    pub fn mark_message_read(message_id: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE messenger_messages SET unread = 0 WHERE id = {message_id}"
        ));
    }

    /// Mirrors `getCategories(int)`.
    pub fn get_categories(user_id: i32) -> Vec<MessengerCategory> {
        let mut categories = Vec::new();

        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM messenger_categories WHERE user_id = {user_id}"))
        {
            if let (Some(id), Some(user_id), Some(name)) = (
                row.i32("id"),
                row.i32("user_id"),
                row.str("name"),
            ) {
                categories.push(MessengerCategory::new(id, user_id, &name));
            }
        }

        categories
    }

    /// Mirrors `deleteCategory(int, int)`.
    pub fn delete_category(category_id: i32, user_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM messenger_categories WHERE id = {category_id} AND user_id = {user_id}"
        ));
    }

    /// Mirrors `addCategory(String, int)`.
    pub fn add_category(name: &str, user_id: i32) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO messenger_categories (user_id, name) VALUES ({user_id}, '{}')",
            escape(name)
        ));
    }

    /// Mirrors `updateCategory(String, int, int)`.
    pub fn update_category(name: &str, category_id: i32, user_id: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE messenger_categories SET name = '{}' WHERE id = {category_id} AND user_id = {user_id}",
            escape(name)
        ));
    }

    /// Shared SELECT for the `messenger_friends` user list.
    fn query_users(sql: &str) -> HashMap<i32, MessengerUser> {
        let mut friends = HashMap::new();

        for row in Storage::get_storage().query_all(sql) {
            if let (Some(result_user_id), Some(username), Some(figure), Some(motto), Some(sex), Some(last_online), Some(allow_stalking), Some(category_id), Some(is_online), Some(online_status_visible)) = (
                row.i32("id"),
                row.str("username"),
                row.str("figure"),
                row.str("motto"),
                row.str("sex"),
                row.i64("last_online"),
                row.bool("allow_stalking"),
                row.i32("category_id"),
                row.bool("is_online"),
                row.bool("online_status_visible"),
            ) {
                friends.insert(
                    result_user_id,
                    MessengerUser::new(
                        result_user_id,
                        &username,
                        &figure,
                        &sex,
                        &motto,
                        last_online,
                        allow_stalking,
                        category_id,
                        is_online,
                        online_status_visible,
                    ),
                );
            }
        }

        friends
    }
}
