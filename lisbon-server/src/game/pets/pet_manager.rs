//! Mirrors `net.h4bbo.lisbon.game.pets.PetManager`.
use std::sync::Arc;

use lazy_static::lazy_static;
use parking_lot::RwLock;
use rand::Rng;

use crate::game::pets::pet::Pet;
use crate::game::pets::pet_stat::PetStat;
use crate::game::pets::pet_type::PetType;

lazy_static! {
    static ref INSTANCE: RwLock<Option<Arc<PetManager>>> = RwLock::new(None);
}

pub struct PetManager;

impl PetManager {
    /// Mirrors `getInstance()`.
    pub fn get_instance() -> Arc<PetManager> {
        if let Some(existing) = INSTANCE.read().as_ref() {
            return existing.clone();
        }
        let instance = Arc::new(PetManager);
        INSTANCE.write().replace(instance.clone());
        instance
    }

    /// Mirrors `isValidName(String, String)`.
    pub fn is_valid_name(&self, owner_name: &str, name: &str) -> bool {
        if name.contains(' ') {
            return false;
        }

        if name.len() > 15 {
            return false;
        }

        if name.is_empty() {
            return false;
        }

        if owner_name.to_lowercase() == name.to_lowercase() {
            return false;
        }

        true
    }

    /// Mirrors `getRandomSpeech(String)`.
    pub fn get_random_speech(&self, pet_type: &str) -> String {
        const DOG_SPEECH: [&str; 5] = [
            "Woooff... wooofff...",
            "Woooooooooooooooffff... wooff",
            "Nomnomnomnom..",
            "Grrrr....",
            "Grrrr.. grrrrr..",
        ];
        const CAT_SPEECH: [&str; 4] = [
            "Prrrrrrr... prrrrrr",
            "Prrrrrrrrrrrrrrrr...... prrrr..",
            "Meowwwww... meowwww..",
            "Meowwww!",
        ];
        const CROC_SPEECH: [&str; 3] = [
            "Gnawwwwwwww... gnaw..",
            "Gnawwwwwwwwwwwwwwwwwwwww.... gnawwwwwwww.....",
            "Gnaw! Gnaw!",
        ];

        let speeches: &[&str] = match pet_type {
            "0" => &DOG_SPEECH,
            "1" => &CAT_SPEECH,
            "2" => &CROC_SPEECH,
            _ => return String::new(),
        };

        speeches[rand::thread_rng().gen_range(0..speeches.len())].to_string()
    }

    /// Mirrors `getType(Pet)`.
    pub fn get_type(&self, pet: &Pet) -> Option<PetType> {
        match pet.get_pet_details().get_type() {
            "0" => Some(PetType::Dog),
            "1" => Some(PetType::Cat),
            "2" => Some(PetType::Croc),
            _ => None,
        }
    }

    /// Mirrors `getPetStats(long, PetStat)`.
    pub fn get_pet_stats(&self, last_time: i64, stat: PetStat) -> i32 {
        let a = (crate::util::date_util::DateUtil::get_current_time_seconds() as i64 - last_time) / 3600;

        if a < 2 {
            return stat.get_attribute_type();
        }

        for x in 1..=stat.get_attribute_type() {
            if a > 2 * x as i64 {
                return x;
            }
        }

        stat.get_attribute_type()
    }
}
