//! Mirrors `org.alexdev.http.game.stickers.StickerType`.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StickerType {
    Sticker,
    Background,
    Note,
    HomeWidget,
    GroupWidget,
}

impl StickerType {
    /// Const array mirroring `values()`.
    pub const ALL: [StickerType; 5] = [
        StickerType::Sticker,
        StickerType::Background,
        StickerType::Note,
        StickerType::HomeWidget,
        StickerType::GroupWidget,
    ];

    /// Mirrors `getByType(int)` (returns `None` on an unknown type; Java
    /// returns `null`).
    pub fn get_by_type(type_id: i32) -> Option<StickerType> {
        Self::ALL
            .iter()
            .copied()
            .find(|sticker_type| sticker_type.type_id() == type_id)
    }

    /// Mirrors `getTypeId()`.
    pub fn type_id(&self) -> i32 {
        match self {
            StickerType::Sticker => 1,
            StickerType::Background => 4,
            StickerType::Note => 3,
            StickerType::HomeWidget => 2,
            StickerType::GroupWidget => 5,
        }
    }
}
