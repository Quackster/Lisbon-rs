//! Mirrors `net.h4bbo.lisbon.dao.mysql.AdvertisementsDao`.

use std::collections::HashMap;

use sqlx::Connection;

use crate::dao::storage::{RowGetters, Storage};
use crate::game::ads::advertisement::Advertisement;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub struct AdvertisementsDao;

impl AdvertisementsDao {
    /// Mirrors `getAds()`.
    pub fn get_ads() -> HashMap<i32, Vec<Advertisement>> {
        let mut room_ads: HashMap<i32, Vec<Advertisement>> = HashMap::new();

        for row in Storage::get_storage().query_all("SELECT * FROM rooms_ads") {
            if let (Some(id), Some(is_loading_ad), Some(room_id), image, url, Some(enabled)) = (
                row.i32("id"),
                row.bool("is_loading_ad"),
                row.i32("room_id"),
                row.str("image"),
                row.str("url"),
                row.bool("enabled"),
            ) {
                room_ads
                    .entry(room_id)
                    .or_default()
                    .push(Advertisement::new(
                        id,
                        is_loading_ad,
                        room_id,
                        image.unwrap_or_default(),
                        url.unwrap_or_default(),
                        enabled,
                    ));
            }
        }

        room_ads
    }

    /// Mirrors `updateAds(Collection)` (the Java `SQLException` is swallowed by
    /// the Java callers; the Java `autocommit` off batch is mirrored with a
    /// transaction).
    pub fn update_ads(ads: &[Advertisement]) {
        if ads.is_empty() {
            return;
        }

        let storage = Storage::get_storage();
        let _ = storage.runtime().block_on(async {
            let mut conn = match storage.pool().acquire().await {
                Ok(conn) => conn,
                Err(e) => {
                    Storage::log_error(e.to_string());
                    return;
                }
            };

            let mut tx = match conn.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    Storage::log_error(e.to_string());
                    return;
                }
            };

            for ad in ads {
                let image = if ad.get_image().is_empty() {
                    "NULL".to_string()
                } else {
                    format!("'{}'", escape(ad.get_image()))
                };

                let url = if ad.get_url().is_empty() {
                    "NULL".to_string()
                } else {
                    format!("'{}'", escape(ad.get_url()))
                };

                if let Err(e) = sqlx::query(&format!(
                    "UPDATE rooms_ads SET room_id = {}, image = {image}, url = {url}, enabled = {}, is_loading_ad = {} WHERE id = {}",
                    ad.get_room_id(),
                    if ad.is_enabled() { 1 } else { 0 },
                    if ad.is_loading_ad() { 1 } else { 0 },
                    ad.get_id()
                ))
                .execute(&mut *tx)
                .await
                {
                    Storage::log_error(e.to_string());
                }
            }

            if let Err(e) = tx.commit().await {
                Storage::log_error(e.to_string());
            }
        });
    }

    /// Mirrors `deleteAd(int)`.
    pub fn delete_ad(id: i32) {
        Storage::get_storage()
            .execute(&format!("DELETE FROM rooms_ads WHERE id = {id}"));
    }

    /// Mirrors `create(int, String, String, boolean, boolean)`.
    pub fn create(room_id: i32, url: &str, image: &str, is_enabled: bool, is_room_loading_ad: bool) {
        let url = if url.is_empty() {
            "NULL".to_string()
        } else {
            format!("'{}'", escape(url))
        };

        let image = if image.is_empty() {
            "NULL".to_string()
        } else {
            format!("'{}'", escape(image))
        };

        Storage::get_storage().execute(&format!(
            "INSERT INTO rooms_ads (room_id, url, image, enabled, is_loading_ad) VALUES ({room_id}, {url}, {image}, {}, {})",
            if is_enabled { 1 } else { 0 },
            if is_room_loading_ad { 1 } else { 0 }
        ));
    }
}
