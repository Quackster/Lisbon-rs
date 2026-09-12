//! Mirrors `net.h4bbo.lisbon.game.ads.AdManager`.

use std::collections::HashMap;
use std::sync::Arc;

use lazy_static::lazy_static;
use parking_lot::RwLock;
use rand::seq::IteratorRandom;

use crate::dao::mysql::advertisements_dao::AdvertisementsDao;
use crate::game::ads::advertisement::Advertisement;

lazy_static! {
    static ref INSTANCE: RwLock<Option<Arc<AdManager>>> = RwLock::new(None);
}

#[derive(Clone)]
pub struct AdManager {
    ads: HashMap<i32, Vec<Advertisement>>,
}

impl AdManager {
    fn new() -> Self {
        Self {
            ads: AdvertisementsDao::get_ads(),
        }
    }

    /// Mirrors `getRandomAd(int)`.
    pub fn get_random_ad(&self, room_id: i32) -> Option<Advertisement> {
        self.ads
            .get(&room_id)?
            .iter()
            .filter(|ad| ad.is_enabled() && !ad.is_loading_ad())
            .cloned()
            .choose(&mut rand::thread_rng())
    }

    /// Mirrors `getRandomLoadingAd()`.
    pub fn get_random_loading_ad(&self) -> Option<Advertisement> {
        self.ads
            .get(&-1)?
            .iter()
            .filter(|ad| ad.is_enabled() && ad.is_loading_ad())
            .cloned()
            .choose(&mut rand::thread_rng())
    }

    /// Mirrors `getAd(int)`.
    pub fn get_ad(&self, id: i32) -> Option<&Advertisement> {
        for room_ads in self.ads.values() {
            for advertisement in room_ads {
                if advertisement.get_id() == id {
                    return Some(advertisement);
                }
            }
        }

        None
    }

    /// Mirrors `getAds()`.
    pub fn get_ads(&self) -> Vec<Advertisement> {
        self.ads.values().flatten().cloned().collect()
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> Arc<AdManager> {
        if let Some(existing) = INSTANCE.read().as_ref() {
            return existing.clone();
        }
        let instance = Arc::new(Self::new());
        INSTANCE.write().replace(instance.clone());
        instance
    }

    /// Mirrors `reset()`.
    pub fn reset() {
        INSTANCE.write().take();
        Self::get_instance();
    }
}
