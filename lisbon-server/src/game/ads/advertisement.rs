//! Mirrors `net.h4bbo.lisbon.game.ads.Advertisement`.

#[derive(Clone, Debug, serde::Serialize)]
pub struct Advertisement {
    id: i32,
    room_id: i32,
    is_loading_ad: bool,
    image: String,
    url: String,
    enabled: bool,
}

impl Advertisement {
    /// Mirrors the 6-arg `Advertisement(int, boolean, int, String, String, boolean)` constructor.
    pub fn new(
        id: i32,
        is_loading_ad: bool,
        room_id: i32,
        image: String,
        url: String,
        enabled: bool,
    ) -> Self {
        Self {
            id,
            room_id,
            is_loading_ad,
            image,
            url,
            enabled,
        }
    }

    /// Mirrors `getId()`.
    pub fn get_id(&self) -> i32 {
        self.id
    }

    /// Mirrors `isLoadingAd()`.
    pub fn is_loading_ad(&self) -> bool {
        self.is_loading_ad
    }

    /// Mirrors `getRoomId()`.
    pub fn get_room_id(&self) -> i32 {
        self.room_id
    }

    /// Mirrors `getImage()`.
    pub fn get_image(&self) -> &str {
        &self.image
    }

    /// Mirrors `getUrl()`.
    pub fn get_url(&self) -> &str {
        &self.url
    }

    /// Mirrors `isEnabled()`.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
