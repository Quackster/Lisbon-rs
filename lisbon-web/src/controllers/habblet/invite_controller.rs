//! Mirrors `org.alexdev.http.controllers.habblet.InviteController`.

use lisbon_server::dao::mysql::messenger_dao::MessengerDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::game::messenger::messenger::Messenger;
use lisbon_server::game::messenger::messenger_manager::MessengerManager;
use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::util::string_util::StringUtil;

use crate::duckhttpd::{ResponseBuilder, TemplateValue, WebConnection};

/// Mirrors `inviteLink(WebConnection)`.
pub fn invite_link(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut template = web_connection.template("habblet/invite_referralLink");
    template.render();
    Ok(())
}

/// Mirrors `searchContent(WebConnection)`.
pub fn search_content(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut template = web_connection.template("habblet/invite_searchContent");

    let player_details = match template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
    {
        Some(details) => details,
        None => return Ok(()),
    };

    let messenger = Messenger::from_details(&player_details);

    let search_string = web_connection
        .post()
        .get_string("searchString")
        .unwrap_or_default();
    let page_id = if web_connection.post().contains("pageNumber") {
        web_connection
            .post()
            .get_int("pageNumber")
            .map(|value| value - 1)
            .unwrap_or(0)
    } else {
        0
    };
    let mut next_page_id = -1;
    let mut previous_page_id = -1;

    let mut searched_friends: Vec<PlayerDetails> = Vec::new();

    for user_id in MessengerDao::search(&search_string) {
        if player_details.get_id() == user_id {
            continue;
        }

        if let Some(friend) = PlayerDao::get_details(user_id) {
            searched_friends.push(friend);
        }
    }

    searched_friends.sort_by(|a, b| a.get_name().cmp(b.get_name()));

    let search_map = StringUtil::paginate(&searched_friends, 5);

    let search_results = search_map
        .get(&(page_id as usize))
        .cloned()
        .unwrap_or_default();

    if let Some(page) = page_id
        .checked_sub(1)
        .and_then(|value| usize::try_from(value).ok())
    {
        if search_map.contains_key(&page) {
            previous_page_id = page_id - 1;
        }
    }

    if let Some(page) = page_id
        .checked_add(1)
        .and_then(|value| usize::try_from(value).ok())
    {
        if search_map.contains_key(&page) {
            next_page_id = page_id + 1;
        }
    }

    next_page_id = if next_page_id > -1 { next_page_id + 1 } else { -1 };
    previous_page_id = if previous_page_id > -1 { previous_page_id + 1 } else { -1 };

    template.set("searchResults", TemplateValue::of(search_results));
    template.set("currentPage", TemplateValue::of(page_id + 1));
    template.set("totalPages", TemplateValue::of(search_map.len()));
    template.set("previousPageId", TemplateValue::of(previous_page_id));
    template.set("nextPageId", TemplateValue::of(next_page_id));
    template.set("messenger", TemplateValue::of(messenger));
    template.render();
    Ok(())
}

/// Mirrors `confirmAddFriend(WebConnection)`.
pub fn confirm_add_friend(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let player_details = match PlayerDao::get_details(user_id) {
        Some(details) => details,
        None => {
            web_connection.send_string("");
            return Ok(());
        }
    };

    let mut template = web_connection.template("habblet/invite_confirmAddFriend");
    template.set(
        "username",
        TemplateValue::of(player_details.get_name().to_string()),
    );
    template.render();
    Ok(())
}

/// Mirrors `addFriend(WebConnection)`.
pub fn add_friend(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let account_id = web_connection.post().get_int("accountId").unwrap_or(0);

    let mut template = web_connection.template("habblet/invite_addFriend");
    template.set(
        "message",
        TemplateValue::of(create_friend_request_response(web_connection, account_id)),
    );
    template.render();
    Ok(())
}

/// Mirrors `add(WebConnection)`.
pub fn add(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let account_id = web_connection.post().get_int("accountId").unwrap_or(0);

    let mut response = ResponseBuilder::create(format!(
        "Dialog.showInfoDialog(\"add-friend-messages\", \"{}\", \"OK\");",
        create_friend_request_response(web_connection, account_id)
    ));
    response.headers().push((
        "Content-Type".to_string(),
        "application/x-javascript".to_string(),
    ));
    web_connection.send(response);
    Ok(())
}

/// Mirrors `createFriendRequestResponse(WebConnection, int)`.
pub fn create_friend_request_response(web_connection: &WebConnection, account_id: i32) -> String {
    let user_id = web_connection.session().get_int("user.id");

    let target = MessengerManager::get_instance().get_messenger_data_by_id(account_id);
    let callee = MessengerManager::get_instance().get_messenger_data_by_id(user_id);

    let response = match (target, callee) {
        (Some(target), Some(callee)) => {
            let abigail_ryan = target
                .get_messenger_user()
                .get_username()
                .eq_ignore_ascii_case("Abigail.Ryan");

            if abigail_ryan {
                "There was an error finding the user for the friend request.".to_string()
            } else if callee.is_friends_limit_reached() {
                "Your friends list is full.".to_string()
            } else if target.has_friend(user_id) {
                "This person is already your friend".to_string()
            } else if target.has_request(user_id) {
                "There is already a friend request for this user.".to_string()
            } else if target.is_friends_limit_reached() {
                "This user's friend list is full.".to_string()
            } else if !target.allows_friend_requests() {
                "This user does not accept friend requests at the moment.".to_string()
            } else if web_connection.post().get_int("accountId").unwrap_or(0) == user_id {
                "There was an error processing your request.".to_string()
            } else {
                let callee_user = callee.get_messenger_user();
                target.add_request(&callee_user);
                // NOTE: the Java `RconUtil.sendCommand(FRIEND_REQUEST, ...)` push to the
                // game server is not ported (RconUtil is absent from the Rust port); the
                // request itself is persisted via `addRequest`.
                "Friend request has been sent successfully.".to_string()
            }
        }
        _ => "There was an error finding the user for the friend request.".to_string(),
    };

    response
}
