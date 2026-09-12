//! Mirrors `org.alexdev.http.game.stickers.StickerCategory`.

#[derive(Clone, Debug, serde::Serialize)]
pub struct StickerCategory {
    pub id: i32,
    pub name: String,
    pub min_rank: i32,
    pub category_type: i32,
}

impl StickerCategory {
    /// Mirrors `BACKGROUND_CATEGORY_TYPE`.
    pub const BACKGROUND_CATEGORY_TYPE: i32 = 2;

    /// Mirrors `STICKER_BACKGROUND_TYPE`.
    pub const STICKER_BACKGROUND_TYPE: i32 = 1;

    /// Mirrors the `StickerCategory(int, String, int, int)` constructor.
    pub fn new(id: i32, name: &str, min_rank: i32, category_type: i32) -> Self {
        Self {
            id,
            name: name.to_string(),
            min_rank,
            category_type,
        }
    }
}
