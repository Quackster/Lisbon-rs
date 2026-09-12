//! Mirrors `org.alexdev.http.dao.NewsDao`.

use std::collections::HashMap;

use chrono::Datelike;
use sqlx::mysql::MySqlRow;

use lisbon_server::dao::storage::{RowGetters, Storage};
use lisbon_server::util::config::server_configuration::ServerConfiguration;

fn escape(value: &str) -> String {
    value.replace('\'', "''")
}

pub use crate::game::news::news_article::NewsArticle;
pub use crate::game::news::news_category::NewsCategory;
pub use crate::game::news::news_date_key::NewsDateKey;

pub struct NewsDao;

impl NewsDao {
    /// Mirrors `getPastYear(boolean, int)`.
    // Port note: the Java `LinkedHashMap` order is preserved with a vec of
    // (month, articles) tuples instead of an unordered `HashMap`.
    pub fn get_past_year(include_unpublished: bool, filter_category_id: i32) -> Vec<(String, Vec<NewsArticle>)> {
        let mut monthly_news: Vec<(String, Vec<NewsArticle>)> = Vec::new();

        let mut query = String::from(
            "SELECT *, CONCAT(MONTHNAME(created_at), ' ', YEAR(created_at)) AS month_year, (SELECT GROUP_CONCAT(category_id SEPARATOR ',') FROM site_articles_categories WHERE article_id = site_articles.id GROUP BY article_id) AS categories FROM `site_articles` WHERE ",
        );

        if include_unpublished {
            query.push_str("(is_published = 0 OR is_published = 1) ");
        } else {
            query.push_str("is_published = 1 ");
        }

        if filter_category_id > 0 {
            query.push_str(&format!(
                "AND ((SELECT COUNT(*) FROM site_articles_categories WHERE category_id = {filter_category_id} AND article_id = id) > 0) "
            ));
        }

        query.push_str("ORDER BY created_at DESC");

        for row in Storage::get_storage().query_all(&query) {
            if let Some(month_year) = row.str("month_year") {
                if let Some(entry) = monthly_news.iter_mut().find(|(key, _)| *key == month_year) {
                    entry.1.push(Self::fill(&row));
                } else {
                    monthly_news.push((month_year, vec![Self::fill(&row)]));
                }
            }
        }

        monthly_news
    }

    /// Mirrors `getArchive(boolean)`.
    pub fn get_archive(include_unpublished: bool) -> Vec<(String, Vec<NewsArticle>)> {
        let mut categorised: Vec<(String, Vec<NewsArticle>)> = Vec::new();

        let mut query = String::from(
            "SELECT *, (SELECT label FROM article_categories WHERE article_categories.id = (SELECT category_id FROM site_articles_categories WHERE article_id = site_articles.id GROUP BY article_id LIMIT 1)) as category_name, (SELECT GROUP_CONCAT(category_id SEPARATOR ',') FROM site_articles_categories WHERE article_id = site_articles.id GROUP BY article_id) AS categories FROM `site_articles` WHERE ",
        );

        if include_unpublished {
            query.push_str("(is_published = 0 OR is_published = 1) ");
        } else {
            query.push_str("is_published = 1 ");
        }

        query.push_str("ORDER BY created_at DESC");

        for row in Storage::get_storage().query_all(&query) {
            if let Some(category_name) = row.str("category_name") {
                if let Some(entry) = categorised.iter_mut().find(|(key, _)| *key == category_name) {
                    entry.1.push(Self::fill(&row));
                } else {
                    categorised.push((category_name, vec![Self::fill(&row)]));
                }
            }
        }

        categorised
    }

    /// Mirrors `getTop(NewsDateKey, int, boolean, List<String>, int)`.
    pub fn get_top(
        date_key: NewsDateKey,
        limit: i32,
        include_unpublished: bool,
        exclude_news: &[String],
        filter_category_id: i32,
    ) -> Vec<NewsArticle> {
        let mut articles = Vec::new();

        let now = chrono::Local::now();
        let days_in_month = Self::days_in_month(now.month() as i32, now.year());
        let day_of_year = now.ordinal0() as i64 + 1;

        let mut query = String::from(
            "SELECT *, (SELECT GROUP_CONCAT(category_id SEPARATOR ',') FROM site_articles_categories WHERE article_id = site_articles.id GROUP BY article_id) AS categories FROM `site_articles` WHERE ",
        );

        if include_unpublished {
            query.push_str("(is_published = 0 OR is_published = 1) ");
        } else {
            query.push_str("is_published = 1 ");
        }

        match date_key {
            NewsDateKey::Today => {
                query.push_str(
                    "AND YEAR(created_at) = YEAR(NOW()) AND MONTH(created_at) = MONTH(NOW()) AND DAY(created_at) = DAY(NOW()) ",
                )
            }
            NewsDateKey::Yesterday => {
                query.push_str("AND DATE(created_at) = SUBDATE(CURRENT_DATE(), INTERVAL 1 DAY) ")
            }
            NewsDateKey::ThisWeek => {
                query.push_str("AND UNIX_TIMESTAMP() < (UNIX_TIMESTAMP(created_at) + 604800) ")
            }
            NewsDateKey::ThisMonth => {
                query.push_str(&format!(
                    "AND UNIX_TIMESTAMP() < (UNIX_TIMESTAMP(created_at) + {}) ",
                    days_in_month * 86400
                ))
            }
            NewsDateKey::PastYear => {
                query.push_str(&format!(
                    "AND UNIX_TIMESTAMP() < (UNIX_TIMESTAMP(created_at) + {}) ",
                    day_of_year * 86400
                ))
            }
            NewsDateKey::All => {}
        }

        if !exclude_news.is_empty() {
            query.push_str(&format!("AND id NOT IN ({}) ", exclude_news.join(",")));
        }

        if filter_category_id > 0 {
            query.push_str(&format!(
                "AND ((SELECT COUNT(*) FROM site_articles_categories WHERE category_id = {filter_category_id} AND article_id = id) > 0) "
            ));
        }

        query.push_str(&format!("ORDER BY created_at DESC LIMIT {limit}"));

        for row in Storage::get_storage().query_all(&query) {
            articles.push(Self::fill(&row));
        }

        articles
    }

    /// Mirrors `getTop(int, boolean, int)`.
    pub fn get_top_by_date(
        limit: i32,
        include_unpublished: bool,
        filter_category_id: i32,
    ) -> HashMap<NewsDateKey, Vec<NewsArticle>> {
        let mut article_map = HashMap::new();
        let mut exclusion_list: Vec<String> = Vec::new();

        let news_today =
            Self::get_top(NewsDateKey::Today, limit, include_unpublished, &exclusion_list, filter_category_id);
        exclusion_list.extend(news_today.iter().map(|article| article.id.to_string()));

        let news_yesterday =
            Self::get_top(NewsDateKey::Yesterday, limit, include_unpublished, &exclusion_list, filter_category_id);
        exclusion_list.extend(news_yesterday.iter().map(|article| article.id.to_string()));

        let news_this_week =
            Self::get_top(NewsDateKey::ThisWeek, limit, include_unpublished, &exclusion_list, filter_category_id);
        exclusion_list.extend(news_this_week.iter().map(|article| article.id.to_string()));

        let news_this_month =
            Self::get_top(NewsDateKey::ThisMonth, limit, include_unpublished, &exclusion_list, filter_category_id);
        exclusion_list.extend(news_this_month.iter().map(|article| article.id.to_string()));

        let news_past_year =
            Self::get_top(NewsDateKey::PastYear, limit, include_unpublished, &exclusion_list, filter_category_id);
        exclusion_list.extend(news_past_year.iter().map(|article| article.id.to_string()));

        article_map.insert(NewsDateKey::Today, news_today);
        article_map.insert(NewsDateKey::Yesterday, news_yesterday);
        article_map.insert(NewsDateKey::ThisWeek, news_this_week);
        article_map.insert(NewsDateKey::ThisMonth, news_this_month);
        article_map.insert(NewsDateKey::PastYear, news_past_year);

        article_map
    }

    /// Mirrors `getCategories()`.
    pub fn get_categories() -> HashMap<i32, NewsCategory> {
        let mut categories = HashMap::new();

        for row in Storage::get_storage().query_all("SELECT * FROM article_categories") {
            if let Some(id) = row.i32("id") {
                let label = row.str("label").unwrap_or_default();
                let index = row.str("category_index").unwrap_or_default();

                categories.insert(id, NewsCategory::new(id, &label, &index));
            }
        }

        categories
    }

    /// Mirrors `insertCategories(int, List<NewsCategory>)`.
    pub fn insert_categories(article_id: i32, news_categories: &[NewsCategory]) {
        Storage::get_storage().execute(
            &format!("DELETE FROM site_articles_categories WHERE article_id = {article_id}"),
        );

        for category in news_categories {
            Storage::get_storage().execute(&format!(
                "INSERT INTO site_articles_categories (article_id, category_id) VALUES ({article_id}, {cid})",
                cid = category.id
            ));
        }
    }

    /// Mirrors `create(String, String, String, String, String, int, String, String, String, long, boolean, boolean)`.
    pub fn create(
        title: &str,
        shortstory: &str,
        fullstory: &str,
        topstory: &str,
        topstory_override: &str,
        author_id: i32,
        author_override: &str,
        _category: &str,
        article_image: &str,
        publish_date: i64,
        is_future_published: bool,
        is_published: bool,
    ) -> i32 {
        Storage::get_storage()
            .execute_insert(&format!(
                "INSERT INTO `site_articles` (title, author_id, author_override, short_story, full_story, created_at, topstory, topstory_override, article_image, is_future_published, is_published) VALUES ('{t}', {author_id}, '{ao}', '{ss}', '{fs}', FROM_UNIXTIME({publish_date}), '{ts}', '{to}', '{img}', {fp}, {p})",
                t = escape(title),
                ao = escape(author_override),
                ss = escape(shortstory),
                fs = escape(fullstory),
                ts = escape(topstory),
                to = escape(topstory_override),
                img = escape(article_image),
                fp = is_future_published as i32,
                p = is_published as i32
            ))
            .map(|id| id as i32)
            .unwrap_or(0)
    }

    /// Mirrors `exists(int)`.
    pub fn exists(id: i32) -> bool {
        for _row in Storage::get_storage()
            .query_all(&format!("SELECT id FROM site_articles WHERE id = {id} LIMIT 1"))
        {
            return true;
        }

        false
    }

    /// Mirrors `get(int)`.
    pub fn get(id: i32) -> Option<NewsArticle> {
        for row in Storage::get_storage().query_all(
            &format!("SELECT *, (SELECT GROUP_CONCAT(category_id SEPARATOR ',') FROM site_articles_categories WHERE article_id = site_articles.id GROUP BY article_id) AS categories FROM site_articles WHERE id = {id} LIMIT 1"),
        ) {
            return Some(Self::fill(&row));
        }

        None
    }

    /// Mirrors `delete(int)`.
    pub fn delete(id: i32) {
        Storage::get_storage()
            .execute(&format!("DELETE FROM site_articles WHERE id = {id} LIMIT 1"));
    }

    /// Mirrors `publishFutureArticles()`.
    pub fn publish_future_articles() {
        Storage::get_storage().execute(
            "UPDATE site_articles SET is_published = 1 WHERE CURRENT_TIMESTAMP() > created_at AND is_published = 0 AND is_future_published = 1",
        );
    }

    /// Mirrors `save(NewsArticle)`.
    pub fn save(article: &NewsArticle) {
        Storage::get_storage().execute(&format!(
            "UPDATE site_articles SET title = '{t}', short_story = '{ss}', full_story = '{fs}', topstory = '{ts}', article_image = '{img}', is_published = {p}, created_at = FROM_UNIXTIME({created_at}), is_future_published = {fp}, author_override = '{ao}', topstory_override = '{to}' WHERE id = {id}",
            t = escape(&article.title),
            ss = escape(&article.short_story),
            fs = escape(&article.full_story),
            ts = escape(&article.topstory),
            img = escape(&article.article_image),
            p = article.is_published as i32,
            fp = article.is_future_published as i32,
            ao = escape(&article.author_override),
            to = escape(&article.topstory_override),
            created_at = article.timestamp,
            id = article.id
        ));
    }

    /// Mirrors `getTopStoryImages()`.
    // Port note: a missing directory yields an empty list instead of the
    // Java NPE (`Objects.requireNonNull`).
    pub fn get_top_story_images() -> Vec<String> {
        let mut images: Vec<String> = Vec::new();
        let site_directory = ServerConfiguration::get_string("site.directory");

        if let Ok(entries) = std::fs::read_dir(format!("{site_directory}/c_images/Top_Story_Images")) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.contains(".gif") {
                        images.push(name.to_string());
                    }
                }
            }
        }

        images.sort();
        images
    }

    /// Mirrors `fill(ResultSet)`.
    fn fill(row: &MySqlRow) -> NewsArticle {
        let title = row.str("title").unwrap_or_default();
        let author_override = row.str("author_override").unwrap_or_default();
        let short_story = row.str("short_story").unwrap_or_default();
        let full_story = row.str("full_story").unwrap_or_default();
        let topstory = row.str("topstory").unwrap_or_default();
        let topstory_override = row.str("topstory_override").unwrap_or_default();
        let article_image = row.str("article_image").unwrap_or_default();
        let categories = row.str("categories").unwrap_or_default();

        NewsArticle::new(
            row.i32("id").unwrap_or(0),
            &title,
            row.i32("author_id").unwrap_or(0),
            &author_override,
            &short_story,
            &full_story,
            row.i64("created_at").unwrap_or(0),
            &topstory,
            &topstory_override,
            &article_image,
            &categories,
            row.bool("is_published").unwrap_or(false),
            row.i32("views").unwrap_or(0),
            row.bool("is_future_published").unwrap_or(false),
        )
    }

    /// Mirrors `Calendar.getInstance().getActualMaximum(Calendar.DAY_OF_MONTH)`.
    fn days_in_month(month: i32, year: i32) -> i64 {
        let is_leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);

        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if is_leap => 29,
            _ => 28,
        }
    }
}
