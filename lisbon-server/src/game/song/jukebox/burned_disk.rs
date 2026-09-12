//! Mirrors `net.h4bbo.lisbon.game.song.jukebox.BurnedDisk`.

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BurnedDisk {
    item_id: i64,
    soundmachine_id: i32,
    slot_id: i32,
    song_id: i32,
    burned: i64,
}

impl BurnedDisk {
    /// Mirrors the 5-arg `BurnedDisk(long, int, int, int, long)` constructor.
    pub fn new(
        item_id: i64,
        soundmachine_id: i32,
        slot_id: i32,
        song_id: i32,
        burned: i64,
    ) -> Self {
        Self {
            item_id,
            soundmachine_id,
            slot_id,
            song_id,
            burned,
        }
    }

    /// Mirrors `getItemId()`.
    pub fn get_item_id(&self) -> i64 {
        self.item_id
    }

    /// Mirrors `getSoundmachineId()`.
    pub fn get_soundmachine_id(&self) -> i32 {
        self.soundmachine_id
    }

    /// Mirrors `getSlotId()`.
    pub fn get_slot_id(&self) -> i32 {
        self.slot_id
    }

    /// Mirrors `getSongId()`.
    pub fn get_song_id(&self) -> i32 {
        self.song_id
    }

    /// Mirrors `getBurned()`.
    pub fn get_burned(&self) -> i64 {
        self.burned
    }
}
