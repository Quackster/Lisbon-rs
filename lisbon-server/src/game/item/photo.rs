//! Mirrors `net.h4bbo.lisbon.game.item.Photo`.
#[derive(Clone, Debug)]
pub struct Photo {
    id: i32,
    checksum: i32,
    data: Vec<u8>,
    time: i64,
}

impl Photo {
    /// Mirrors the `Photo(int, int, byte[], long)` constructor.
    pub fn new(id: i32, checksum: i32, data: Vec<u8>, time: i64) -> Self {
        Self {
            id,
            checksum,
            data,
            time,
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

    /// Mirrors `getChecksum()`.
    pub fn get_checksum(&self) -> i32 {
        self.checksum
    }

    /// Mirrors `getData()`.
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    /// Mirrors `getTime()`.
    pub fn get_time(&self) -> i64 {
        self.time
    }
}
