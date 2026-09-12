//! Mirrors `org.alexdev.http.dao.HousekeepingDao`.

use lisbon_server::dao::storage::{RowGetters, Storage};

pub struct HousekeepingDao;

impl HousekeepingDao {
    /// Mirrors `getUserCount()`.
    pub fn get_user_count() -> i32 {
        Self::count("SELECT COUNT(*) AS counted FROM users")
    }

    /// Mirrors `getRoomItemCount()`.
    pub fn get_room_item_count() -> i32 {
        Self::count("SELECT COUNT(*) AS counted FROM items WHERE room_id > 0")
    }

    /// Mirrors `getGroupCount()`.
    pub fn get_group_count() -> i32 {
        Self::count("SELECT COUNT(*) AS counted FROM groups_details")
    }

    /// Mirrors `getInventoryItemsCount()`.
    pub fn get_inventory_items_count() -> i32 {
        Self::count("SELECT COUNT(*) AS counted FROM items WHERE room_id = 0")
    }

    /// Mirrors `getPetCount()`.
    //+ Port note: the Java SQL has no `AS counted` alias, so the Java
    // `getInt("counted")` reads 0; the port mirrors that.
    pub fn get_pet_count() -> i32 {
        Self::count(
            "SELECT COUNT(*) FROM items_pets INNER JOIN items ON items_pets.item_id = items.id",
        )
    }

    /// Mirrors `getPhotoCount()`.
    //+ Port note: the Java SQL has no `AS counted` alias, so the Java
    // `getInt("counted")` reads 0; the port mirrors that.
    pub fn get_photo_count() -> i32 {
        Self::count(
            "SELECT COUNT(*) FROM items_photos INNER JOIN items ON items_photos.photo_id = items.id",
        )
    }

    fn count(sql: &str) -> i32 {
        let mut count = 0;

        for row in Storage::get_storage().query_all(sql) {
            if let Some(value) = row.i32("counted") {
                count = value;
            }
        }

        count
    }
}
