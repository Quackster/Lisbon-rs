//! Mirrors `net.h4bbo.lisbon.dao.mysql.JukeboxDao`.

use crate::dao::storage::{RowGetters, Storage};
use crate::game::song::jukebox::burned_disk::BurnedDisk;
use crate::util::date_util::DateUtil;

pub struct JukeboxDao;

impl JukeboxDao {
    /// Mirrors `addDisk(long, int, int)`.
    pub fn add_disk(item_id: i64, slot_id: i32, song_id: i32) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO soundmachine_disks (item_id, slot_id, song_id, burned_at) VALUES ({item_id}, {slot_id}, {song_id}, {})",
            DateUtil::get_current_time_seconds()
        ));
    }

    /// Mirrors `editDisk(int, int, int)`.
    pub fn edit_disk(item_id: i32, song_machine_id: i32, slot_id: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE soundmachine_disks SET soundmachine_id = {song_machine_id}, slot_id = {slot_id} WHERE item_id = {item_id}"
        ));
    }

    /// Mirrors `getDisks(long)`.
    pub fn get_disks(soundmachine_id: i64) -> Vec<BurnedDisk> {
        let mut disks = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!("SELECT * FROM soundmachine_disks WHERE soundmachine_id = {soundmachine_id}"),
        ) {
            if let (Some(item_id), Some(soundmachine_id), Some(slot_id), Some(song_id), Some(burned_at)) = (
                row.i64("item_id"),
                row.i32("soundmachine_id"),
                row.i32("slot_id"),
                row.i32("song_id"),
                row.i64("burned_at"),
            ) {
                disks.push(BurnedDisk::new(
                    item_id,
                    soundmachine_id,
                    slot_id,
                    song_id,
                    burned_at,
                ));
            }
        }

        disks
    }

    /// Mirrors `getDisk(long, int)`.
    pub fn get_disk(soundmachine_id: i64, song_id: i32) -> Option<BurnedDisk> {
        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT * FROM soundmachine_disks WHERE slot_id = {song_id} AND soundmachine_id = {soundmachine_id}"
            ),
        ) {
            if let (Some(item_id), Some(soundmachine_id), Some(slot_id), Some(song_id), Some(burned_at)) = (
                row.i64("item_id"),
                row.i32("soundmachine_id"),
                row.i32("slot_id"),
                row.i32("song_id"),
                row.i64("burned_at"),
            ) {
                return Some(BurnedDisk::new(
                    item_id,
                    soundmachine_id,
                    slot_id,
                    song_id,
                    burned_at,
                ));
            }
        }

        None
    }

    /// Mirrors `getSongIdByItem(long)`.
    pub fn get_song_id_by_item(item_id: i64) -> i32 {
        let mut song_id = -1;

        for row in Storage::get_storage().query_all(
            &format!("SELECT * FROM soundmachine_disks WHERE item_id = {item_id}"),
        ) {
            if let Some(id) = row.i32("song_id") {
                song_id = id;
            }
        }

        song_id
    }

    /// Mirrors `setBurned(int, boolean)`.
    pub fn set_burned(song_id: i32, burned_state: bool) {
        Storage::get_storage().execute(&format!(
            "UPDATE soundmachine_songs SET burnt = {} WHERE id = {song_id}",
            if burned_state { 1 } else { 0 }
        ));
    }
}
