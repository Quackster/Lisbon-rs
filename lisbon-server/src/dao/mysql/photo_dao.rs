//! Mirrors `net.h4bbo.lisbon.dao.mysql.PhotoDao`.

use sqlx::Row;

use crate::dao::storage::{RowGetters, Storage};
use crate::game::item::photo::Photo;

fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);

    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }

    out
}

pub struct PhotoDao;

impl PhotoDao {
    /// Mirrors `addPhoto(long, int, long, byte[], int)`.
    pub fn add_photo(photo_id: i64, user_id: i32, timestamp: i64, photo: &[u8], checksum: i32) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO items_photos (photo_id, photo_user_id, timestamp, photo_data, photo_checksum) VALUES ({photo_id}, {user_id}, {timestamp}, X'{}', {checksum})",
            to_hex(photo)
        ));
    }

    /// Mirrors `getPhoto(int)`.
    pub fn get_photo(photo_id: i32) -> Option<Photo> {
        for row in Storage::get_storage()
            .query_all(&format!("SELECT * FROM items_photos WHERE photo_id = {photo_id}"))
        {
            let checksum = row.i32("photo_checksum").unwrap_or(0);
            let data: Vec<u8> = row.try_get("photo_data").unwrap_or_default();
            let time = row.i64("timestamp").unwrap_or(0);

            return Some(Photo::new(photo_id, checksum, data, time));
        }

        None
    }

    /// Mirrors `deleteItem(long)`.
    pub fn delete_item(photo_id: i64) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM items_photos WHERE photo_id = {photo_id}"
        ));
    }
}
