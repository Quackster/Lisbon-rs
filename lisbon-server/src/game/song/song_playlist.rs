//! Mirrors `net.h4bbo.lisbon.game.song.SongPlaylist`.

use crate::game::song::song::Song;

#[derive(Clone, Debug)]
pub struct SongPlaylist {
    item_id: i32,
    song: Song,
    slot_id: i32,
}

impl SongPlaylist {
    /// Mirrors the 3-arg `SongPlaylist(int, Song, int)` constructor.
    pub fn new(item_id: i32, song: Song, slot_id: i32) -> Self {
        Self {
            item_id,
            song,
            slot_id,
        }
    }

    /// Mirrors `getItemId()`.
    pub fn get_item_id(&self) -> i32 {
        self.item_id
    }

    /// Mirrors `getSong()`.
    pub fn get_song(&self) -> &Song {
        &self.song
    }

    /// Mirrors `getSlotId()`.
    pub fn get_slot_id(&self) -> i32 {
        self.slot_id
    }
}
