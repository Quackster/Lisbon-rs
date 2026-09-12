//! Mirrors `org.alexdev.http.controllers.site.MinimailController`.

use lisbon_server::dao::mysql::messenger_dao::MessengerDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::dao::mysql::player_statistics_dao::PlayerStatisticsDao;
use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::game::player::statistics::player_statistic::PlayerStatistic;
use lisbon_server::game::wordfilter::wordfilter_manager::WordfilterManager;
use lisbon_server::util::date_util::DateUtil;
use lisbon_server::util::string_util::StringUtil;

use crate::dao::minimail_dao::MinimailDao;
use crate::duckhttpd::Template;
use crate::duckhttpd::TemplateValue;
use crate::duckhttpd::WebConnection;
use crate::duckhttpd::web_connection::SessionValue;
use crate::game::minimail::minimail_message::MinimailMessage;
use crate::util::bbcode::BBCode;
use crate::util::html_util::HtmlUtil;
use crate::util::xss_util::XssUtil as XSSUtil;

/// Mirrors `loadMessages(WebConnection)`.
pub fn load_messages(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    if !web_connection.session().get_boolean("authenticated") {
        web_connection.send_string("");
        return Ok(());
    }

    let mut template = web_connection.template("habblet/minimail/minimail_messages");
    append_messages(
        web_connection,
        &mut template,
        false,
        false,
        false,
        false,
        false,
        false,
    );
    template.render();

    Ok(())
}

/// Mirrors `appendMessages(WebConnection, Template, boolean, boolean, boolean, boolean, boolean, boolean)`.
pub fn append_messages(
    web_connection: &WebConnection,
    template: &mut impl Template,
    is_page_load: bool,
    message_sent: bool,
    message_deleted: bool,
    message_undeleted: bool,
    trash_emptied: bool,
    is_muted: bool,
) {
    XSSUtil::clear(web_connection);

    let mut label = web_connection
        .post()
        .get_string("label")
        .unwrap_or_default();

    if label.trim().is_empty() {
        if !web_connection.session().contains("minimailLabel") {
            label = "inbox".to_string();
        } else {
            label = web_connection
                .session()
                .get_string("minimailLabel")
                .unwrap_or_else(|| "inbox".to_string());

            if label == "conversation" && !web_connection.post().contains("conversationId") {
                label = "inbox".to_string();
            }
        }
    }

    let start_number = match web_connection.post().get_int("start") {
        Some(value) => value,
        None => {
            web_connection.send_string("");
            return;
        }
    };

    let unread_only = web_connection.post().get_boolean("unreadOnly");

    let user_id = web_connection.session().get_int("user.id");

    let page_number = if start_number > 0 {
        start_number / 10
    } else {
        0
    };

    web_connection.session().set("minimailLabel", SessionValue::Str(label.clone()));
    template.set("minimailLabel", TemplateValue::of(label.clone()));

    let mut entire_message_list: Vec<MinimailMessage> = Vec::new();

    match label.to_lowercase().as_str() {
        "inbox" => entire_message_list = MinimailDao::get_messages(user_id),
        "sent" => entire_message_list = MinimailDao::get_messages_sent(user_id),
        "trash" => entire_message_list = MinimailDao::get_message_trash(user_id),
        "conversation" => {
            let conversation_id = web_connection
                .post()
                .get_int("conversationId")
                .unwrap_or(0);
            entire_message_list = MinimailDao::get_messages_conversation(user_id, conversation_id);
        }
        _ => {}
    }

    if unread_only {
        entire_message_list.retain(|message| !message.is_read);
    }

    template.set("unreadOnly", TemplateValue::of(unread_only));

    entire_message_list.sort_by(|a, b| b.date_sent.cmp(&a.date_sent));

    let paginated_messages = StringUtil::paginate_ext(&entire_message_list, 10, true);
    let mut minimail_messages = paginated_messages
        .get(&(page_number as usize))
        .cloned()
        .unwrap_or_default();

    let page = page_number as usize;

    template.set(
        "showOlder",
        TemplateValue::of(paginated_messages.contains_key(&(page + 1))),
    );
    template.set(
        "showOldest",
        TemplateValue::of(paginated_messages.contains_key(&(page + 2))),
    );
    template.set(
        "showNewer",
        TemplateValue::of(page > 0 && paginated_messages.contains_key(&(page - 1))),
    );
    template.set(
        "showNewest",
        TemplateValue::of(page >= 2 && paginated_messages.contains_key(&(page - 2))),
    );

    template.set("minimailMessages", TemplateValue::of(&minimail_messages));
    template.set(
        "totalMessages",
        TemplateValue::of(entire_message_list.len()),
    );
    template.set("minimailClient", TemplateValue::of(false));

    let mut end_page = 10;
    let mut start_page = start_number;

    if start_number != 0 {
        start_page += 1;
        end_page = start_number + 10;
    } else {
        start_page = 1;
    }

    if end_page > entire_message_list.len() as i32 {
        end_page = entire_message_list.len() as i32;
    }

    template.set("startPage", TemplateValue::of(start_page));
    template.set("endPage", TemplateValue::of(end_page));

    {
        let mut player_details_map: std::collections::HashMap<i32, Option<_>> =
            std::collections::HashMap::new();

        for message in &mut minimail_messages {
            if !player_details_map.contains_key(&message.to_id) {
                player_details_map.insert(message.to_id, PlayerDao::get_details(message.to_id));
            }

            if !player_details_map.contains_key(&message.sender_id) {
                player_details_map
                    .insert(message.sender_id, PlayerDao::get_details(message.sender_id));
            }

            message.set_author(
                player_details_map
                    .get(&message.sender_id)
                    .and_then(|details| details.clone()),
            );
            message.set_target(
                player_details_map
                    .get(&message.to_id)
                    .and_then(|details| details.clone()),
            );
        }
    }

    if !is_page_load {
        let total = entire_message_list.len();

        if message_sent {
            if is_muted {
                web_connection.set_header(
                    "X-JSON",
                    &format!("{{\"message\":\"You are muted and cannot send messages.\",\"totalMessages\":{total}}}"),
                );
            } else {
                web_connection.set_header(
                    "X-JSON",
                    &format!("{{\"message\":\"Message sent successfully.\",\"totalMessages\":{total}}}"),
                );
            }
        } else if message_deleted {
            web_connection.set_header(
                "X-JSON",
                &format!("{{\"message\":\"The message has been moved to the trash. You can undelete it, if you wish\",\"totalMessages\":{total}}}"),
            );
        } else if message_undeleted {
            web_connection.set_header(
                "X-JSON",
                &format!("{{\"message\":\"Message undeleted\",\"totalMessages\":{total}}}"),
            );
        } else if trash_emptied {
            web_connection.set_header(
                "X-JSON",
                &format!("{{\"message\":\"The trash has been emptied. Good Job!\",\"totalMessages\":{total}}}"),
            );
        } else {
            web_connection.set_header(
                "X-JSON",
                &format!("{{\"totalMessages\":{total}}}"),
            );
        }
    }
}

/// Mirrors `recipients(WebConnection)`.
pub fn recipients(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");

    let friends = MessengerDao::get_friends(user_id);

    let mut recipients = String::new();
    let mut i = 0;

    for friend in friends.values() {
        i += 1;

        recipients.push_str(&format!(
            "{{\"id\":{},\"name\":\"{}\"}}",
            friend.get_user_id(),
            friend.get_username()
        ));

        if friends.len() > i {
            recipients.push(',');
        }
    }

    web_connection.send_string(&format!("/*-secure-\n[{recipients}]\n */"));

    Ok(())
}

/// Mirrors `preview(WebConnection)`.
pub fn preview(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.send_string("");
        return Ok(());
    }

    web_connection.send_string(&BBCode::format(
        &HtmlUtil::escape(
            web_connection
                .post()
                .get_string("body")
                .as_deref()
                .unwrap_or_default(),
        ),
        false,
    ));

    Ok(())
}

/// Mirrors `sendMessage(WebConnection)`.
pub fn send_message(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.send_string("");
        return Ok(());
    }

    let mut minimail_message_list: Vec<MinimailMessage> = Vec::new();
    let message = web_connection
        .post()
        .get_string("body")
        .unwrap_or_default();

    let mut template = web_connection.template("habblet/minimail/minimail_messages");

    let user_id = web_connection.session().get_int("user.id");
    let player_details = match template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
    {
        Some(details) => details,
        None => {
            template.render();
            return Ok(());
        }
    };

    let mute_expire_time =
        PlayerStatisticsDao::get_statistic_long(player_details.get_id(), PlayerStatistic::MuteExpiresAt);
    let is_muted = mute_expire_time > 0 && mute_expire_time > DateUtil::get_current_time_seconds() as i64;

    if !is_muted {
        if web_connection.post().contains("recipientIds") {
            let recipients = web_connection
                .post()
                .get_string("recipientIds")
                .map_or(Vec::new(), |value| value.split(',').map(str::to_string).collect());
            let subject = web_connection
                .post()
                .get_string("subject")
                .unwrap_or_default();

            for data in recipients {
                if !(!data.is_empty() && data.chars().all(|character| character.is_ascii_digit())) {
                    continue;
                }

                let to_id = match data.parse::<i32>() {
                    Ok(value) => value,
                    Err(_) => continue,
                };

                if !MessengerDao::friend_exists(user_id, to_id) {
                    continue;
                }

                if WordfilterManager::filter_sentence(&message) == message {
                    minimail_message_list.push(MinimailMessage::new(
                        -1,
                        user_id,
                        to_id,
                        user_id,
                        false,
                        &subject,
                        &message,
                        0,
                        0,
                        false,
                    ));
                    minimail_message_list.push(MinimailMessage::new(
                        -1,
                        to_id,
                        to_id,
                        user_id,
                        false,
                        &subject,
                        &message,
                        0,
                        0,
                        false,
                    ));
                }
            }
        } else if web_connection.post().contains("messageId") {
            let message_id = web_connection.post().get_int("messageId").unwrap_or(0);

            if let Some(mut minimail_message) = MinimailDao::get_message(message_id, user_id) {
                minimail_message.set_conversation_id(message_id);
                MinimailDao::update_message(&minimail_message);

                if WordfilterManager::filter_sentence(&message) == message {
                    let re_subject = format!("Re: {}", minimail_message.subject);
                    let conversation_id = minimail_message.conversation_id;
                    let sender_id = minimail_message.sender_id;

                    minimail_message_list.push(MinimailMessage::new(
                        -1,
                        user_id,
                        sender_id,
                        user_id,
                        false,
                        &re_subject,
                        &message,
                        0,
                        conversation_id,
                        false,
                    ));
                    minimail_message_list.push(MinimailMessage::new(
                        -1,
                        sender_id,
                        sender_id,
                        user_id,
                        false,
                        &re_subject,
                        &message,
                        0,
                        conversation_id,
                        false,
                    ));
                }
            }
        }

        MinimailDao::create_messages(&minimail_message_list);
    }

    append_messages(
        web_connection,
        &mut template,
        false,
        true,
        false,
        false,
        false,
        is_muted,
    );
    template.render();

    Ok(())
}

/// Mirrors `loadMessage(WebConnection)`.
pub fn load_message(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.send_string("");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");

    let message_id = match web_connection.get().get_int("messageId") {
        Some(value) => value,
        None => {
            web_connection.send_string("1");
            return Ok(());
        }
    };

    if !(message_id > 0) {
        web_connection.send_string("2");
        return Ok(());
    }

    let mut minimail_message = match MinimailDao::get_message(message_id, user_id) {
        Some(message) => message,
        None => {
            web_connection.send_string("");
            return Ok(());
        }
    };

    let minimail_label = web_connection
        .session()
        .get_string("minimailLabel")
        .unwrap_or_default()
        .to_lowercase();

    if minimail_label == "conversation"
        && user_id != minimail_message.target_id
        && user_id != minimail_message.sender_id
    {
        web_connection.send_string("");
        return Ok(());
    }

    if minimail_label == "sent"
        && user_id != minimail_message.target_id
        && user_id != minimail_message.sender_id
    {
        web_connection.send_string("");
        return Ok(());
    }

    minimail_message.set_target(PlayerDao::get_details(minimail_message.to_id));
    minimail_message.set_author(PlayerDao::get_details(minimail_message.sender_id));

    if !minimail_message.is_read {
        minimail_message.set_read(true);
        MinimailDao::update_message(&minimail_message);
    }

    let mut template = web_connection.template("habblet/minimail/minimail_load_message");
    template.set("minimailLabel", TemplateValue::of(minimail_label));

    template.set("minimailMessage", TemplateValue::of(&minimail_message));
    template.render();

    Ok(())
}

/// Mirrors `deleteMessage(WebConnection)`.
pub fn delete_message(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.send_string("");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");

    let message_id = match web_connection.post().get_int("messageId") {
        Some(value) => value,
        None => {
            web_connection.send_string("");
            return Ok(());
        }
    };

    if !(message_id > 0) {
        web_connection.send_string("");
        return Ok(());
    }

    let mut minimail_message = match MinimailDao::get_message(message_id, user_id) {
        Some(message) => message,
        None => {
            web_connection.send_string("");
            return Ok(());
        }
    };

    if !minimail_message.is_trash {
        minimail_message.set_trash(true);
        MinimailDao::update_message(&minimail_message);
    } else {
        MinimailDao::delete_message(&minimail_message);
    }

    let mut template = web_connection.template("habblet/minimail/minimail_messages");
    append_messages(
        web_connection,
        &mut template,
        false,
        false,
        true,
        false,
        false,
        false,
    );
    template.render();

    Ok(())
}

/// Mirrors `undeleteMessage(WebConnection)`.
pub fn undelete_message(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.send_string("");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");

    let message_id = match web_connection.post().get_int("messageId") {
        Some(value) => value,
        None => {
            web_connection.send_string("");
            return Ok(());
        }
    };

    if !(message_id > 0) {
        web_connection.send_string("");
        return Ok(());
    }

    let mut minimail_message = match MinimailDao::get_message(message_id, user_id) {
        Some(message) => message,
        None => {
            web_connection.send_string("");
            return Ok(());
        }
    };

    minimail_message.set_trash(false);
    MinimailDao::update_message(&minimail_message);

    let mut template = web_connection.template("habblet/minimail/minimail_messages");
    append_messages(
        web_connection,
        &mut template,
        false,
        false,
        false,
        true,
        false,
        false,
    );
    template.render();

    Ok(())
}

/// Mirrors `emptyTrash(WebConnection)`.
pub fn empty_trash(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.send_string("");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    MinimailDao::empty_trash(user_id);

    let mut template = web_connection.template("habblet/minimail/minimail_messages");
    append_messages(
        web_connection,
        &mut template,
        false,
        false,
        false,
        false,
        true,
        false,
    );
    template.render();

    Ok(())
}
