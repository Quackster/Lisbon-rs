//! Mirrors `org.alexdev.http.controllers.groups.discussions.DiscussionActionsController`.

use lisbon_server::dao::mysql::group_dao::GroupDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::game::groups::group_forum_type::GroupForumType;
use lisbon_server::game::groups::group_member_rank::GroupMemberRank;
use lisbon_server::game::groups::group_permission_type::GroupPermissionType;
use lisbon_server::game::player::player_rank::PlayerRank;
use lisbon_server::util::config::game_configuration::GameConfiguration;

use crate::dao::group_discussion_dao::GroupDiscussionDao;
use crate::duckhttpd::{ResponseBuilder, WebConnection};
use crate::util::captcha::Captcha;
use crate::util::xss_util::XssUtil;

/// Mirrors `Group.hasTopicAdmin(PlayerRank)`.
fn has_topic_admin(rank: Option<PlayerRank>) -> bool {
    rank.map(|rank| rank.rank_id()).unwrap_or(0) >= 5
}

/// Mirrors `newtopic(WebConnection)`.
pub fn newtopic(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XssUtil::clear(web_connection);

    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut template = web_connection.template("groups/discussions/newpost");
    template.render_html().ok();
    Ok(())
}

/// Mirrors `savetopic(WebConnection)`.
pub fn savetopic(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XssUtil::clear(web_connection);

    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let captcha = web_connection.post().get_string("captcha").unwrap_or_default();
    let message = web_connection.post().get_string("message").unwrap_or_default();
    let topic_name = web_connection.post().get_string("topicName").unwrap_or_default();

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);

    if topic_name.trim().is_empty() || message.trim().is_empty() {
        let mut template = web_connection.template("groups/discussion_replies");
        template.set("hasMessage", crate::duckhttpd::TemplateValue::of(true));
        template.set("message", crate::duckhttpd::TemplateValue::of("Please supply a valid message."));
        template.render_html().ok();
        return Ok(());
    }

    let Some(captcha_session) = web_connection.session().get_string("captcha-text") else {
        web_connection.session().delete("captcha-text");

        let mut response = ResponseBuilder::create("");
        response.set_header("X-JSON", "{\"captchaError\":\"true\"}");
        web_connection.send(response);
        return Ok(());
    };

    if captcha_session != captcha {
        web_connection.session().delete("captcha-text");

        let mut response = ResponseBuilder::create("");
        response.set_header("X-JSON", "{\"captchaError\":\"true\"}");
        web_connection.send(response);
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let latest_message = GroupDiscussionDao::get_latest_reply(user_id);

    if let Some(latest) = &latest_message {
        if latest.get_message().starts_with(&message) {
            let mut template = web_connection.template("groups/discussion_replies");
            template.set("hasMessage", crate::duckhttpd::TemplateValue::of(true));
            template.set("message", crate::duckhttpd::TemplateValue::of("Do not spam the forums"));
            template.render_html().ok();
            return Ok(());
        }
    }

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.redirect("/");
        return Ok(());
    };

    if group.get_forum_type() == GroupForumType::Private
        || group.get_forum_permission() == GroupPermissionType::MemberOnly
        || group.get_forum_permission() == GroupPermissionType::AdminOnly
    {
        let group_member = group.get_member(user_id);

        if group_member.is_none() {
            web_connection.redirect("/");
            return Ok(());
        }

        if group.get_forum_permission() == GroupPermissionType::AdminOnly {
            let member_rank =
                group_member.as_ref().and_then(|member| member.get_member_rank());

            if member_rank != Some(GroupMemberRank::Administrator)
                && member_rank != Some(GroupMemberRank::Owner)
            {
                web_connection.redirect("/");
                return Ok(());
            }
        }
    }

    let mut topic_name = topic_name;

    if topic_name.chars().count() > 32 {
        topic_name = topic_name.chars().take(32).collect();
    }

    let topic_id = GroupDiscussionDao::create_discussion(group_id, user_id, &topic_name);
    GroupDiscussionDao::create_replies(topic_id, user_id, &message);

    web_connection.session().delete("captcha-text");
    web_connection
        .send_string(&format!("{}/discussions/{topic_id}/id", group.generate_click_link()));
    Ok(())
}

/// Mirrors `pingsession(WebConnection)`.
pub fn pingsession(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut response = ResponseBuilder::create("");
    response.set_header("X-JSON", "{\"privilegeLevel\":\"1\"}");
    web_connection.send(response);
    Ok(())
}

/// Mirrors `opentopicsettings(WebConnection)`.
pub fn opentopicsettings(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);
    let topic_id = web_connection.post().get_int("topicId").unwrap_or(0);

    let Some(discussion_topic) = GroupDiscussionDao::get_discussion(
        group_id,
        topic_id,
        web_connection.session().get_int_or("user.id", 0),
    ) else {
        web_connection.redirect("/");
        return Ok(());
    };

    let mut template = web_connection.template("groups/discussions/opentopicsettings");
    template.set("topic", crate::duckhttpd::TemplateValue::of(&discussion_topic));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `confirm_delete_topic(WebConnection)`.
pub fn confirm_delete_topic(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut template = web_connection.template("groups/discussions/confirm_delete_topic");
    template.render_html().ok();
    Ok(())
}

/// Mirrors `deletetopic(WebConnection)`.
pub fn deletetopic(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);
    let topic_id = web_connection.post().get_int("topicId").unwrap_or(0);

    let Some(discussion_topic) = GroupDiscussionDao::get_discussion(
        group_id,
        topic_id,
        web_connection.session().get_int_or("user.id", 0),
    ) else {
        web_connection.redirect("/");
        return Ok(());
    };

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.redirect("/");
        return Ok(());
    };

    let user_id = web_connection.session().get_int("user.id");

    if discussion_topic.creator_id != user_id {
        let Some(player_details) = PlayerDao::get_details(user_id) else {
            return Ok(());
        };

        let group_member = group.get_member(user_id);

        if !has_topic_admin(player_details.get_rank()) {
            let member_rank =
                group_member.as_ref().and_then(|member| member.get_member_rank());

            if group_member.is_none() || member_rank == Some(GroupMemberRank::Member) {
                return Ok(());
            }
        }
    }

    GroupDiscussionDao::delete_discussion(group_id, topic_id);
    web_connection.send_string("SUCCESS");
    Ok(())
}

/// Mirrors `savetopicsettings(WebConnection)`.
pub fn savetopicsettings(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);
    let topic_id = web_connection.post().get_int("topicId").unwrap_or(0);

    let mut discussion_topic =
        match GroupDiscussionDao::get_discussion(
            group_id,
            topic_id,
            web_connection.session().get_int_or("user.id", 0),
        ) {
            Some(topic) => topic,
            None => {
                web_connection.redirect("/");
                return Ok(());
            }
        };

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.redirect("/");
        return Ok(());
    };

    let user_id = web_connection.session().get_int("user.id");
    let Some(player_details) = PlayerDao::get_details(user_id) else {
        return Ok(());
    };

    if discussion_topic.creator_id != user_id {
        let group_member = group.get_member(user_id);

        if !has_topic_admin(player_details.get_rank()) {
            let member_rank =
                group_member.as_ref().and_then(|member| member.get_member_rank());

            if group_member.is_none() || member_rank == Some(GroupMemberRank::Member) {
                return Ok(());
            }
        }
    }

    let mut page_number = web_connection.post().get_int("page").unwrap_or(1);

    if page_number <= 0 {
        page_number = 1;
    }

    // Mirrors the Java `try` block: if `topicName` is absent the setters are
    // skipped.
    if let Some(topic_title) = web_connection.post().get_string("topicName") {
        let mut topic_title = topic_title;

        if topic_title.chars().count() > 32 {
            topic_title = topic_title.chars().take(32).collect();
        }

        discussion_topic.set_open(web_connection.post().get_int("topicClosed").unwrap_or(0) == 0);
        discussion_topic.set_stickied(web_connection.post().get_int("topicSticky").unwrap_or(0) == 1);
        discussion_topic.set_topic_title(&topic_title);
        GroupDiscussionDao::save_discussion(&discussion_topic);
    }

    let mut template = web_connection.template("groups/discussion_replies");
    super::discussion_controller::append_paged_data(
        &mut template,
        web_connection,
        &group,
        &discussion_topic,
        page_number,
    );
    template.render_html().ok();
    Ok(())
}

/// Mirrors `updatepost(WebConnection)`.
pub fn updatepost(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);
    let topic_id = web_connection.post().get_int("topicId").unwrap_or(0);
    let post_id = web_connection.post().get_int("postId").unwrap_or(0);
    let mut page_number = web_connection.post().get_int("page").unwrap_or(1);

    if page_number <= 0 {
        page_number = 1;
    }

    let Some(discussion_topic) = GroupDiscussionDao::get_discussion(
        group_id,
        topic_id,
        web_connection.session().get_int_or("user.id", 0),
    ) else {
        web_connection.redirect("/");
        return Ok(());
    };

    if !discussion_topic.is_open {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut discussion_reply =
        match GroupDiscussionDao::get_reply(discussion_topic.id, post_id, web_connection.session().get_int_or("user.id", 0))
        {
            Some(reply) => reply,
            None => {
                web_connection.redirect("/");
                return Ok(());
            }
        };

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.redirect("/");
        return Ok(());
    };

    if Captcha::matches(web_connection, &web_connection.post().get_string("captcha").unwrap_or_default()) {
        web_connection.session().delete("captcha-text");

        let mut response = ResponseBuilder::create("");
        response.set_header("X-JSON", "{\"captchaError\":\"true\"}");
        web_connection.send(response);
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");

    if discussion_reply.user_id != user_id {
        // The moderator override is commented out in the Java source; the
        // redirect is mirrored.
        web_connection.redirect("/");
        return Ok(());
    }

    let Some(player_details) = PlayerDao::get_details(user_id) else {
        return Ok(());
    };

    let message = web_connection.post().get_string("message").unwrap_or_default();
    discussion_reply.set_message(&message);

    if !has_topic_admin(player_details.get_rank()) {
        discussion_reply.set_edited(true);
    }

    GroupDiscussionDao::save_reply(&discussion_reply);

    web_connection.session().delete("captcha-text");

    let mut template = web_connection.template("groups/discussion_replies");
    super::discussion_controller::append_paged_data(
        &mut template,
        web_connection,
        &group,
        &discussion_topic,
        page_number,
    );
    template.render_html().ok();
    Ok(())
}

/// Mirrors `deletepost(WebConnection)`.
pub fn deletepost(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);
    let topic_id = web_connection.post().get_int("topicId").unwrap_or(0);
    let post_id = web_connection.post().get_int("postId").unwrap_or(0);
    let mut page_number = web_connection.post().get_int("page").unwrap_or(1);

    if page_number <= 0 {
        page_number = 1;
    }

    let Some(discussion_topic) = GroupDiscussionDao::get_discussion(
        group_id,
        topic_id,
        web_connection.session().get_int_or("user.id", 0),
    ) else {
        web_connection.redirect("/?1");
        return Ok(());
    };

    if !discussion_topic.is_open {
        web_connection.redirect("/?1");
        return Ok(());
    }

    let mut discussion_reply =
        match GroupDiscussionDao::get_reply(discussion_topic.id, post_id, web_connection.session().get_int_or("user.id", 0))
        {
            Some(reply) => reply,
            None => {
                web_connection.redirect("/?2");
                return Ok(());
            }
        };

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.redirect("/?3");
        return Ok(());
    };

    let user_id = web_connection.session().get_int("user.id");
    let Some(player_details) = PlayerDao::get_details(user_id) else {
        return Ok(());
    };

    if discussion_reply.user_id != user_id {
        let group_member = group.get_member(user_id);

        if !has_topic_admin(player_details.get_rank()) {
            let member_rank =
                group_member.as_ref().and_then(|member| member.get_member_rank());

            if group_member.is_none() || member_rank == Some(GroupMemberRank::Member) {
                return Ok(());
            }
        }
    }

    if discussion_reply.user_id != user_id {
        GroupDiscussionDao::delete_reply(&discussion_reply);
    } else {
        if !has_topic_admin(player_details.get_rank()) {
            discussion_reply.set_deleted(true);
            GroupDiscussionDao::save_reply(&discussion_reply);
        } else {
            GroupDiscussionDao::delete_reply(&discussion_reply);
        }
    }

    let mut template = web_connection.template("groups/discussion_replies");
    super::discussion_controller::append_paged_data(
        &mut template,
        web_connection,
        &group,
        &discussion_topic,
        page_number,
    );
    template.render_html().ok();
    Ok(())
}

/// Mirrors `savepost(WebConnection)`.
pub fn savepost(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);
    let topic_id = web_connection.post().get_int("topicId").unwrap_or(0);

    let Some(captcha_session) = web_connection.session().get_string("captcha-text") else {
        web_connection.session().delete("captcha-text");

        let mut response = ResponseBuilder::create("");
        response.set_header("X-JSON", "{\"captchaError\":\"true\"}");
        web_connection.send(response);
        return Ok(());
    };

    if captcha_session != web_connection.post().get_string("captcha").unwrap_or_default() {
        web_connection.session().delete("captcha-text");

        let mut response = ResponseBuilder::create("");
        response.set_header("X-JSON", "{\"captchaError\":\"true\"}");
        web_connection.send(response);
        return Ok(());
    }

    let message = web_connection.post().get_string("message").unwrap_or_default();

    if message.trim().is_empty() {
        let mut template = web_connection.template("groups/discussion_replies");
        template.set("hasMessage", crate::duckhttpd::TemplateValue::of(true));
        template.set("message", crate::duckhttpd::TemplateValue::of("Please supply a valid message."));
        template.render_html().ok();
        return Ok(());
    }

    let mut template = web_connection.template("groups/discussion_replies");

    let user_id = web_connection.session().get_int("user.id");
    let Some(player_details) = PlayerDao::get_details(user_id) else {
        return Ok(());
    };

    let Some(discussion_topic) = GroupDiscussionDao::get_discussion(
        group_id,
        topic_id,
        web_connection.session().get_int_or("user.id", 0),
    ) else {
        web_connection.redirect("/");
        return Ok(());
    };

    if !discussion_topic.is_open && !has_topic_admin(player_details.get_rank()) {
        web_connection.redirect("/");
        return Ok(());
    }

    let latest_message = GroupDiscussionDao::get_latest_reply(user_id);

    if let Some(latest) = &latest_message {
        if latest.get_message().starts_with(&message) {
            template.set("hasMessage", crate::duckhttpd::TemplateValue::of(true));
            template.set("message", crate::duckhttpd::TemplateValue::of("Do not spam the forums"));
            template.render_html().ok();
            return Ok(());
        }
    }

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.redirect("/");
        return Ok(());
    };

    if group.get_forum_type() == GroupForumType::Private {
        let group_member = group.get_member(user_id);

        if group_member.is_none() {
            web_connection.redirect("/");
            return Ok(());
        }
    }

    web_connection.session().delete("captcha-text");

    GroupDiscussionDao::create_replies(topic_id, user_id, &message);

    let limit = GameConfiguration::get_instance().get_integer("discussions.replies.per.page");
    let reply_count = GroupDiscussionDao::count_replies(discussion_topic.id);
    let pages = if reply_count > 0 {
        (reply_count as f64 / limit as f64).ceil() as i32
    } else {
        1
    };

    super::discussion_controller::append_paged_data(
        &mut template,
        web_connection,
        &group,
        &discussion_topic,
        pages,
    );
    template.render_html().ok();
    Ok(())
}
