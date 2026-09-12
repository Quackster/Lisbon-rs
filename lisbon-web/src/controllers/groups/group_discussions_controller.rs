//! Mirrors `org.alexdev.http.controllers.groups.GroupDiscussionsController`.

use lisbon_server::dao::mysql::group_dao::GroupDao;
use lisbon_server::game::groups::group_forum_type::GroupForumType;
use lisbon_server::game::groups::group::Group;
use lisbon_server::util::config::game_configuration::GameConfiguration;

use crate::dao::group_discussion_dao::GroupDiscussionDao;
use crate::duckhttpd::web_connection::WebConnection;
use crate::duckhttpd::{Settings, TemplateValue};
use http::StatusCode;
use crate::template::twig_template::TwigTemplate;
use crate::util::xss_util::XssUtil;

fn not_found(web_connection: &WebConnection) -> bool {
    let Some(response) = Settings::get_instance()
        .get_default_responses()
        .get_response(StatusCode::NOT_FOUND, web_connection)
    else {
        return false;
    };

    web_connection.send(response);
    true
}

fn is_numeric(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|character| character.is_ascii_digit())
}

/// Mirrors `viewDiscussions(WebConnection)`.
pub fn view_discussions(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    XssUtil::clear(web_connection);

    web_connection.session().set(
        "page",
        crate::duckhttpd::web_connection::SessionValue::Str("community".into()),
    );

    let matches = web_connection.get_matches();
    let Some(match0) = matches.first() else {
        return Ok(());
    };

    let group = if is_numeric(match0) && web_connection.get_route_request().ends_with("/id/discussions") {
        let Some(found) = GroupDao::get_group(match0.parse().unwrap_or(0)) else {
            not_found(web_connection);
            return Ok(());
        };

        if !found.get_alias().trim().is_empty() {
            web_connection
                .redirect(&format!("/groups/{}/discussions", found.get_alias()));
            return Ok(());
        }

        found
    } else if web_connection.get_route_request().ends_with("/discussions") {
        match GroupDao::get_group_by_alias(match0) {
            Some(group) => group,
            None => {
                not_found(web_connection);
                return Ok(());
            }
        }
    } else {
        not_found(web_connection);
        return Ok(());
    };

    let mut template = web_connection.template("groups/view_discussions");
    template.set("group", TemplateValue::of(&group));
    render(web_connection, &group, &mut template, 1);
    template.render_html().ok();
    Ok(())
}

/// Mirrors `viewDiscussionsPage(WebConnection)`.
pub fn view_discussions_page(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    XssUtil::clear(web_connection);

    web_connection.session().set(
        "page",
        crate::duckhttpd::web_connection::SessionValue::Str("community".into()),
    );

    let matches = web_connection.get_matches();
    let Some(match0) = matches.first() else {
        return Ok(());
    };

    let group = if is_numeric(match0) && web_connection.get_route_request().contains("/id/discussions") {
        let Some(found) = GroupDao::get_group(match0.parse().unwrap_or(0)) else {
            not_found(web_connection);
            return Ok(());
        };

        if !found.get_alias().trim().is_empty() {
            web_connection
                .redirect(&format!("/groups/{}/discussions", found.get_alias()));
            return Ok(());
        }

        found
    } else if web_connection.get_route_request().contains("/discussions")
        && !web_connection.get_route_request().contains("id/")
    {
        match GroupDao::get_group_by_alias(match0) {
            Some(group) => group,
            None => {
                not_found(web_connection);
                return Ok(());
            }
        }
    } else {
        not_found(web_connection);
        return Ok(());
    };

    let mut page = 1;

    if let Some(page_value) = web_connection.get_matches().get(1).and_then(|value| value.parse().ok()) {
        page = page_value;
    }

    let mut template = web_connection.template("groups/view_discussions");
    template.set("group", TemplateValue::of(&group));
    render(web_connection, &group, &mut template, page);
    template.render_html().ok();
    Ok(())
}

/// Mirrors `render(WebConnection, Group, Template, int)`.
fn render(
    web_connection: &WebConnection,
    group: &Group,
    template: &mut TwigTemplate<'_>,
    page_number: i32,
) {
    let logged_in = web_connection.session().get_boolean("authenticated");
    let group_id = group.get_id();
    template.set("hasMember", TemplateValue::of(false));
    template.set(
        "canViewForum",
        TemplateValue::of(group.get_forum_type() == GroupForumType::Public),
    );
    template.set("canPostForum", TemplateValue::of(false));

    let mut can_view_forum = group.get_forum_type() == GroupForumType::Public;

    if logged_in {
        let user_id = web_connection.session().get_int("user.id");
        let group_member = group.get_member(user_id);

        template.set(
            "canPostForum",
            TemplateValue::of(group.can_forum_post(group_member.as_ref())),
        );

        if let Some(group_member) = group_member {
            template.set("hasMember", TemplateValue::of(true));
            can_view_forum = logged_in && group.can_view_forum(Some(&group_member));
            template.set("groupMember", TemplateValue::of(group_member));
        }
    }

    let mut page_number = page_number;

    if page_number <= 0 {
        page_number = 1;
    }

    let mut discussion_topics = Vec::new();

    let limit = GameConfiguration::get_instance().get_integer("discussions.per.page");
    let pages;

    if can_view_forum {
        let discussion_count = GroupDiscussionDao::count_discussions(group_id);
        pages = if discussion_count > 0 {
            (discussion_count as f64 / limit as f64).ceil() as i32
        } else {
            1
        };
        discussion_topics =
            GroupDiscussionDao::get_discussions(group_id, page_number, limit, web_connection.session().get_int_or("user.id", 0));
    } else {
        pages = 1;
    }

    for i in 1..(3 + 1) {
        let new_page = page_number - i;

        if new_page >= 1 {
            template.set(&format!("previousPage{i}"), TemplateValue::of(new_page));
        } else {
            template.set(&format!("previousPage{i}"), TemplateValue::of(-1));
        }
    }

    for i in 1..(3 + 1) {
        let new_page = page_number + i;

        if new_page > 1 && new_page <= pages {
            template.set(&format!("nextPage{i}"), TemplateValue::of(new_page));
        } else {
            template.set(&format!("nextPage{i}"), TemplateValue::of(-1));
        }
    }

    template.set("currentPage", TemplateValue::of(page_number));
    template.set("pages", TemplateValue::of(pages));
    template.set("discussionTopics", TemplateValue::of(discussion_topics));
}
