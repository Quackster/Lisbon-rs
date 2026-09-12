//! Mirrors `org.alexdev.http.util.HomeUtil`.

use rand::Rng;

/// Mirrors `org.alexdev.http.util.HomeUtil`.
pub struct HomeUtil;

impl HomeUtil {
    const ADVERTISEMENTS: [&str; 9] = [
        "habbo_banner_1.gif",
        "Habbohome_phold_160x600.gif",
        "Habbohome_phold_160x600.gif",
        "Habbohome_phold_160x600.gif",
        "bb2_placeholder.gif",
        "SciFi_spaceholder_160x600_001.gif",
        "HC2_placeh_160x600.gif",
        "battleball_reddevil_br.gif",
        "HC_Promo_160x600_GIF01.gif",
    ];

    const VALENTINES_IMAGES: [&str; 5] = [
        "valentines_1_chanaho.png",
        "valentines_2_ultra.png",
        "valentines_3_santi13.png",
        "valentines_4_santi13.png",
        "valentines_5_rasta.png",
    ];

    /// Mirrors `getRandomAd()`.
    pub fn get_random_ad() -> Option<String> {
        if !Self::ADVERTISEMENTS.is_empty() {
            let mut rng = rand::thread_rng();
            Some(Self::ADVERTISEMENTS[rng.gen_range(0..Self::ADVERTISEMENTS.len())].to_string())
        } else {
            None
        }
    }

    /// Mirrors `getRandomValentinesImage()`.
    pub fn get_random_valentines_image() -> Option<String> {
        if !Self::VALENTINES_IMAGES.is_empty() {
            let mut rng = rand::thread_rng();
            Some(
                Self::VALENTINES_IMAGES[rng.gen_range(0..Self::VALENTINES_IMAGES.len())].to_string(),
            )
        } else {
            None
        }
    }

    /// Mirrors `getStickerLimit(boolean)`.
    pub fn get_sticker_limit(has_club_subscription: bool) -> i32 {
        if has_club_subscription {
            350
        } else {
            200
        }
    }
}
