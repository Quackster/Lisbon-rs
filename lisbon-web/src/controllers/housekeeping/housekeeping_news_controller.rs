//! Mirrors `org.alexdev.http.controllers.housekeeping.HousekeepingNewsController`.

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::game::player::player_rank::PlayerRank;
use lisbon_server::util::date_util::DateUtil;

use crate::dao::news_dao::NewsDao;
use crate::duckhttpd::web_connection::SessionValue;
use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::game::housekeeping::housekeeping_manager::HousekeepingManager;
use crate::game::news::news_date_key::NewsDateKey;
use crate::game::news::news_manager::NewsManager;
use crate::game::news::news_article::NewsArticle;
use crate::routes::HOUSEKEEPING_PATH;
use crate::util::housekeeping_util::HousekeepingUtil;
use crate::util::session_util::SessionUtil;
use rand::Rng;
const MAX_NEWS_TO_DISPLAY: i32 = 250;

fn check_permission(
    web_connection: &WebConnection,
    permission: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean(SessionUtil::LOGGED_IN_HOUSKEEPING) {
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(false);
    }

    let user_id = web_connection.session().get_int("user.id");
    let Some(player_details) = PlayerDao::get_details(user_id) else {
        // Java NPEs on a null player.
        return Ok(false);
    };

    if !HousekeepingManager::get_instance().has_permission(
        player_details.get_rank().unwrap_or(PlayerRank::Rankless),
        permission,
    ) {
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(false);
    }

    Ok(true)
}

fn top_articles() -> Vec<NewsArticle> {
    NewsDao::get_top(NewsDateKey::All, MAX_NEWS_TO_DISPLAY, true, &[], 0)
}

/// Mirrors `articles(WebConnection)`.
pub fn articles(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !check_permission(web_connection, "articles/create")? {
        return Ok(());
    }

    let mut tpl = web_connection.template("housekeeping/articles");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );

    tpl.set("pageName", TemplateValue::of("View News"));
    tpl.set("articles", TemplateValue::of(top_articles()));
    tpl.render_html().ok();

    // Delete alert after it's been rendered
    web_connection.session().delete("alertMessage");
    Ok(())
}

/// Mirrors `create(WebConnection)`.
pub fn create(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !check_permission(web_connection, "articles/create")? {
        return Ok(());
    }

    let Some(session) = PlayerDao::get_details(web_connection.session().get_int("user.id"))
    else {
        // Java NPEs on a null player.
        return Ok(());
    };

    if web_connection.post().queries().len() > 0 {
        let date_published = web_connection
            .post()
            .get_string("datePublished")
            .unwrap_or_default();
        let publish_date = DateUtil::get_from_format("yyyy-MM-dd'T'HH:mm", &date_published);

        let mut categories: Vec<crate::game::news::news_category::NewsCategory> = Vec::new();

        for data in web_connection.post().get_array("categories[]") {
            if let Some(category) = NewsManager::get_instance().get_category_by_label(&data) {
                categories.push(category);
            }
        }

        let article_id = NewsDao::create(
            &web_connection.post().get_string("title").unwrap_or_default(),
            &web_connection.post().get_string("shortstory").unwrap_or_default(),
            &web_connection.post().get_string("fullstory").unwrap_or_default(),
            &web_connection.post().get_string("topstory").unwrap_or_default(),
            &web_connection.post().get_string("topstoryOverride").unwrap_or_default(),
            session.get_id(),
            &web_connection.post().get_string("authorOverride").unwrap_or_default(),
            &web_connection.post().get_string("category").unwrap_or_default(),
            &web_connection.post().get_string("articleimage").unwrap_or_default(),
            publish_date,
            web_connection
                .post()
                .get_string("futurePublished")
                .map(|value| value == "true")
                .unwrap_or(false),
            web_connection
                .post()
                .get_string("published")
                .map(|value| value == "true")
                .unwrap_or(false),
        );

        NewsDao::insert_categories(article_id, &categories);

        web_connection
            .session()
            .set("alertColour", SessionValue::Str("success".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str(
                "The submission of the news article was successful".into(),
            ),
        );
        web_connection.redirect(&format!("/{HOUSEKEEPING_PATH}/articles"));
        return Ok(());
    }

    let images = NewsDao::get_top_story_images();
    let random_image = if images.is_empty() {
        String::new()
    } else {
        images[rand::thread_rng().gen_range(0..images.len())].clone()
    };

    let mut tpl = web_connection.template("housekeeping/articles_create");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );
    tpl.set("pageName", TemplateValue::of("Create News"));
    tpl.set("images", TemplateValue::of(images));
    tpl.set("randomImage", TemplateValue::of(random_image));
    tpl.set(
        "currentDate",
        TemplateValue::of(DateUtil::get_date(
            DateUtil::get_current_time_seconds() as i64,
            "yyyy-MM-dd'T'HH:mm",
        )),
    );
    tpl.set(
        "categories",
        TemplateValue::of(NewsManager::get_instance().get_categories()),
    );
    tpl.render_html().ok();

    // Delete alert after it's been rendered
    web_connection.session().delete("alertMessage");
    Ok(())
}

/// Mirrors `delete(WebConnection)`.
pub fn delete(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean(SessionUtil::LOGGED_IN_HOUSKEEPING) {
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(());
    }

    let article_id = web_connection.get().get_int("id").unwrap_or(0);
    let article = NewsDao::get(article_id);

    let mut tpl = web_connection.template("housekeeping/articles");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );

    let user_id = web_connection.session().get_int("user.id");
    let Some(player_details) = PlayerDao::get_details(user_id) else {
        // Java NPEs on a null player.
        return Ok(());
    };

    if article.as_ref().map(|article| article.author_id).unwrap_or(0) != player_details.get_id()
    {
        if !HousekeepingManager::get_instance().has_permission(
            player_details.get_rank().unwrap_or(PlayerRank::Rankless),
            "articles/delete_any",
        ) {
            web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
            return Ok(());
        }
    }

    if !HousekeepingManager::get_instance().has_permission(
        player_details.get_rank().unwrap_or(PlayerRank::Rankless),
        "articles/delete_own",
    ) {
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(());
    }

    if !web_connection.get().contains("id") {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("danger".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str("There was no article selected to delete".into()),
        );
    } else if !NewsDao::exists(article_id) {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("danger".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str("The article does not exist".into()),
        );
    } else {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("success".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str("Successfully deleted the article".into()),
        );
        NewsDao::delete(article_id);
    }

    tpl.set("pageName", TemplateValue::of("Delete News"));
    tpl.set("articles", TemplateValue::of(top_articles()));
    tpl.render_html().ok();

    // Delete alert after it's been rendered
    web_connection.session().delete("alertMessage");
    Ok(())
}

/// Mirrors `edit(WebConnection)`.
pub fn edit(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean(SessionUtil::LOGGED_IN_HOUSKEEPING) {
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(());
    }

    let mut tpl = web_connection.template("housekeeping/articles_edit");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );

    let user_id = web_connection.session().get_int("user.id");
    let Some(player_details) = PlayerDao::get_details(user_id) else {
        // Java NPEs on a null player.
        return Ok(());
    };

    if !HousekeepingManager::get_instance().has_permission(
        player_details.get_rank().unwrap_or(PlayerRank::Rankless),
        "articles/edit_own",
    ) {
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(());
    }

    tpl.set("images", TemplateValue::of(NewsDao::get_top_story_images()));

    if !web_connection.get().contains("id") {
        web_connection
            .session()
            .set("alertColour", SessionValue::Str("danger".into()));
        web_connection.session().set(
            "alertMessage",
            SessionValue::Str("There was no article selected to edit".into()),
        );
    } else {
        let article_id = web_connection.get().get_int("id").unwrap_or(0);

        if !NewsDao::exists(article_id) {
            web_connection
                .session()
                .set("alertColour", SessionValue::Str("danger".into()));
            web_connection.session().set(
                "alertMessage",
                SessionValue::Str("The article does not exist".into()),
            );
        } else {
            let Some(mut article) = NewsDao::get(article_id) else {
                return Ok(());
            };

            if article.author_id != player_details.get_id() {
                if !HousekeepingManager::get_instance().has_permission(
                    player_details.get_rank().unwrap_or(PlayerRank::Rankless),
                    "articles/edit_any",
                ) {
                    web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
                    return Ok(());
                }
            }

            if web_connection.post().queries().len() > 0 {
                let date_published = web_connection
                    .post()
                    .get_string("datePublished")
                    .unwrap_or_default();
                let publish_date =
                    DateUtil::get_from_format("yyyy-MM-dd'T'HH:mm", &date_published);

                let mut categories: Vec<crate::game::news::news_category::NewsCategory> =
                    Vec::new();

                for data in web_connection.post().get_array("categories[]") {
                    if let Some(category) =
                        NewsManager::get_instance().get_category_by_label(&data)
                    {
                        categories.push(category);
                    }
                }

                NewsDao::insert_categories(article.id, &categories);

                article.set_title(&web_connection.post().get_string("title").unwrap_or_default());
                article.set_short_story(&web_connection.post().get_string("shortstory").unwrap_or_default());
                article.set_full_story(&web_connection.post().get_string("fullstory").unwrap_or_default());
                article.set_top_story(&web_connection.post().get_string("topstory").unwrap_or_default());
                article.set_topstory_override(&web_connection.post().get_string("topstoryOverride").unwrap_or_default());
                article.set_author_override(&web_connection.post().get_string("authorOverride").unwrap_or_default());
                article.set_article_image(&web_connection.post().get_string("articleimage").unwrap_or_default());
                article.set_published(
                    web_connection
                        .post()
                        .get_string("published")
                        .map(|value| value == "true")
                        .unwrap_or(false),
                );
                article.set_future_published(
                    web_connection
                        .post()
                        .get_string("futurePublished")
                        .map(|value| value == "true")
                        .unwrap_or(false),
                );
                article.set_timestamp(publish_date);

                // Mirrors `article.getCategories().clear(); addAll(categories);`.
                article.categories = categories;

                NewsDao::save(&article);

                web_connection
                    .session()
                    .set("alertColour", SessionValue::Str("success".into()));
                web_connection.session().set(
                    "alertMessage",
                    SessionValue::Str("The article was successfully saved".into()),
                );
            }

            tpl.set(
                "currentDate",
                TemplateValue::of(DateUtil::get_date(
                    article.timestamp,
                    "yyyy-MM-dd'T'HH:mm",
                )),
            );
            tpl.set("article", TemplateValue::of(article));
            tpl.set(
                "categories",
                TemplateValue::of(NewsManager::get_instance().get_categories()),
            );
        }
    }

    tpl.render_html().ok();

    // Delete alert after it's been rendered
    web_connection.session().delete("alertMessage");
    Ok(())
}

/// Mirrors `preview_news_article(WebConnection)`.
pub fn preview_news_article(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.post().contains("body") {
        web_connection.send_string("");
        return Ok(());
    }

    let util = HousekeepingUtil;
    let body = web_connection.post().get_string("body").unwrap_or_default();
    web_connection.send_string(&util.format_news_story(&body));
    Ok(())
}
