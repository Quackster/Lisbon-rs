//! Mirrors `org.alexdev.http.controllers.site.CommunityController`.

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::util::date_util::DateUtil;


use crate::duckhttpd::TemplateValue;
use crate::duckhttpd::WebConnection;
use crate::duckhttpd::web_connection::SessionValue;
use crate::game::news::news_article::NewsArticle;
use crate::server::watchdog::{
    HIDDEN_RECOMMENDED_ROOMS, NEXT_RECENT_DISCUSSIONS, RECENT_DISCUSSIONS, RECOMMENDED_ROOMS,
    TAG_CLOUD_10, NEWS,
};
use crate::util::xss_util::XssUtil as XSSUtil;

/// Mirrors `community(WebConnection)`.
pub fn community(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);
    let mut template = web_connection.template("community");

    let article_list = NEWS.read().clone();
    let mut articles: Vec<Option<NewsArticle>> = vec![None; 5];
    for (index, article) in article_list.into_iter().enumerate().take(5) {
        articles[index] = Some(article);
    }

    for index in 0..5 {
        if articles[index].is_none() {
            articles[index] = Some(NewsArticle::new(
                0,
                "No news",
                0,
                "",
                "",
                "",
                DateUtil::get_current_time_seconds() as i64,
                "attention_topstory.png",
                "",
                "",
                "0",
                true,
                0,
                false,
            ));
        }

        template.set(
            &format!("article{}", index + 1),
            TemplateValue::of(articles[index].as_ref().unwrap()),
        );
    }

    web_connection.session().set("page", SessionValue::Str("community".to_string()));

    template.set(
        "recommendedRooms",
        TemplateValue::of(&*RECOMMENDED_ROOMS.read()),
    );
    template.set(
        "hiddenRecommendedRooms",
        TemplateValue::of(&*HIDDEN_RECOMMENDED_ROOMS.read()),
    );
    template.set(
        "randomHabbos",
        TemplateValue::of(&PlayerDao::get_random_habbos(18)),
    );
    template.set("tagCloud", TemplateValue::of(TAG_CLOUD_10.read().clone()));
    template.set(
        "recentTopics",
        TemplateValue::of(&*RECENT_DISCUSSIONS.read()),
    );
    template.set(
        "recentHiddenTopics",
        TemplateValue::of(&*NEXT_RECENT_DISCUSSIONS.read()),
    );

    template.render();

    Ok(())
}
