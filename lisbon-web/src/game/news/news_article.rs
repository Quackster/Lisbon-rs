//! Mirrors `org.alexdev.http.game.news.NewsArticle`.

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::date_util::DateUtil;

use crate::game::news::news_category::NewsCategory;
use crate::game::news::news_manager::NewsManager;
use crate::util::housekeeping_util::HousekeepingUtil;

#[derive(Clone, Debug, serde::Serialize)]
pub struct NewsArticle {
    pub id: i32,
    pub title: String,
    pub author_id: i32,
    pub author_override: String,
    pub short_story: String,
    pub full_story: String,
    pub timestamp: i64,
    pub topstory: String,
    pub topstory_override: String,
    pub article_image: String,
    pub categories: Vec<NewsCategory>,
    pub is_published: bool,
    pub views: i32,
    pub is_future_published: bool,
}

impl NewsArticle {
    /// Mirrors the `NewsArticle(int, String, int, String, String, String, long, String, String, String, String, boolean, int, boolean)` constructor
    /// (the `categories` CSV is parsed through `NewsManager`).
    pub fn new(
        id: i32,
        title: &str,
        author_id: i32,
        author_override: &str,
        shortstory: &str,
        fullstory: &str,
        timestamp: i64,
        topstory: &str,
        topstory_override: &str,
        article_image: &str,
        categories: &str,
        is_published: bool,
        views: i32,
        is_future_published: bool,
    ) -> Self {
        Self {
            id,
            title: title.to_string(),
            author_id,
            author_override: author_override.to_string(),
            short_story: shortstory.to_string(),
            full_story: fullstory.to_string(),
            timestamp,
            topstory: topstory.to_string(),
            topstory_override: topstory_override.to_string(),
            article_image: article_image.to_string(),
            categories: Self::parse_categories(categories),
            is_published,
            views,
            is_future_published,
        }
    }

    /// Mirrors `parseCategories(String)`.
    fn parse_categories(categories: &str) -> Vec<NewsCategory> {
        let mut category_list = Vec::new();

        if !categories.is_empty() {
            for category_data in categories.split(',') {
                if let Ok(id) = category_data.parse::<i32>() {
                    if let Some(category) = NewsManager::get_instance().get_category_by_id(id) {
                        category_list.push(category);
                    }
                }
            }
        }

        category_list
    }

    /// Mirrors `hasCategory(int)`.
    pub fn has_category(&self, id: i32) -> bool {
        self.categories.iter().any(|category| category.id == id)
    }

    /// Mirrors `getAuthor()`.
    /// Port note: a missing `PlayerDao.get_name` row yields an empty string
    // instead of Java's `null`.
    pub fn get_author(&self) -> String {
        if !self.author_override.is_empty() {
            return self.author_override.clone();
        }

        PlayerDao::get_name(self.author_id).unwrap_or_default()
    }

    /// Mirrors `getUrl()`.
    pub fn get_url(&self) -> String {
        if self.id == 0 {
            return "0-no-news".to_string();
        }

        let mut new_title = self.title.clone();
        for unwanted in [
            '!', '\'', '@', '&', '*', '%', '[', ']', '#', '=', '"', ':', '>', '<', ',', '.',
            '+', '-', '_', '/', '?', '\\',
        ] {
            new_title = new_title.replace(unwanted, "");
        }
        new_title = new_title.replace(' ', "-");
        new_title = new_title.to_lowercase();

        format!("{}-{new_title}", self.id)
    }

    /// Mirrors `getDate()`.
    pub fn get_date(&self) -> String {
        DateUtil::get_date(self.timestamp, "EEE dd MMM, yyyy").replace('.', "")
    }

    /// Mirrors `getLiveTopStory()`.
    pub fn get_live_top_story(&self) -> String {
        if !self.topstory_override.is_empty() {
            return self.topstory_override.clone();
        }

        format!(
            "{}/c_images/Top_Story_Images/{}",
            GameConfiguration::get_instance().get_string("static.content.path"),
            self.topstory
        )
    }

    /// Mirrors `getEscapedStory()`.
    pub fn get_escaped_story(&self) -> String {
        HousekeepingUtil.format_news_story(&self.full_story)
    }

    /// Mirrors `setTitle(String)`.
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    /// Mirrors `setShortStory(String)`.
    pub fn set_short_story(&mut self, shortstory: &str) {
        self.short_story = shortstory.to_string();
    }

    /// Mirrors `setTimestamp(long)`.
    pub fn set_timestamp(&mut self, timestamp: i64) {
        self.timestamp = timestamp;
    }

    /// Mirrors `setTopStory(String)`.
    pub fn set_top_story(&mut self, topstory: &str) {
        self.topstory = topstory.to_string();
    }

    /// Mirrors `setFullStory(String)`.
    pub fn set_full_story(&mut self, fullstory: &str) {
        self.full_story = fullstory.to_string();
    }

    /// Mirrors `setArticleImage(String)`.
    pub fn set_article_image(&mut self, article_image: &str) {
        self.article_image = article_image.to_string();
    }

    /// Mirrors `setPublished(boolean)`.
    pub fn set_published(&mut self, published: bool) {
        self.is_published = published;
    }

    /// Mirrors `setFuturePublished(boolean)`.
    pub fn set_future_published(&mut self, future_published: bool) {
        self.is_future_published = future_published;
    }

    /// Mirrors `setAuthorOverride(String)`.
    pub fn set_author_override(&mut self, author_override: &str) {
        self.author_override = author_override.to_string();
    }

    /// Mirrors `setTopstoryOverride(String)`.
    pub fn set_topstory_override(&mut self, topstory_override: &str) {
        self.topstory_override = topstory_override.to_string();
    }
}
