//! Mirrors `org.alexdev.http.controllers.groups.discussions.DiscussionPreviewController`.

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::util::date_util::DateUtil;

use crate::dao::group_discussion_dao::GroupDiscussionDao;
use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::util::bbcode::BBCode;
use crate::util::html_util::HtmlUtil;

fn formatted_message(topic_message: &str) -> String {
    BBCode::format(&HtmlUtil::escape(&BBCode::normalise(topic_message)), false)
}

/// Mirrors `previewtopic(WebConnection)`.
pub fn previewtopic(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);

    let topic_name = web_connection.post().get_string("topicName").unwrap_or_default();
    let topic_message = web_connection.post().get_string("message").unwrap_or_default();

    let user_id = web_connection.session().get_int("user.id");
    let display_badges = GroupDiscussionDao::get_display_badges(user_id);
    let _ = group_id;

    let mut template = web_connection.template("groups/discussions/previewtopic");
    template.set("topicName", TemplateValue::of(topic_name));
    template.set("topicMessage", TemplateValue::of(formatted_message(&topic_message)));
    template.set(
        "previewDay",
        TemplateValue::of(
            DateUtil::get_date(
                DateUtil::get_current_time_seconds() as i64,
                "MMM dd, yyyy",
            )
            .replace("am", "AM")
            .replace("pm", "PM")
            .replace('.', ""),
        ),
    );
    template.set(
        "previewTime",
        TemplateValue::of(
            DateUtil::get_date(DateUtil::get_current_time_seconds() as i64, "h:mm a")
                .replace("am", "AM")
                .replace("pm", "PM")
                .replace('.', ""),
        ),
    );
    template.set("userReplies", TemplateValue::of(GroupDiscussionDao::count_user_replies(user_id)));

    template.set("hasBadge", TemplateValue::of(false));
    template.set("hasGroup", TemplateValue::of(false));

    if let Some(badge) = &display_badges.0 {
        template.set("hasBadge", TemplateValue::of(true));
        template.set("badge", TemplateValue::of(badge));
    }

    if let Some(group_badge) = &display_badges.1 {
        template.set("hasGroup", TemplateValue::of(true));
        // Mirrors `((PlayerDetails) template.get("playerDetails")).getFavouriteGroupId()`.
        let group_id_value = PlayerDao::get_details(user_id)
            .map(|player_details| player_details.get_favourite_group_id())
            .unwrap_or(0);
        template.set("groupId", TemplateValue::of(group_id_value));
        template.set("groupBadge", TemplateValue::of(group_badge));
    }

    template.render_html().ok();
    Ok(())
}

/// Mirrors `previewpost(WebConnection)`.
pub fn previewpost(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let topic_id = web_connection.post().get_int("topicId").unwrap_or(0);
    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);

    let Some(discussion_topic) = GroupDiscussionDao::get_discussion(
        group_id,
        topic_id,
        web_connection.session().get_int_or("user.id", 0),
    ) else {
        web_connection.redirect("/");
        return Ok(());
    };

    let topic_message = web_connection.post().get_string("message").unwrap_or_default();

    let user_id = web_connection.session().get_int("user.id");
    let display_badges = GroupDiscussionDao::get_display_badges(user_id);

    let mut template = web_connection.template("groups/discussions/previewpost");
    template.set(
        "postName",
        TemplateValue::of(format!("RE: {}", discussion_topic.get_topic_title())),
    );
    template.set("postMessage", TemplateValue::of(formatted_message(&topic_message)));
    template.set(
        "previewDay",
        TemplateValue::of(
            DateUtil::get_date(
                DateUtil::get_current_time_seconds() as i64,
                "MMM dd, yyyy",
            )
            .replace("am", "AM")
            .replace("pm", "PM")
            .replace('.', ""),
        ),
    );
    template.set(
        "previewTime",
        TemplateValue::of(
            DateUtil::get_date(DateUtil::get_current_time_seconds() as i64, "h:mm a")
                .replace("am", "AM")
                .replace("pm", "PM")
                .replace('.', ""),
        ),
    );
    template.set("userReplies", TemplateValue::of(GroupDiscussionDao::count_user_replies(user_id)));

    template.set("hasBadge", TemplateValue::of(false));
    template.set("hasGroup", TemplateValue::of(false));

    if let Some(badge) = &display_badges.0 {
        template.set("hasBadge", TemplateValue::of(true));
        template.set("badge", TemplateValue::of(badge));
    }

    if let Some(group_badge) = &display_badges.1 {
        template.set("hasGroup", TemplateValue::of(true));
        // Mirrors `((PlayerDetails) template.get("playerDetails")).getFavouriteGroupId()`.
        let group_id_value = PlayerDao::get_details(user_id)
            .map(|player_details| player_details.get_favourite_group_id())
            .unwrap_or(0);
        template.set("groupId", TemplateValue::of(group_id_value));
        template.set("groupBadge", TemplateValue::of(group_badge));
    }

    template.render_html().ok();
    Ok(())
}
