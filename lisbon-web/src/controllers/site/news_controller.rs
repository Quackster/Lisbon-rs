//! Mirrors `org.alexdev.http.controllers.site.NewsController`.

use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::util::date_util::DateUtil;

use crate::dao::news_dao::NewsDao;
use crate::duckhttpd::Template;
use crate::duckhttpd::TemplateValue;
use crate::duckhttpd::WebConnection;
use crate::duckhttpd::web_connection::SessionValue;
use crate::game::news::news_article::NewsArticle;
use crate::game::news::news_category::NewsCategory;
use crate::game::news::news_date_key::NewsDateKey;
use crate::game::news::news_manager::NewsManager;
use crate::game::news::news_view::NewsView;
use crate::util::xss_util::XssUtil as XSSUtil;

/// Mirrors `articles(WebConnection)`.
pub fn articles(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    web_connection.session().set("page", SessionValue::Str("community".to_string()));

    let mut template = web_connection.template("news_articles");
    template.set("newsPage", TemplateValue::of("news"));
    render_news(web_connection, &mut template, None);
    template.render();

    Ok(())
}

/// Mirrors `fansites(WebConnection)`.
pub fn fansites(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);
    web_connection.session().set("page", SessionValue::Str("community".to_string()));

    let mut template = web_connection.template("news_articles");
    template.set("newsPage", TemplateValue::of("fansites"));
    render_news(
        web_connection,
        &mut template,
        NewsManager::get_instance().get_category_by_label("fansites"),
    );
    template.render();

    Ok(())
}

/// Mirrors `events(WebConnection)`.
pub fn events(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    web_connection.session().set("page", SessionValue::Str("community".to_string()));

    let mut template = web_connection.template("news_articles");
    template.set("newsPage", TemplateValue::of("events"));
    render_news(
        web_connection,
        &mut template,
        NewsManager::get_instance().get_category_by_label("events"),
    );
    template.render();

    Ok(())
}

/// Mirrors `rendernews(WebConnection, Template, NewsCategory)`.
fn render_news(
    web_connection: &WebConnection,
    template: &mut impl Template,
    override_category: Option<NewsCategory>,
) {
    let mut news_article_id = 0;

    if let Some(first_match) = web_connection.get_matches().first() {
        news_article_id = first_match.parse().unwrap_or(0);
    }

    let mut filter_category_id = override_category.as_ref().map_or(0, |category| category.id);
    let mut override_category = override_category;
    let mut view = NewsView::Default;

    template.set("monthlyView", TemplateValue::of(false));
    template.set("archiveView", TemplateValue::of(false));

    template.set(
        "archives",
        TemplateValue::json(serde_json::Value::Array(Vec::new())),
    );
    template.set("months", TemplateValue::json(serde_json::Value::Array(Vec::new())));
    template.set("articlesToday", TemplateValue::json(serde_json::Value::Array(Vec::new())));
    template.set(
        "articlesYesterday",
        TemplateValue::json(serde_json::Value::Array(Vec::new())),
    );
    template.set(
        "articlesThisWeek",
        TemplateValue::json(serde_json::Value::Array(Vec::new())),
    );
    template.set(
        "articlesThisMonth",
        TemplateValue::json(serde_json::Value::Array(Vec::new())),
    );
    template.set(
        "articlesPastYear",
        TemplateValue::json(serde_json::Value::Array(Vec::new())),
    );

    let route_request = web_connection.get_route_request();

    if route_request.ends_with("archive") || route_request.ends_with("archive/") {
        template.set("urlSuffix", TemplateValue::of("/in/archive"));
        template.set("archiveView", TemplateValue::of(true));
        view = NewsView::Archive;
    } else if filter_category_id > 0 || route_request.contains("/category/") {
        if route_request.contains("/category/") {
            if let Some(first_match) = web_connection.get_matches().first() {
                override_category = NewsManager::get_instance().get_category_by_label(first_match);
            }
        }

        if let Some(category) = override_category.as_ref() {
            filter_category_id = category.id;
            view = NewsView::Months;
            template.set("monthlyView", TemplateValue::of(true));
        }
    } else {
        template.set("urlSuffix", TemplateValue::of(""));
    }

    let include_unpublished = template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
        .and_then(|details| details.get_rank())
        .map_or(false, |rank| rank.rank_id() > 1);

    let mut news_article: Option<NewsArticle> = None;

    if !NewsDao::exists(news_article_id) {
        let top_articles =
            NewsDao::get_top(NewsDateKey::All, 1, include_unpublished, &[], filter_category_id);

        if !top_articles.is_empty() {
            news_article = Some(top_articles[0].clone());
        }
    } else {
        news_article = NewsDao::get(news_article_id);
    }

    if !news_article
        .as_ref()
        .map_or(false, |article| article.is_published || include_unpublished)
    {
        news_article = Some(NewsArticle::new(
            1,
            "No news",
            -1,
            "Hotel Staff",
            "",
            "There is no news.",
            DateUtil::get_current_time_seconds() as i64,
            "",
            "",
            "",
            "",
            true,
            0,
            false,
        ));
    }

    template.set("currentArticle", TemplateValue::of(&news_article));

    if view == NewsView::Archive {
        let monthly_articles = NewsDao::get_archive(include_unpublished);
        template.set(
            "archives",
            TemplateValue::of(
                monthly_articles
                    .into_iter()
                    .collect::<std::collections::HashMap<String, Vec<NewsArticle>>>(),
            ),
        );
        template.set("archiveView", TemplateValue::of(true));
    }

    if view == NewsView::Months {
        let monthly_articles = NewsDao::get_past_year(include_unpublished, filter_category_id);
        template.set(
            "months",
            TemplateValue::of(
                monthly_articles
                    .into_iter()
                    .collect::<std::collections::HashMap<String, Vec<NewsArticle>>>(),
            ),
        );
        template.set("monthlyView", TemplateValue::of(true));
    }

    if view == NewsView::Default {
        let news_today =
            NewsDao::get_top(NewsDateKey::Today, i32::MAX, include_unpublished, &[], filter_category_id);
        let mut exclusion_list: Vec<String> =
            news_today.iter().map(|article| article.id.to_string()).collect();

        let news_yesterday = NewsDao::get_top(
            NewsDateKey::Yesterday,
            i32::MAX,
            include_unpublished,
            &exclusion_list,
            filter_category_id,
        );
        exclusion_list.extend(
            news_yesterday.iter().map(|article| article.id.to_string()),
        );

        let news_this_week = NewsDao::get_top(
            NewsDateKey::ThisWeek,
            i32::MAX,
            include_unpublished,
            &exclusion_list,
            filter_category_id,
        );
        exclusion_list.extend(
            news_this_week.iter().map(|article| article.id.to_string()),
        );

        let news_this_month = NewsDao::get_top(
            NewsDateKey::ThisMonth,
            i32::MAX,
            include_unpublished,
            &exclusion_list,
            filter_category_id,
        );
        exclusion_list.extend(
            news_this_month.iter().map(|article| article.id.to_string()),
        );

        let news_past_year = NewsDao::get_top(
            NewsDateKey::PastYear,
            i32::MAX,
            include_unpublished,
            &exclusion_list,
            filter_category_id,
        );

        template.set("articlesToday", TemplateValue::of(&news_today));
        template.set(
            "articlesYesterday",
            TemplateValue::of(&news_yesterday),
        );
        template.set(
            "articlesThisWeek",
            TemplateValue::of(&news_this_week),
        );
        template.set(
            "articlesThisMonth",
            TemplateValue::of(&news_this_month),
        );
        let past_year_list: Vec<NewsArticle> = if news_today.is_empty()
            && news_yesterday.is_empty()
            && news_this_week.is_empty()
            && news_this_month.is_empty()
        {
            news_past_year
        } else {
            Vec::new()
        };
        template.set(
            "articlesPastYear",
            TemplateValue::of(&past_year_list),
        );
    }
}
