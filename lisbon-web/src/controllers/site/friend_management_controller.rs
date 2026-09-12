//! Mirrors `org.alexdev.http.controllers.site.FriendManagementController`.

use std::cmp::Reverse;
use std::collections::HashMap;

use lisbon_server::dao::mysql::messenger_dao::MessengerDao;
use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::server::rcon::messages::rcon_header::RconHeader;

use crate::dao::friend_management_dao::FriendManagementDao;
use crate::duckhttpd::Template;
use crate::duckhttpd::TemplateValue;
use crate::duckhttpd::WebConnection;
use crate::util::rcon_util::RconUtil;
use crate::util::xss_util::XssUtil as XSSUtil;

/// Mirrors `friendmanagement(Template, WebConnection, int, int, int, String)`.
pub fn friendmanagement(
    template: &mut impl Template,
    web_connection: &WebConnection,
    limit: i32,
    current_page: i32,
    category_id: i32,
    search_string: Option<&str>,
) {
    XSSUtil::clear(web_connection);

    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return;
    }

    let player_id = template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
        .map_or(0, |details| details.get_id());

    let mut friends: Vec<_>;
    let friends_count;

    if let Some(search_string) = search_string {
        let mut sorted: Vec<_> = FriendManagementDao::get_friends_search(
            player_id,
            search_string,
            current_page,
            limit,
        )
        .into_iter()
        .collect();
        sorted.sort_by_key(|friend| Reverse(friend.get_last_online()));
        friends = sorted;
        friends_count =
            FriendManagementDao::get_friends_count_search(player_id, search_string);
    } else {
        let mut sorted: Vec<_> =
            FriendManagementDao::get_friends(player_id, current_page, limit)
                .into_iter()
                .collect();
        sorted.sort_by_key(|friend| Reverse(friend.get_last_online()));
        friends = sorted;
        friends_count = FriendManagementDao::get_friends_count(player_id);
    }

    let mut pages = if friends_count > 0 {
        (friends_count as f64 / limit as f64).ceil() as i32
    } else {
        0
    };

    if pages == 0 {
        pages = 1;
    }

    let mut categories = MessengerDao::get_categories(player_id);
    categories.sort_by_key(|category| category.get_id());

    for friend in &mut friends {
        if !categories.iter().any(|category| friend.get_category_id() == category.get_id()) {
            friend.set_category_id(0);
            MessengerDao::update_friend_category(player_id, friend.get_user_id(), 0);
        }
    }

    if category_id > -1 {
        friends = friends
            .into_iter()
            .filter(|friend| friend.get_category_id() == category_id)
            .collect();
    }

    template.set("friends", TemplateValue::of(friends));
    template.set("categories", TemplateValue::of(categories));
    template.set("currentPage", TemplateValue::of(current_page));
    template.set("pageLimit", TemplateValue::of(limit));

    if current_page >= 2 {
        template.set("firstPage", TemplateValue::of(1));
    } else {
        template.set("firstPage", TemplateValue::of(-1));
    }

    if current_page > 1 {
        template.set("previousPage", TemplateValue::of(current_page - 1));
    } else {
        template.set("previousPage", TemplateValue::of(-1));
    }

    if pages >= current_page + 1 {
        template.set("nextPage", TemplateValue::of(current_page + 1));
    } else {
        template.set("nextPage", TemplateValue::of(-1));
    }

    if pages >= current_page + 2 {
        template.set("lastPage", TemplateValue::of(pages));
    } else {
        template.set("lastPage", TemplateValue::of(-1));
    }
}

/// Mirrors `editCategory(WebConnection)`.
pub fn edit_category(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut template = web_connection.template("profile/profile_widgets/friend_category_widget");
    let player_id = template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
        .map_or(0, |details| details.get_id());

    let new_name = web_connection.post().get_string("name").unwrap_or_default();
    let category_id = web_connection.post().get_int("categoryId").unwrap_or(0);

    if !new_name.trim().is_empty() {
        MessengerDao::update_category(&new_name, category_id, player_id);
    }

    RconUtil::send_command(
        RconHeader::RefreshMessengerCategories,
        HashMap::from([("userId".to_string(), player_id.to_string())]),
    );

    let mut categories = MessengerDao::get_categories(player_id);
    categories.sort_by_key(|category| category.get_id());

    template.set("categories", TemplateValue::of(categories));
    template.render();

    Ok(())
}

/// Mirrors `createcategory(WebConnection)`.
pub fn createcategory(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut template = web_connection.template("profile/profile_widgets/friend_category_widget");
    let player_id = template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
        .map_or(0, |details| details.get_id());

    let mut new_name = web_connection.post().get_string("name").unwrap_or_default();

    if !new_name.trim().is_empty() {
        if new_name.chars().count() > 50 {
            new_name = new_name.chars().take(50).collect();
        }

        MessengerDao::add_category(&new_name, player_id);
    }

    RconUtil::send_command(
        RconHeader::RefreshMessengerCategories,
        HashMap::from([("userId".to_string(), player_id.to_string())]),
    );

    let mut categories = MessengerDao::get_categories(player_id);
    categories.sort_by_key(|category| category.get_id());

    template.set("categories", TemplateValue::of(categories));
    template.render();

    Ok(())
}

/// Mirrors `deletecategory(WebConnection)`.
pub fn deletecategory(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut template = web_connection.template("profile/profile_widgets/friend_category_widget");
    let player_id = template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
        .map_or(0, |details| details.get_id());

    let category_id = web_connection.post().get_int("categoryId").unwrap_or(0);
    MessengerDao::delete_category(category_id, player_id);

    RconUtil::send_command(
        RconHeader::RefreshMessengerCategories,
        HashMap::from([("userId".to_string(), player_id.to_string())]),
    );

    let mut categories = MessengerDao::get_categories(player_id);
    categories.sort_by_key(|category| category.get_id());

    MessengerDao::reset_friend_categories(player_id, category_id);

    template.set("categories", TemplateValue::of(categories));
    template.render();

    Ok(())
}

/// Mirrors `viewCategory(WebConnection)`.
pub fn view_category(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let page_number = if web_connection.get().contains("pageNumber") {
        web_connection.get().get_int("pageNumber").unwrap_or(1)
    } else {
        1
    };
    let mut page_size = web_connection.get().get_int("pageSize").unwrap_or(0);
    let category_id = if web_connection.get().contains("categoryId") {
        web_connection.get().get_int("categoryId").unwrap_or(-1)
    } else {
        -1
    };
    let mut search_string = if web_connection.get().contains("searchString") {
        Some(web_connection.get().get_string("searchString").unwrap_or_default())
    } else {
        None
    };

    if web_connection.get().queries().is_empty() {
        page_size = web_connection.post().get_int("pageSize").unwrap_or(0);
        search_string = if web_connection.post().contains("searchString") {
            Some(web_connection.post().get_string("searchString").unwrap_or_default())
        } else {
            None
        };
    }

    if page_size > 100 || page_size <= 0 {
        page_size = 30;
    }

    let page_number = if page_number <= 0 { 1 } else { page_number };

    let mut template = web_connection.template("profile/profile_widgets/friend_view_category");
    friendmanagement(
        &mut template,
        web_connection,
        page_size,
        page_number,
        category_id,
        search_string.as_deref(),
    );
    template.render();

    Ok(())
}

/// Mirrors `updateCategoryOptions(WebConnection)`.
pub fn update_category_options(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut template = web_connection.template("profile/profile_widgets/friend_category_options");
    let player_id = template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
        .map_or(0, |details| details.get_id());

    template.set(
        "categories",
        TemplateValue::of(MessengerDao::get_categories(player_id)),
    );
    template.render();

    Ok(())
}

/// Mirrors `movefriends(WebConnection)`.
pub fn movefriends(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut page_size = web_connection.post().get_int("pageSize").unwrap_or(0);
    let category_id = if web_connection.post().contains("moveCategoryId") {
        web_connection.post().get_int("moveCategoryId").unwrap_or(-1)
    } else {
        -1
    };

    if page_size > 100 || page_size <= 0 {
        page_size = 30;
    }

    let mut template = web_connection.template("profile/profile_widgets/friend_view_category");
    let player_id = template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
        .map_or(0, |details| details.get_id());

    if web_connection.post().contains("friendList[]") {
        for value in web_connection.post().get_array("friendList[]") {
            if let Ok(friend_id) = value.parse::<i32>() {
                MessengerDao::update_friend_category(player_id, friend_id, category_id);
            }
        }
    }

    RconUtil::send_command(
        RconHeader::RefreshMessengerCategories,
        HashMap::from([("userId".to_string(), player_id.to_string())]),
    );

    friendmanagement(&mut template, web_connection, page_size, 1, category_id, None);
    template.render();

    Ok(())
}

/// Mirrors `deletefriends(WebConnection)`.
pub fn deletefriends(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut template = web_connection.template("profile/profile_widgets/friend_view_category");
    let player_id = template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
        .map_or(0, |details| details.get_id());

    if web_connection.post().contains("friendList[]") {
        for value in web_connection.post().get_array("friendList[]") {
            if let Ok(friend_id) = value.parse::<i32>() {
                MessengerDao::remove_friend(player_id, friend_id);
                MessengerDao::remove_friend(friend_id, player_id);
            }
        }
    }

    if web_connection.post().contains("friendId") {
        if let Some(friend_id) = web_connection.post().get_int("friendId") {
            MessengerDao::remove_friend(player_id, friend_id);
            MessengerDao::remove_friend(friend_id, player_id);
        }
    }

    RconUtil::send_command(
        RconHeader::RefreshMessengerCategories,
        HashMap::from([("userId".to_string(), player_id.to_string())]),
    );

    friendmanagement(&mut template, web_connection, 30, 1, -1, None);
    template.render();

    Ok(())
}
