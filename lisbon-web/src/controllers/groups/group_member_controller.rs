//! Mirrors `org.alexdev.http.controllers.groups.GroupMemberController`.


use lisbon_server::dao::mysql::group_dao::GroupDao;
use lisbon_server::dao::mysql::group_member_dao::GroupMemberDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::game::groups::group_member_rank::GroupMemberRank;
use lisbon_server::game::player::player_rank::PlayerRank;
use lisbon_server::server::rcon::messages::rcon_header::RconHeader;

use std::collections::HashMap;

use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::util::rcon_util::RconUtil;

fn is_numeric(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|character| character.is_ascii_digit())
}

/// Mirrors `join(WebConnection)`.
pub fn join(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    let user_id = web_connection.session().get_int("user.id");
    let Some(player_details) = PlayerDao::get_details(user_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    if group.is_member(user_id)
        || group.is_pending_member(user_id)
        || group.get_group_type() == 2
    {
        web_connection.send_string("");
        return Ok(());
    }

    let group_type = group.get_group_type();
    let rank_id = player_details.get_rank().map(|rank| rank.rank_id()).unwrap_or(0);

    if group_type == 0 || group_type == 3 || rank_id >= PlayerRank::Moderator.rank_id() {
        let mut template = web_connection.template("groups/member/member_added");
        template.render_html().ok();
    }

    if group_type == 1 {
        let mut template = web_connection.template("groups/member/member_added_request");
        template.render_html().ok();
    }

    if rank_id >= PlayerRank::Moderator.rank_id() {
        GroupMemberDao::add_member(user_id, group_id, false);
    } else {
        GroupMemberDao::add_member(user_id, group_id, group_type == 1);
    }

    Ok(())
}

/// Mirrors `confirmLeave(WebConnection)`.
pub fn confirm_leave(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut template = web_connection.template("groups/member/confirm_leave");
    template.render_html().ok();
    Ok(())
}

/// Mirrors `leave(WebConnection)`.
pub fn leave(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    let user_id = web_connection.session().get_int("user.id");

    if group.is_member(user_id) {
        let group_member = group.get_member(user_id);

        GroupMemberDao::delete_member(user_id, group_id);

        if group_member.as_ref().is_some_and(|member| member.is_favourite(group_id)) {
            PlayerDao::save_favourite_group(user_id, 0);

            let mut parameters = HashMap::new();
            parameters.insert("userId".to_string(), user_id.to_string());
            RconUtil::send_command(RconHeader::RefreshGroupPerms, parameters);
        }
    }

    let mut template = web_connection.template("groups/member/leave");
    template.set("groupId", TemplateValue::of(group_id));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `memberlist(WebConnection)`.
pub fn memberlist(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);
    let user_id = web_connection.session().get_int("user.id");

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    if !group.has_administrator(user_id) {
        web_connection.send_string("");
        return Ok(());
    }

    let mut page_number = 1;

    if let Some(page_number_value) = web_connection.post().get_int("pageNumber") {
        page_number = page_number_value;
    }

    if page_number <= 0 {
        page_number = 1;
    }

    let pending_members = GroupMemberDao::count_members(group_id, true);
    let group_members = GroupMemberDao::count_members(group_id, false);

    let limit = 12;
    let is_pending = web_connection.post().get_boolean("pending");
    let group_member_list = GroupMemberDao::get_members(group_id, is_pending, "", page_number, limit);

    let member_count = if is_pending { pending_members } else { group_members };
    let mut pages = if member_count > 0 {
        (member_count as f64 / limit as f64).ceil() as i32
    } else {
        0
    };

    if pages == 0 {
        pages = 1;
    }

    let self_member = group.get_member(user_id);
    web_connection.set_header(
        "X-JSON",
        &format!("{{\"pending\":\"Pending members ({pending_members})\",\"members\":\"Members ({group_members})\"}}"),
    );

    let mut template = web_connection.template("groups/memberlist");
    template.set("pages", TemplateValue::of(pages));
    template.set("memberList", TemplateValue::of(group_member_list));
    template.set("currentPage", TemplateValue::of(page_number));
    template.set("selfMember", TemplateValue::of(self_member));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `confirmRevokeRights(WebConnection)`.
pub fn confirm_revoke_rights(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let target_ids = web_connection.post().get_array("targetIds").len();

    let mut template = web_connection.template("groups/member/confirm_revoke_rights");
    template.set("targetIds", TemplateValue::of(target_ids));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `revokeRights(WebConnection)`.
pub fn revoke_rights(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);
    let _user_id = web_connection.session().get_int("user.id");

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    let user_id = web_connection.session().get_int("user.id");

    let self_rank = group.get_member(user_id).and_then(|member| member.get_member_rank());

    if self_rank != Some(GroupMemberRank::Owner) {
        web_connection.send_string("");
        return Ok(());
    }

    for user in web_connection.post().get_array("targetIds") {
        if !is_numeric(&user) {
            continue;
        }

        let member_id = user.parse().unwrap_or(0);

        let Some(group_member) = group.get_member(member_id) else {
            continue;
        };

        if group_member.get_member_rank() == Some(GroupMemberRank::Owner) {
            continue;
        }

        GroupMemberDao::update_member(
            group_member.get_user_id(),
            group_member.get_group_id(),
            GroupMemberRank::Member,
            false,
        );
    }

    web_connection.send_string("OK");
    Ok(())
}

/// Mirrors `confirmGiveRights(WebConnection)`.
pub fn confirm_give_rights(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let target_ids = web_connection.post().get_array("targetIds").len();

    let mut template = web_connection.template("groups/member/confirm_give_rights");
    template.set("targetIds", TemplateValue::of(target_ids));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `giveRights(WebConnection)`.
pub fn give_rights(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);
    let _user_id = web_connection.session().get_int("user.id");

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    let user_id = web_connection.session().get_int("user.id");

    let self_rank = group.get_member(user_id).and_then(|member| member.get_member_rank());

    if self_rank != Some(GroupMemberRank::Owner) {
        web_connection.send_string("");
        return Ok(());
    }

    for user in web_connection.post().get_array("targetIds") {
        if !is_numeric(&user) {
            continue;
        }

        let member_id = user.parse().unwrap_or(0);

        let Some(group_member) = group.get_member(member_id) else {
            continue;
        };

        if group_member.get_member_rank() == Some(GroupMemberRank::Owner) {
            continue;
        }

        GroupMemberDao::update_member(
            group_member.get_user_id(),
            group_member.get_group_id(),
            GroupMemberRank::Administrator,
            false,
        );
    }

    web_connection.send_string("OK");
    Ok(())
}

/// Mirrors `confirmRemove(WebConnection)`.
pub fn confirm_remove(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let target_ids = web_connection.post().get_array("targetIds").len();

    let mut template = web_connection.template("groups/member/confirm_remove");
    template.set("targetIds", TemplateValue::of(target_ids));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `remove(WebConnection)`.
pub fn remove(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);
    let _user_id = web_connection.session().get_int("user.id");

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    let user_id = web_connection.session().get_int("user.id");

    let self_rank = group.get_member(user_id).and_then(|member| member.get_member_rank());

    if self_rank != Some(GroupMemberRank::Owner) && self_rank != Some(GroupMemberRank::Administrator) {
        web_connection.send_string("");
        return Ok(());
    }

    for user in web_connection.post().get_array("targetIds") {
        if !is_numeric(&user) {
            continue;
        }

        let member_id = user.parse().unwrap_or(0);

        let Some(group_member) = group.get_member(member_id) else {
            continue;
        };

        if group_member.get_member_rank() == Some(GroupMemberRank::Owner)
            || group_member.get_member_rank() == Some(GroupMemberRank::Administrator)
        {
            continue;
        }

        GroupMemberDao::delete_member(
            group_member.get_user_id(),
            group_member.get_group_id(),
        );

        if group_member.is_favourite(group_id) {
            PlayerDao::save_favourite_group(group_member.get_user_id(), 0);

            if group_member
                .get_user()
                .is_some_and(|member_user| member_user.is_online())
            {
                let mut parameters = HashMap::new();
                parameters.insert("userId".to_string(), user_id.to_string());
                RconUtil::send_command(RconHeader::RefreshGroupPerms, parameters);
            }
        }
    }

    web_connection.send_string("OK");
    Ok(())
}

/// Mirrors `confirmAccept(WebConnection)`.
pub fn confirm_accept(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);

    let Some(group_name) = GroupDao::get_group_name(group_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    let mut template = web_connection.template("groups/member/confirm_accept");
    template.set("groupName", TemplateValue::of(group_name));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `accept(WebConnection)`.
pub fn accept(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);
    let _user_id = web_connection.session().get_int("user.id");

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    let user_id = web_connection.session().get_int("user.id");

    let self_rank = group.get_member(user_id).and_then(|member| member.get_member_rank());

    if self_rank != Some(GroupMemberRank::Owner) && self_rank != Some(GroupMemberRank::Administrator) {
        web_connection.send_string("");
        return Ok(());
    }

    for user in web_connection.post().get_array("targetIds") {
        if !is_numeric(&user) {
            continue;
        }

        let member_id = user.parse().unwrap_or(0);

        let Some(group_member) = group.get_pending_member(member_id) else {
            continue;
        };

        if group_member.get_member_rank() == Some(GroupMemberRank::Owner)
            || group_member.get_member_rank() == Some(GroupMemberRank::Administrator)
        {
            continue;
        }

        GroupMemberDao::update_member(
            group_member.get_user_id(),
            group_member.get_group_id(),
            GroupMemberRank::Member,
            false,
        );
    }

    web_connection.send_string("OK");
    Ok(())
}

/// Mirrors `confirmDecline(WebConnection)`.
pub fn confirm_decline(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let target_ids = web_connection
        .post()
        .get_string("targetIds")
        .unwrap_or_default()
        .split(',')
        .count();

    let mut template = web_connection.template("groups/member/confirm_decline");
    template.set("targetIds", TemplateValue::of(target_ids));
    template.render_html().ok();
    Ok(())
}

/// Mirrors `decline(WebConnection)`.
pub fn decline(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let group_id = web_connection.post().get_int("groupId").unwrap_or(0);
    let _user_id = web_connection.session().get_int("user.id");

    let Some(group) = GroupDao::get_group(group_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    let user_id = web_connection.session().get_int("user.id");

    let self_rank = group.get_member(user_id).and_then(|member| member.get_member_rank());

    if self_rank != Some(GroupMemberRank::Owner) && self_rank != Some(GroupMemberRank::Administrator) {
        web_connection.send_string("");
        return Ok(());
    }

    for user in web_connection.post().get_array("targetIds") {
        if !is_numeric(&user) {
            continue;
        }

        let member_id = user.parse().unwrap_or(0);

        let Some(group_member) = group.get_pending_member(member_id) else {
            continue;
        };

        if group_member.get_member_rank() == Some(GroupMemberRank::Owner)
            || group_member.get_member_rank() == Some(GroupMemberRank::Administrator)
        {
            continue;
        }

        GroupMemberDao::delete_member(
            group_member.get_user_id(),
            group_member.get_group_id(),
        );
    }

    web_connection.send_string("OK");
    Ok(())
}
