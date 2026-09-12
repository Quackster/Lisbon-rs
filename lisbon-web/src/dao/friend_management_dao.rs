//! Mirrors `org.alexdev.http.dao.FriendManagementDao`.

use lisbon_server::dao::storage::{RowGetters, Storage};
use lisbon_server::game::messenger::messenger_user::MessengerUser;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct FriendManagementDao;

impl FriendManagementDao {
    /// Mirrors `getFriends(int, int, int)`.
    pub fn get_friends(user_id: i32, page: i32, items_per_page: i32) -> Vec<MessengerUser> {
        Self::query_friends(
            &format!(
                "SELECT id,username,figure,motto,last_online,sex,allow_stalking,is_online,category_id,online_status_visible FROM messenger_friends INNER JOIN users ON messenger_friends.from_id = users.id WHERE to_id = {user_id} ORDER BY UNIX_TIMESTAMP(last_online) DESC LIMIT {}, {}",
                (page - 1) * items_per_page,
                items_per_page
            ),
        )
    }

    /// Mirrors `getFriendsSearch(int, String, int, int)`.
    pub fn get_friends_search(user_id: i32, search_query: &str, page: i32, items_per_page: i32) -> Vec<MessengerUser> {
        Self::query_friends(
            &format!(
                "SELECT id,username,figure,motto,last_online,sex,allow_stalking,is_online,category_id,online_status_visible FROM messenger_friends INNER JOIN users ON messenger_friends.from_id = users.id WHERE to_id = {user_id} AND username LIKE '{}%' ORDER BY UNIX_TIMESTAMP(last_online) DESC LIMIT {}, {}",
                escape(search_query),
                (page - 1) * items_per_page,
                items_per_page
            ),
        )
    }

    /// Mirrors `getFriendsCount(int, String)`.
    pub fn get_friends_count_search(user_id: i32, search_query: &str) -> i32 {
        let mut count = 0;

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT COUNT(*) as friends_amount,username FROM messenger_friends INNER JOIN users ON messenger_friends.from_id = users.id WHERE to_id = {user_id} AND username LIKE '{}%'",
                escape(search_query)
            ),
        ) {
            if let Some(value) = row.i32("friends_amount") {
                count = value;
            }
        }

        count
    }

    /// Mirrors `getFriendsCount(int)`.
    pub fn get_friends_count(user_id: i32) -> i32 {
        let mut count = 0;

        for row in Storage::get_storage()
            .query_all(&format!("SELECT count(*) as friends_amount FROM messenger_friends WHERE to_id = {user_id}"))
        {
            if let Some(value) = row.i32("friends_amount") {
                count = value;
            }
        }

        count
    }

    /// Shared SELECT for the `messenger_friends` user list.
    fn query_friends(sql: &str) -> Vec<MessengerUser> {
        let mut friends = Vec::new();

        for row in Storage::get_storage().query_all(sql) {
            if let (Some(id), Some(username), Some(figure), Some(sex), Some(motto), Some(last_online), Some(allow_stalking), Some(category_id), Some(is_online), Some(online_status_visible)) = (
                row.i32("id"),
                row.str("username"),
                row.str("figure"),
                row.str("sex"),
                row.str("motto"),
                row.i64("last_online"),
                row.bool("allow_stalking"),
                row.i32("category_id"),
                row.bool("is_online"),
                row.bool("online_status_visible"),
            ) {
                friends.push(MessengerUser::new(
                    id,
                    &username,
                    &figure,
                    &sex,
                    &motto,
                    last_online,
                    allow_stalking,
                    category_id,
                    is_online,
                    online_status_visible,
                ));
            }
        }

        friends
    }
}
