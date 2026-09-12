//! Mirrors `net.h4bbo.lisbon.dao.mysql.SongMachineDao`.

use std::collections::HashMap;

use crate::dao::storage::{RowGetters, Storage};
use crate::game::song::song::Song;
use crate::game::song::song_playlist::SongPlaylist;

pub struct SongMachineDao;

impl SongMachineDao {
    /// Mirrors `getSong(int)`.
    pub fn get_song(song_id: i32) -> Option<Song> {
        for row in Storage::get_storage().query_all(
            &format!("SELECT * FROM soundmachine_songs WHERE id = {song_id}"),
        ) {
            if let (Some(id), Some(title), Some(user_id), Some(length), Some(data), Some(burnt)) = (
                row.i32("id"),
                row.str("title"),
                row.i32("user_id"),
                row.i32("length"),
                row.str("data"),
                row.bool("burnt"),
            ) {
                return Some(Song::new(
                    id,
                    title,
                    song_id as i64,
                    user_id,
                    length,
                    data,
                    burnt,
                ));
            }
        }

        None
    }

    /// Mirrors `getSongUserList(int)`.
    pub fn get_song_user_list(user_id: i32) -> Vec<Song> {
        let mut songs = Vec::new();

        for row in Storage::get_storage().query_all(&format!(
            "SELECT * FROM soundmachine_songs WHERE user_id = {user_id}"
        )) {
            if let (
                Some(id),
                Some(title),
                Some(item_id),
                Some(song_user_id),
                Some(length),
                Some(data),
                Some(burnt),
            ) = (
                row.i32("id"),
                row.str("title"),
                row.i32("item_id"),
                row.i32("user_id"),
                row.i32("length"),
                row.str("data"),
                row.bool("burnt"),
            ) {
                songs.push(Song::new(
                    id,
                    title,
                    item_id as i64,
                    song_user_id,
                    length,
                    data,
                    burnt,
                ));
            }
        }

        songs
    }

    /// Mirrors `getSongList(int)`.
    pub fn get_song_list(item_id: i32) -> Vec<Song> {
        let mut songs = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!("SELECT * FROM soundmachine_songs WHERE item_id = {item_id}"),
        ) {
            if let (
                Some(id),
                Some(title),
                Some(user_id),
                Some(length),
                Some(data),
                Some(burnt),
            ) = (
                row.i32("id"),
                row.str("title"),
                row.i32("user_id"),
                row.i32("length"),
                row.str("data"),
                row.bool("burnt"),
            ) {
                songs.push(Song::new(
                    id,
                    title,
                    item_id as i64,
                    user_id,
                    length,
                    data,
                    burnt,
                ));
            }
        }

        songs
    }

    /// Mirrors `getSongPlaylist(int)`.
    pub fn get_song_playlist(item_id: i32) -> Vec<SongPlaylist> {
        let mut playlist = Vec::new();

        for row in Storage::get_storage().query_all(
            &format!("SELECT * FROM soundmachine_playlists WHERE item_id = {item_id}"),
        ) {
            if let (Some(song_id), Some(slot_id)) = (row.i32("song_id"), row.i32("slot_id")) {
                if let Some(song) = Self::get_song(song_id) {
                    playlist.push(SongPlaylist::new(item_id, song, slot_id));
                }
            }
        }

        playlist
    }

    /// Mirrors `addPlaylist(int, int, int)`.
    pub fn add_playlist(item_id: i32, song_id: i32, slot_id: i32) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO soundmachine_playlists (item_id, song_id, slot_id) VALUES ({item_id}, {song_id}, {slot_id})"
        ));
    }

    /// Mirrors `removePlaylistSong(int, long)`.
    pub fn remove_playlist_song(song_id: i32, item_id: i64) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM soundmachine_playlists WHERE song_id = {song_id} AND item_id = {item_id} LIMIT 1"
        ));
    }

    /// Mirrors `deleteSong(int)`.
    pub fn delete_song(song_id: i32) {
        Storage::get_storage()
            .execute(&format!("DELETE FROM soundmachine_songs WHERE id = {song_id}"));
    }

    /// Mirrors `clearSong(int)`.
    pub fn clear_song(song_id: i32) {
        Storage::get_storage().execute(&format!(
            "UPDATE soundmachine_songs SET item_id = -1 WHERE id = {song_id}"
        ));
    }

    /// Mirrors `clearPlaylist(int)`.
    pub fn clear_playlist(item_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM soundmachine_playlists WHERE item_id = {item_id}"
        ));
    }

    /// Mirrors `addSong(int, int, int, String, int, String)`.
    pub fn add_song(
        id: i32,
        user_id: i32,
        sound_machine_id: i32,
        title: &str,
        length: i32,
        data: &str,
    ) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO soundmachine_songs (id, user_id, item_id, title, length, data) VALUES ({id}, {user_id}, {sound_machine_id}, '{title}', {length}, '{data}')"
        ));
    }

    /// Mirrors the `addSong(int, int, String, int, String)` overload.
    pub fn add_song_new(
        user_id: i32,
        sound_machine_id: i32,
        title: &str,
        length: i32,
        data: &str,
    ) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO soundmachine_songs (user_id, item_id, title, length, data) VALUES ({user_id}, {sound_machine_id}, '{title}', {length}, '{data}')"
        ));
    }

    /// Mirrors `saveSong(int, String, int, String)`.
    pub fn save_song(song_id: i32, title: &str, length: i32, data: &str) {
        Storage::get_storage().execute(&format!(
            "UPDATE soundmachine_songs SET title = '{title}', length = {length}, data = '{data}' WHERE id = {song_id}"
        ));
    }

    /// Mirrors `addTrack(int, int, int)`.
    pub fn add_track(sound_machine_id: i32, track_id: i32, slot_id: i32) {
        Storage::get_storage().execute(&format!(
            "INSERT INTO soundmachine_tracks (soundmachine_id, track_id, slot_id) VALUES ({sound_machine_id}, {track_id}, {slot_id})"
        ));
    }

    /// Mirrors `removeTrack(int, int)`.
    pub fn remove_track(sound_machine_id: i32, slot_id: i32) {
        Storage::get_storage().execute(&format!(
            "DELETE FROM soundmachine_tracks WHERE soundmachine_id = {sound_machine_id} AND slot_id = {slot_id}"
        ));
    }

    /// Mirrors `getTracks(int)`.
    pub fn get_tracks(sound_machine_id: i32) -> HashMap<i32, i32> {
        let mut tracks = HashMap::new();

        for row in Storage::get_storage().query_all(
            &format!(
                "SELECT track_id, slot_id FROM soundmachine_tracks WHERE soundmachine_id = {sound_machine_id}"
            ),
        ) {
            if let (Some(slot_id), Some(track_id)) = (row.i32("slot_id"), row.i32("track_id")) {
                tracks.insert(slot_id, track_id);
            }
        }

        tracks
    }
}
