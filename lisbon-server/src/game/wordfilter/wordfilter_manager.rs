//! Mirrors `net.h4bbo.lisbon.game.wordfilter.WordfilterManager`.

use std::collections::HashMap;
use std::sync::Arc;

use lazy_static::lazy_static;
use parking_lot::RwLock;

use crate::dao::mysql::ban_dao::BanDao;
use crate::dao::mysql::wordfilter_dao::WordfilterDao;
use crate::game::ban::ban_manager::BanManager;
use crate::game::ban::ban_type::BanType;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::wordfilter::wordfilter_word::WordfilterWord;
use crate::util::config::game_configuration::GameConfiguration;
use crate::util::date_util::DateUtil;

lazy_static! {
    static ref INSTANCE: RwLock<Option<Arc<WordfilterManager>>> = RwLock::new(None);
}

#[derive(Clone)]
pub struct WordfilterManager {
    banned_words: Vec<WordfilterWord>,
}

impl WordfilterManager {
    fn new() -> Self {
        let mut banned_words = WordfilterDao::get_bad_words();
        banned_words
            .sort_by(|a, b| b.get_word().len().cmp(&a.get_word().len()));
        Self { banned_words }
    }

    /// Mirrors `hasBannableSentence(Player, String)`.
    pub fn has_bannable_sentence(player: &Player, sentence: &str) -> bool {
        let mins_since_joined =
            (DateUtil::get_current_time_seconds() as i64 - player.get_details().get_join_date()) / 60;

        if mins_since_joined > 30 {
            return false;
        }

        if GameConfiguration::get_instance().get_bool("wordfitler.enabled") {
            for filter_word in Self::get_instance().get_banned_words() {
                if !filter_word.is_bannable() {
                    continue;
                }

                if sentence
                    .to_lowercase()
                    .contains(&filter_word.get_word().to_lowercase())
                {
                    return true;
                }
            }
        }

        false
    }

    /// Mirrors `filterMandatorySentence(String)`.
    pub fn filter_mandatory_sentence(sentence: &str) -> String {
        if GameConfiguration::get_instance().get_bool("wordfitler.enabled") {
            let mut sentence = sentence.to_string();
            for filter_word in Self::get_instance().get_banned_words() {
                if filter_word.is_filterable() {
                    continue;
                }

                let word = filter_word.get_word().to_lowercase();
                if sentence.to_lowercase().contains(&word) {
                    sentence = sentence.to_lowercase().replace(
                        &word,
                        &GameConfiguration::get_instance().get_string("wordfilter.word.replacement"),
                    );
                }
            }
            return sentence;
        }

        sentence.to_string()
    }

    /// Mirrors `filterSentence(String)`.
    pub fn filter_sentence(sentence: &str) -> String {
        if GameConfiguration::get_instance().get_bool("wordfitler.enabled") {
            let mut sentence = sentence.to_string();
            for filter_word in Self::get_instance().get_banned_words() {
                let word = filter_word.get_word().to_lowercase();
                if sentence.to_lowercase().contains(&word) {
                    sentence = sentence.to_lowercase().replace(
                        &word,
                        &GameConfiguration::get_instance().get_string("wordfilter.word.replacement"),
                    );
                }
            }
            return sentence;
        }

        sentence.to_string()
    }

    /// Mirrors `performBan(Player)`.
    pub fn perform_ban(player: &Player) {
        let user_id = player.get_details().get_id();
        let in_20_years =
            DateUtil::get_current_time_seconds() as i64 + (365i64 * 24 * 60 * 60) * 20;

        BanDao::add_ban(
            BanType::UserId,
            &user_id.to_string(),
            in_20_years,
            "Banned for breaking the Habbo Way",
            -1,
        );

        let mut criteria: HashMap<BanType, String> = HashMap::new();
        criteria.insert(BanType::UserId, user_id.to_string());
        BanManager::get_instance().disconnect_ban_accounts(&criteria);
    }

    /// Mirrors `getBannedWords()`.
    pub fn get_banned_words(&self) -> Vec<WordfilterWord> {
        self.banned_words.clone()
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> Arc<WordfilterManager> {
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
