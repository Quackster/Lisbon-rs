//! Mirrors `org.alexdev.http.game.stickers.StickerProduct`.

use crate::game::stickers::sticker_type::StickerType;

#[derive(Clone, Debug, serde::Serialize)]
pub struct StickerProduct {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub min_rank: i32,
    pub data: String,
    pub price: i32,
    pub amount: i32,
    pub category_id: i32,
    pub widget_type: i32,
    pub type_: i32,
}

impl StickerProduct {
    /// Mirrors the `StickerProduct(int, String, String, int, String, int, int, int, int, int)` constructor.
    pub fn new(
        id: i32,
        name: &str,
        description: &str,
        min_rank: i32,
        data: &str,
        price: i32,
        amount: i32,
        category_id: i32,
        widget_type: i32,
        type_: i32,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: description.to_string(),
            min_rank,
            data: data.to_string(),
            price,
            amount,
            category_id,
            widget_type,
            type_,
        }
    }

    /// Mirrors `getCssClass()`.
    pub fn get_css_class(&self) -> Option<String> {
        if self.type_ == 1 {
            return Some(format!("s_{}_pre", self.data));
        }

        if self.type_ == 4 {
            return Some(format!("b_{}_pre", self.data));
        }

        if self.type_ == 3 {
            return Some(format!("commodity_{}_pre", self.data));
        }

        if self.type_ == StickerType::GroupWidget.type_id()
            || self.type_ == StickerType::HomeWidget.type_id()
        {
            return Some(format!("w_{}_pre", self.data));
        }

        None
    }

    /// Mirrors `getType()`.
    pub fn get_type(&self) -> Option<StickerType> {
        StickerType::get_by_type(self.type_)
    }

    /// Mirrors `isProduct()`.
    pub fn is_product(&self) -> bool {
        self.widget_type == 0
    }

    /// Mirrors `isGroupWidget()`.
    pub fn is_group_widget(&self) -> bool {
        self.widget_type == -1
    }

    /// Mirrors `isHomeWidget()`.
    pub fn is_home_widget(&self) -> bool {
        self.widget_type == 1
    }
}
