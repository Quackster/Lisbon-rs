//! Mirrors `net.h4bbo.lisbon.dao.mysql.WordfilterDao`.

use crate::dao::storage::{RowGetters, Storage};
use crate::game::wordfilter::wordfilter_word::WordfilterWord;

pub struct WordfilterDao;

impl WordfilterDao {
    /// Mirrors `getBadWords()`.
    pub fn get_bad_words() -> Vec<WordfilterWord> {
        let mut words = Vec::new();

        for row in Storage::get_storage().query_all("SELECT * FROM wordfilter") {
            if let (Some(word), Some(is_bannable), Some(is_filterable)) = (
                row.str("word"),
                row.bool("is_bannable"),
                row.bool("is_filterable"),
            ) {
                words.push(WordfilterWord::new(word, is_bannable, is_filterable));
            }
        }

        words
    }
}
