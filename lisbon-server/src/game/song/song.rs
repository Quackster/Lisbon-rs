//! Mirrors `net.h4bbo.lisbon.game.song.Song`.

#[derive(Clone, Debug, Default)]
pub struct Song {
    id: i32,
    title: String,
    item_id: i64,
    user_id: i32,
    length: i32,
    data: String,
    slot_id: i32,
    is_burnt: bool,
}

impl Song {
    /// Mirrors the 7-arg `Song(int, String, long, int, int, String, boolean)` constructor.
    pub fn new(
        id: i32,
        title: String,
        item_id: i64,
        user_id: i32,
        length: i32,
        data: String,
        is_burnt: bool,
    ) -> Self {
        Self {
            id,
            title,
            item_id,
            user_id,
            length,
            data,
            slot_id: 0,
            is_burnt,
        }
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `setId(int)`.
    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    /// Mirrors `getTitle()`.
    pub fn get_title(&self) -> &str {
        &self.title
    }

    /// Mirrors `setTitle(String)`.
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Mirrors `getItemId()`.
    pub fn get_item_id(&self) -> i64 {
        self.item_id
    }

    /// Mirrors `setItemId(int)`.
    pub fn set_item_id(&mut self, item_id: i32) {
        self.item_id = item_id as i64;
    }

    /// Mirrors `getUserId()`.
    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    /// Mirrors `setUserId(int)`.
    pub fn set_user_id(&mut self, user_id: i32) {
        self.user_id = user_id;
    }

    /// Mirrors `getLength()`.
    pub fn get_length(&self) -> i32 {
        self.length
    }

    /// Mirrors `setLength(int)`.
    pub fn set_length(&mut self, length: i32) {
        self.length = length;
    }

    /// Mirrors `getData()`.
    pub fn get_data(&self) -> &str {
        &self.data
    }

    /// Mirrors `setData(String)`.
    pub fn set_data(&mut self, data: String) {
        self.data = data;
    }

    /// Mirrors `isBurnt()`.
    pub fn is_burnt(&self) -> bool {
        self.is_burnt
    }

    /// Mirrors `setBurnt(boolean)`.
    pub fn set_burnt(&mut self, burnt: bool) {
        self.is_burnt = burnt;
    }

    /// Mirrors `getSlotId()`.
    pub fn get_slot_id(&self) -> i32 {
        self.slot_id
    }

    /// Mirrors `setSlotId(int)`.
    pub fn set_slot_id(&mut self, slot_id: i32) {
        self.slot_id = slot_id;
    }
}
