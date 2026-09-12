//! Mirrors `org.alexdev.http.controllers.site.TagController`.

use std::collections::HashMap;

use lisbon_server::dao::mysql::messenger_dao::MessengerDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::dao::mysql::tag_dao::TagDao;
use lisbon_server::game::tags::habbo_tag::HabboTag;
use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::game::wordfilter::wordfilter_manager::WordfilterManager;
use lisbon_server::server::rcon::messages::rcon_header::RconHeader;
use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::string_util::StringUtil;

use crate::duckhttpd::TemplateValue;
use crate::duckhttpd::WebConnection;
use crate::duckhttpd::web_connection::SessionValue;
use crate::server::watchdog::TAG_CLOUD_10;
use crate::util::html_util::HtmlUtil;
use crate::util::rcon_util::RconUtil;
use crate::util::tag_util::TagUtil;
use crate::util::xss_util::XssUtil as XSSUtil;

fn normalize_space(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Mirrors `tag(WebConnection)`.
pub fn tag(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    let mut page_id = web_connection.get().get_int("pageNumber").unwrap_or(0);

    let tags: Vec<HabboTag> = Vec::new();

    let paginated_users = StringUtil::paginate(&tags, 5);

    if !paginated_users.contains_key(&(page_id.saturating_sub(1) as usize)) {
        page_id = 1;
    }

    let mut template = web_connection.template("tag");

    template.set(
        "tagList",
        TemplateValue::of(
            paginated_users
                .get(&(page_id.saturating_sub(1) as usize))
                .unwrap_or(&Vec::new()),
        ),
    );
    template.set("pageId", TemplateValue::of(page_id));
    template.set("totalCount", TemplateValue::of(tags.len()));
    template.set(
        "tagCloud",
        TemplateValue::of(TAG_CLOUD_10.read().clone()),
    );
    template.set("tagSearchAdd", TemplateValue::of(""));
    template.set("showOlder", TemplateValue::of(false));
    template.set("showOldest", TemplateValue::of(false));
    template.set("showNewer", TemplateValue::of(false));
    template.set("showNewest", TemplateValue::of(false));
    template.set("showFirst", TemplateValue::of(false));
    template.set("showLast", TemplateValue::of(false));
    template.set("showFirstPage", TemplateValue::of(1));

    web_connection.session().set("page", SessionValue::Str("community".to_string()));
    template.render();

    Ok(())
}

/// Mirrors `search(WebConnection)`.
pub fn search(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let page_id = web_connection.get().get_int("pageNumber").unwrap_or(1);

    let tag = if web_connection.get().contains("tag") {
        web_connection.get().get_string("tag").unwrap_or_default()
    } else {
        web_connection.get_matches().first().cloned().unwrap_or_default()
    };

    respond_with_search(web_connection, &tag, page_id, "tag");

    Ok(())
}

/// Mirrors `tagsearch(WebConnection)`.
pub fn tagsearch(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    let tag = web_connection.post().get_string("tag").unwrap_or_default();

    respond_with_search(web_connection, &tag, 0, "base/tag_search");

    Ok(())
}

/// Mirrors `respondWithSearch(WebConnection, String, int, String)`.
fn respond_with_search(
    web_connection: &WebConnection,
    raw_tag: &str,
    mut page_id: i32,
    template_name: &str,
) {
    XSSUtil::clear(web_connection);

    let tag = urlencoding::decode(raw_tag).unwrap_or_default().into_owned();

    let tags: Vec<HabboTag> = if tag.trim().is_empty() {
        Vec::new()
    } else {
        TagDao::get_tag_info_list(&tag)
    };

    let paginated_users = StringUtil::paginate(&tags, 5);

    if !paginated_users.contains_key(&(page_id.saturating_sub(1) as usize)) {
        page_id = 1;
    }

    let tag = normalize_space(&HtmlUtil::remove_html_tags(&tag));

    let mut template = web_connection.template(template_name);

    template.set("tagSearchAdd", TemplateValue::of(""));

    if web_connection.session().get_boolean("authenticated") {
        let user_id = web_connection.session().get_int("user.id");

        let is_valid_tag = StringUtil::is_valid_tag(&tag, user_id, 0, 0).is_some();

        if is_valid_tag {
            template.set("tagSearchAdd", TemplateValue::of(tag.clone()));
        }
    }

    template.set("showOlder", TemplateValue::of(false));
    template.set("showOldest", TemplateValue::of(false));
    template.set("showNewer", TemplateValue::of(false));
    template.set("showNewest", TemplateValue::of(false));
    template.set("showFirst", TemplateValue::of(false));
    template.set("showFirstPage", TemplateValue::of(1));
    template.set("showLast", TemplateValue::of(false));

    let code_page = page_id - 1;

    if code_page >= 2 && paginated_users.contains_key(&((code_page - 2) as usize)) {
        template.set("showOlder", TemplateValue::of(true));
    }

    if code_page >= 3 && paginated_users.contains_key(&((code_page - 3) as usize)) {
        template.set("showOldest", TemplateValue::of(true));
    }

    if paginated_users.contains_key(&(code_page as usize + 1)) {
        template.set("showNewer", TemplateValue::of(true));
    }

    if paginated_users.contains_key(&(code_page as usize + 2)) {
        template.set("showNewest", TemplateValue::of(true));
    }

    if paginated_users.contains_key(&(code_page as usize + 3)) {
        template.set("showLast", TemplateValue::of(true));
        template.set("showLastPage", TemplateValue::of(paginated_users.len() as i32));
    }

    if code_page >= 4 && paginated_users.contains_key(&((code_page - 4) as usize)) {
        template.set("showFirst", TemplateValue::of(true));
        template.set("showFirstPage", TemplateValue::of(1));
    }

    template.set(
        "tagList",
        TemplateValue::of(
            paginated_users
                .get(&(page_id.saturating_sub(1) as usize))
                .unwrap_or(&Vec::new()),
        ),
    );

    template.set("totalTagUsers", TemplateValue::of(&paginated_users));
    template.set("tag", TemplateValue::of(tag.clone()));
    template.set("pageId", TemplateValue::of(page_id));
    template.set("totalCount", TemplateValue::of(tags.len()));
    template.set(
        "tagCloud",
        TemplateValue::of(TAG_CLOUD_10.read().clone()),
    );
    template.set("lastPage", TemplateValue::of(tags.len()));

    web_connection.session().set("page", SessionValue::Str("community".to_string()));
    template.render();
}

/// Mirrors `add(WebConnection)`.
pub fn add(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    let user_id = web_connection.session().get_int("user.id");

    if user_id < 1 {
        return Ok(());
    }

    let tag_list = TagDao::get_user_tags(user_id);

    if tag_list.len() >= GameConfiguration::get_instance().get_integer("max.tags.users") as usize {
        web_connection.send_string("taglimit");
        return Ok(());
    }

    let tag = StringUtil::is_valid_tag(
        web_connection.post().get_string("tagName").as_deref().unwrap_or_default(),
        user_id,
        0,
        0,
    );

    match tag {
        None => {
            web_connection.send_string("invalidtag");
            return Ok(());
        }
        Some(tag) => {
            if WordfilterManager::filter_sentence(&tag) == tag {
                StringUtil::add_tag(&tag, user_id, 0, 0);
            }

            web_connection.send_string("valid");

            RconUtil::send_command(
                RconHeader::RefreshTags,
                HashMap::from([("userId".to_string(), user_id.to_string())]),
            );
        }
    }

    Ok(())
}

/// Mirrors `remove(WebConnection)`.
pub fn remove(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    let user_id = web_connection.session().get_int("user.id");

    if user_id < 1 {
        web_connection.send_string("");
        return Ok(());
    }

    let mut template = web_connection.template("homes/widget/habblet/taglist");

    let player_details = match template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
    {
        Some(details) => details,
        None => {
            web_connection.send_string("");
            return Ok(());
        }
    };

    let tag = web_connection
        .post()
        .get_string("tagName")
        .unwrap_or_default();
    TagDao::remove_tag(user_id, 0, 0, &tag);

    let tags = TagDao::get_user_tags(user_id);

    template.set("tags", TemplateValue::of(tags));

    template.set("user", TemplateValue::of(&player_details));
    template.render();

    RconUtil::send_command(
        RconHeader::RefreshTags,
        HashMap::from([("userId".to_string(), user_id.to_string())]),
    );

    Ok(())
}

/// Mirrors `mytaglist(WebConnection)`.
pub fn mytaglist(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = web_connection.session().get_int("user.id");

    if user_id < 1 {
        return Ok(());
    }

    let mut template = web_connection.template("habblet/myTagList");
    template.set(
        "tags",
        TemplateValue::of(TagDao::get_user_tags(user_id)),
    );
    template.set(
        "tagRandomQuestion",
        TemplateValue::of(TagUtil::get_random_question()),
    );
    template.render();

    Ok(())
}

/// Mirrors `tagfight(WebConnection)`.
pub fn tagfight(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let first_tag = HtmlUtil::remove_html_tags(
        web_connection
            .post()
            .get_string("tag1")
            .as_deref()
            .unwrap_or_default(),
    );
    let second_tag = HtmlUtil::remove_html_tags(
        web_connection
            .post()
            .get_string("tag2")
            .as_deref()
            .unwrap_or_default(),
    );

    let first_count = TagDao::count_tag(&first_tag);
    let second_count = TagDao::count_tag(&second_tag);

    let mut image_number = 0;
    let mut result = "Tie!".to_string();

    if second_count > first_count {
        image_number = 1;
        result = "The winner is:".to_string();
    }

    if second_count < first_count {
        image_number = 2;
        result = "The winner is:".to_string();
    }

    let mut template = web_connection.template("habblet/tagFightResult");
    template.set("result", TemplateValue::of(result));
    template.set("resultTag1", TemplateValue::of(first_tag));
    template.set("resultTag2", TemplateValue::of(second_tag));
    template.set("resultHits1", TemplateValue::of(first_count));
    template.set("resultHits2", TemplateValue::of(second_count));
    template.set("tagFightImage", TemplateValue::of(image_number));
    template.render();

    Ok(())
}

/// Mirrors `tagmatch(WebConnection)`.
pub fn tagmatch(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut template = web_connection.template("habblet/tagMatch");

    let player_details = match template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
    {
        Some(details) => details,
        None => {
            web_connection.redirect("/");
            return Ok(());
        }
    };

    let friend_name = web_connection
        .post()
        .get_string("friendName")
        .unwrap_or_default();

    let mut error_message = String::new();

    let friend_id = PlayerDao::get_id(&friend_name);

    if !MessengerDao::friend_exists(player_details.get_id(), friend_id) {
        error_message = "Friend not found. Are you sure that they really exist?".to_string();
    }

    template.set("errorMsg", TemplateValue::of(error_message));
    template.render();

    Ok(())
}

/// Mirrors `remove_all_tags(WebConnection)`.
pub fn remove_all_tags(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = web_connection.session().get_int("user.id");

    if user_id < 1 {
        web_connection.send_string("Please login to remove all your tags.");
        return Ok(());
    }

    let my_tag_list = TagDao::get_user_tags(user_id);
    TagDao::remove_tags(user_id, 0, 0);

    web_connection.send_string(&format!(
        "All tags removed!<br><br>The tags removed: {}",
        my_tag_list.join(", ")
    ));

    Ok(())
}
