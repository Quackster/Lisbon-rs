//! Mirrors `org.alexdev.http.controllers.groups.discussions.DiscussionController`.

use lisbon_server::dao::mysql::group_dao::GroupDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::game::groups::group_forum_type::GroupForumType;
use lisbon_server::game::groups::group_member_rank::GroupMemberRank;
use lisbon_server::game::groups::group::Group;
use lisbon_server::game::player::player_rank::PlayerRank;
use lisbon_server::util::config::game_configuration::GameConfiguration;

use crate::dao::group_discussion_dao::GroupDiscussionDao;
use crate::dao::reply_dao::ReplyDao;
use crate::duckhttpd::web_connection::SessionValue;
use crate::duckhttpd::{Settings, TemplateValue, WebConnection};
use http::StatusCode;
use crate::game::groups::discussion_topic::DiscussionTopic;
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

/// Mirrors `Group.hasTopicAdmin(PlayerRank)`.
fn has_topic_admin(rank: Option<PlayerRank>) -> bool {
    rank.map(|rank| rank.rank_id()).unwrap_or(0) >= 5
}

/// Mirrors `viewDiscussion(WebConnection)`.
pub fn view_discussion(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    XssUtil::clear(web_connection);

    web_connection.session().set("page", SessionValue::Str("community".into()));

    let matches = web_connection.get_matches();
    let Some(match0) = matches.first() else {
        return Ok(());
    };

    let discussion_id = web_connection
        .get_matches()
        .get(1)
        .and_then(|value| value.parse().ok())
        .unwrap_or(0);

    let mut page_number = 1;
    let mut has_page_specified = false;

    if let Some(page) = web_connection
        .get_matches()
        .get(2)
        .and_then(|value| value.parse().ok())
    {
        page_number = page;
        has_page_specified = true;
    }

    let group = if is_numeric(match0) && web_connection.get_route_request().contains("/id/discussions") {
        let Some(found) = GroupDao::get_group(match0.parse().unwrap_or(0)) else {
            not_found(web_connection);
            return Ok(());
        };

        if !found.get_alias().trim().is_empty() {
            let mut link = format!(
                "/groups/{}/discussions/{discussion_id}/id",
                found.get_alias()
            );

            if has_page_specified {
                link = format!("{link}/page/{page_number}");
            }

            web_connection.redirect(&link);
            return Ok(());
        }

        found
    } else {
        match GroupDao::get_group_by_alias(match0) {
            Some(group) => group,
            None => {
                not_found(web_connection);
                return Ok(());
            }
        }
    };

    let Some(discussion_topic) = GroupDiscussionDao::get_discussion(
        group.get_id(),
        discussion_id,
        web_connection.session().get_int_or("user.id", 0),
    ) else {
        not_found(web_connection);
        return Ok(());
    };

    let mut template = web_connection.template("groups/discussion");

    if page_number <= 0 {
        page_number = 1;
    }

    if !web_connection.session().contains(&format!("hasViewedDiscussion{discussion_id}")) {
        web_connection.session().set(
            &format!("hasViewedDiscussion{discussion_id}"),
            SessionValue::Bool(true),
        );
        GroupDiscussionDao::increment_views(discussion_id);
    }

    append_paged_data(
        &mut template,
        web_connection,
        &group,
        &discussion_topic,
        page_number,
    );
    template.render_html().ok();
    Ok(())
}

/// Mirrors `appendpagedata(Template, WebConnection, Group, DiscussionTopic, int)`.
pub fn append_paged_data(
    template: &mut TwigTemplate<'_>,
    web_connection: &WebConnection,
    group: &Group,
    discussion_topic: &DiscussionTopic,
    page_number: i32,
) {
    let logged_in = web_connection.session().get_boolean("authenticated");
    let user_id = web_connection.session().get_int_or("user.id", 0);
    let mut has_admin = false;

    if logged_in {
        let Some(player_details) = PlayerDao::get_details(user_id) else {
            return;
        };

        has_admin = has_topic_admin(player_details.get_rank());
    }

    template.set("group", TemplateValue::of(group));
    template.set("hasMember", TemplateValue::of(false));
    template.set("hasMessage", TemplateValue::of(false));
    template.set(
        "canViewForum",
        TemplateValue::of(group.get_forum_type() == GroupForumType::Public),
    );
    template.set("canReplyForum", TemplateValue::of(false));

    if logged_in {
        let group_member = group.get_member(user_id);

        template.set(
            "canViewForum",
            TemplateValue::of(logged_in && group.can_view_forum(group_member.as_ref())),
        );
        template.set(
            "canReplyForum",
            TemplateValue::of(logged_in && group.can_reply_forum(group_member.as_ref())),
        );

        if let Some(group_member) = group_member {
            template.set("hasMember", TemplateValue::of(true));

            if !has_admin {
                let member_rank = group_member.get_member_rank();
                has_admin = member_rank == Some(GroupMemberRank::Administrator)
                    || member_rank == Some(GroupMemberRank::Owner);
            }

            template.set("groupMember", TemplateValue::of(group_member));
        }
    }

    if !discussion_topic.is_open {
        template.set("canReplyForum", TemplateValue::of(false));
    }

    let can_view_forum = template
        .get("canViewForum")
        .map_or(false, |value| value.value().as_bool().unwrap_or(false));

    if !can_view_forum {
        template.set("hasMessage", TemplateValue::of(true));
        template.set(
            "message",
            TemplateValue::of(
                "View forums denied. Please check that you are logged in and have the appropriate rights to view the forums. If you are logged in and still can't view the forums, the group may be private. If so, you need to join the group in order to view the forums. ",
            ),
        );
        return;
    }

    let first_reply = GroupDiscussionDao::get_first_reply(discussion_topic.id);
    template.set("firstReply", TemplateValue::of(first_reply));

    let limit = GameConfiguration::get_instance().get_integer("discussions.replies.per.page");

    let reply_count = GroupDiscussionDao::count_replies(discussion_topic.id);
    let pages = if reply_count > 0 {
        (reply_count as f64 / limit as f64).ceil() as i32
    } else {
        1
    };
    let reply_list =
        GroupDiscussionDao::get_replies(discussion_topic.id, page_number, limit, user_id);

    if user_id > 0 {
        let is_new = reply_list.iter().filter(|reply| reply.is_new).count();

        if is_new > 0 {
            let new_replies: Vec<_> = reply_list.iter().filter(|reply| reply.is_new).cloned().collect();
            ReplyDao::read(user_id, &new_replies);
        }
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
    template.set("replyList", TemplateValue::of(reply_list));
    template.set("discussionId", TemplateValue::of(discussion_topic.id));
    template.set("discussionTopic", TemplateValue::of(discussion_topic));
    template.set("hasTopicAdmin", TemplateValue::of(has_admin));
}
