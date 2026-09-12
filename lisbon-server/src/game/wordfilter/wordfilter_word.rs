//! Mirrors `net.h4bbo.lisbon.game.wordfilter.WordfilterWord`.

#[derive(Clone, Debug)]
pub struct WordfilterWord {
    word: String,
    is_bannable: bool,
    is_filterable: bool,
}

impl WordfilterWord {
    /// Mirrors the 3-arg `WordfilterWord(String, boolean, boolean)` constructor.
    pub fn new(word: String, is_bannable: bool, is_filterable: bool) -> Self {
        Self {
            word,
            is_bannable,
            is_filterable,
        }
    }

    /// Mirrors `getWord()`.
    pub fn get_word(&self) -> &str {
        &self.word
    }

    /// Mirrors `isBannable()`.
    pub fn is_bannable(&self) -> bool {
        self.is_bannable
    }

    /// Mirrors `isFilterable()`.
    pub fn is_filterable(&self) -> bool {
        self.is_filterable
    }
}
