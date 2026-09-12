//! Mirrors `net.h4bbo.lisbon.dao.mysql.RoomDao`.

use sqlx::mysql::MySqlRow;

use crate::dao::storage::{RowGetters, Storage};
use crate::game::moderation::chat_message::ChatMessage;
use crate::messages::outgoing::rooms::user::chat_message::ChatMessageType;
use crate::game::room::room::Room;
use crate::util::date_util::DateUtil;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct RoomDao;

impl RoomDao {
    /// Mirrors `getRoomsByUserId(int)`.
    pub fn get_rooms_by_user_id(user_id: i32) -> Vec<Room> {
        let mut rooms = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!("SELECT * FROM rooms LEFT JOIN users ON rooms.owner_id = users.id WHERE rooms.owner_id = {user_id}"),
        ) {
            rooms.push(Self::fill_room(&row));
        }

        rooms
    }

    /// Mirrors `getRoomById(int)`.
    pub fn get_room_by_id(room_id: i32) -> Option<Room> {
        for row in Storage::get_storage()
            .query_all(
                &format!(
                    "SELECT * FROM rooms LEFT JOIN users ON rooms.owner_id = users.id WHERE rooms.id = {room_id}"
                ),
            )
        {
            return Some(Self::fill_room(&row));
        }

        None
    }

    /// Mirrors `getRecommendedRooms(int, int)`.
    pub fn get_recommended_rooms(limit: i32, offset: i32) -> Vec<Room> {
        let mut rooms = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT * FROM rooms LEFT JOIN users ON rooms.owner_id = users.id WHERE owner_id > 0 AND accesstype = 0 ORDER BY visitors_now DESC, rating DESC LIMIT {limit} OFFSET {offset}"
            ),
        ) {
            rooms.push(Self::fill_room(&row));
        }

        rooms
    }

    /// Mirrors `querySearchRooms(String)`.
    pub fn query_search_rooms(search_query: &str) -> Vec<Room> {
        let mut rooms = Vec::new();

        for row in Storage::get_storage().query_all(&format!(
            "SELECT * FROM rooms INNER JOIN users ON rooms.owner_id = users.id WHERE LOWER(users.username) LIKE '%{}%' OR LOWER(rooms.name) LIKE '%{}%' LIMIT 30",
            escape(search_query),
            escape(search_query)
        )) {
            rooms.push(Self::fill_room(&row));
        }

        rooms
    }

    /// Mirrors `getHighestRatedRooms(int, int)`.
    pub fn get_highest_rated_rooms(limit: i32, offset: i32) -> Vec<Room> {
        let mut rooms = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT * FROM rooms LEFT JOIN users ON rooms.owner_id = users.id WHERE owner_id > 0 ORDER BY rating DESC LIMIT {limit} OFFSET {offset}"
            ),
        ) {
            rooms.push(Self::fill_room(&row));
        }

        rooms
    }

    /// Mirrors `searchRooms(String, int, int)`.
    pub fn search_rooms(search_query: &str, room_owner: i32, limit: i32) -> Vec<Room> {
        let mut rooms = Vec::new();

        if search_query.trim().is_empty() && room_owner == -1 {
            return rooms;
        }

        let where_clause = if room_owner > 0 {
            format!(
                " owner_id = {room_owner} AND LOWER(rooms.name) LIKE '%{}%'",
                escape(search_query)
            )
        } else {
            format!(
                " LOWER(users.username) LIKE '%{}%' OR LOWER(rooms.name) LIKE '%{}%'",
                escape(search_query),
                escape(search_query)
            )
        };

        for row in Storage::get_storage()
            .query_all(&format!(
                "SELECT * FROM rooms INNER JOIN users ON rooms.owner_id = users.id WHERE{where_clause} ORDER BY visitors_now DESC, rating DESC LIMIT {limit}"
            ))
        {
            rooms.push(Self::fill_room(&row));
        }

        rooms
    }

    /// Mirrors the Java `fill(RoomData, ResultSet)` helper.
    pub fn fill_room(row: &MySqlRow) -> Room {
        let mut room = Room::new();
        let owner_name = row.str("username").unwrap_or_default();
        // `RoomData::fill`'s `room` argument cannot be the room itself
        // (`data` is owned inside `room`, a double borrow), so a throwaway
        // room is passed and the walkway follow-redirect is applied to the
        // real room below.
        room.get_data_mut().fill(
            &mut Room::new(),
            row.i32("id").unwrap_or(0),
            row.i32("owner_id").unwrap_or(0),
            &owner_name,
            row.i32("category").unwrap_or(0),
            row.str("name").as_deref().unwrap_or(""),
            row.str("description").as_deref().unwrap_or(""),
            row.str("model").as_deref().unwrap_or(""),
            row.str("ccts").as_deref().unwrap_or(""),
            row.i32("wallpaper").unwrap_or(0),
            row.i32("floor").unwrap_or(0),
            row.str("landscape").as_deref().unwrap_or(""),
            row.bool("showname").unwrap_or(false),
            row.bool("superusers").unwrap_or(false),
            row.i32("accesstype").unwrap_or(0),
            row.str("password").as_deref(),
            row.i32("visitors_now").unwrap_or(0),
            row.i32("visitors_max").unwrap_or(0),
            row.i32("rating").unwrap_or(0),
            row.i32("group_id").unwrap_or(0),
            row.bool("is_hidden").unwrap_or(false),
        );
        room.get_data().apply_walkway_follow_redirect(&room);
        room
    }

    /// Mirrors `getRoomIdByModel`.
    pub fn get_room_id_by_model(model: &str) -> i32 {
        Storage::get_storage()
            .get_string(
                &format!("SELECT id FROM rooms WHERE model = '{}'", escape(model)),
                "id",
            )
            .and_then(|value| value.parse().ok())
            .unwrap_or(-1)
    }

    /// Mirrors `saveRating`.
    pub fn save_rating(room_id: i32, rating: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE rooms SET rating = {rating} WHERE id = {room_id}"
        ));
    }

    /// Mirrors `saveGroupId(int, int)` (the Java `SQLException` is swallowed).
    pub fn save_group_id(room_id: i32, group_id: i32) {
        Storage::get_storage()
            .execute(&format!("UPDATE rooms SET group_id = {room_id} WHERE id = {group_id}"));
    }

    /// Mirrors `delete(Room)`.
    pub fn delete(room: &Room) {
        Storage::get_storage()
            .execute(&format!("DELETE FROM rooms WHERE id = {}", room.get_id()));
    }

    /// Mirrors `saveChatLog(List<ChatMessage>)`.
    pub fn save_chat_log(chat_message_list: &[ChatMessage]) {
        let timestamp = DateUtil::get_current_time_seconds() as i64;

        for chat_message in chat_message_list {
            let chat_type = match chat_message.get_chat_message_type() {
                ChatMessageType::Chat => 0,
                ChatMessageType::Shout => 1,
                _ => 2,
            };

            Storage::get_storage().execute(&format!(
                "INSERT INTO room_chatlogs (user_id, room_id, timestamp, chat_type, message) VALUES ({}, {}, {timestamp}, {chat_type}, '{}')",
                chat_message.get_player_id(),
                chat_message.get_room_id(),
                escape(chat_message.get_message())
            ));
        }
    }

    /// Mirrors `saveVisitors(Room)`.
    pub fn save_visitors(room: &Room) {
        Storage::get_storage().execute(&format!(
            "UPDATE rooms SET visitors_now = {} WHERE id = {}",
            room.get_data().get_visitors_now(),
            room.get_id()
        ));
    }

    /// Mirrors `saveDecorations(Room)`.
    pub fn save_decorations(room: &Room) {
        let data = room.get_data();

        Storage::get_storage().execute(&format!(
            "UPDATE rooms SET wallpaper = {}, floor = {}, landscape = '{}' WHERE id = {}",
            data.get_wallpaper(),
            data.get_floor(),
            escape(data.get_landscape()),
            room.get_id()
        ));
    }

    /// Mirrors `save(Room)`.
    pub fn save(room: &Room) {
        let data = room.get_data();

        Storage::get_storage().execute(&format!(
            "UPDATE rooms SET category = {}, name = '{}', description = '{}', showname = {}, superusers = {}, accesstype = {}, password = '{}', visitors_max = {} WHERE id = {}",
            data.get_category_id(),
            escape(data.get_name()),
            escape(data.get_description()),
            if data.show_owner_name() { 1 } else { 0 },
            if data.allow_super_users() { 1 } else { 0 },
            data.get_access_type_id(),
            escape(data.get_password().unwrap_or("")),
            data.get_visitors_max(),
            room.get_id()
        ));
    }
}
