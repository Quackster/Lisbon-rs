//! Mirrors `org.alexdev.http.controllers.homes.HomesController`.

use lisbon_server::dao::mysql::badge_dao::BadgeDao;
use lisbon_server::dao::mysql::group_dao::GroupDao;
use lisbon_server::dao::mysql::messenger_dao::MessengerDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::dao::mysql::player_statistics_dao::PlayerStatisticsDao;
use lisbon_server::dao::mysql::tag_dao::TagDao;
use lisbon_server::game::badges::badge::Badge;
use lisbon_server::game::player::player_rank::PlayerRank;
use lisbon_server::game::player::statistics::player_statistic::PlayerStatistic;
use lisbon_server::util::config::game_configuration::GameConfiguration;

use crate::dao::home_edit_dao::HomeEditDao;
use crate::dao::homes_dao::HomesDao;
use crate::dao::widget_dao::WidgetDao;
use crate::duckhttpd::web_connection::SessionValue;
use crate::duckhttpd::{Settings, TemplateValue, WebConnection};
use http::StatusCode;
use crate::game::stickers::sticker_category::StickerCategory;
use crate::game::stickers::sticker_manager::StickerManager;
use crate::game::stickers::sticker_type::StickerType;
use crate::util::home_util::HomeUtil;
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

/// Mirrors `home(WebConnection)`.
pub fn home(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XssUtil::clear(web_connection);

    if !web_connection.session().contains("authenticated") {
        return Ok(());
    }

    let mut template = web_connection.template("home");
    let player_details = PlayerDao::get_details(web_connection.session().get_int("user.id"));

    let mut username = None;

    if let Some(first_match) = web_connection.get_matches().first() {
        username = Some(first_match.clone());
    } else {
        if web_connection.get().contains("tag") {
            username = web_connection.get().get_string("tag");
        }
    }

    if web_connection.get_route_request().ends_with("/id") {
        if let Some(username_value) = &username {
            if is_numeric(username_value) {
                username = PlayerDao::get_name(username_value.parse().unwrap_or(0));
            }
        }
    }

    let Some(username) = username else {
        not_found(web_connection);
        return Ok(());
    };

    let Some(user) = PlayerDao::get_details_by_name(&username) else {
        not_found(web_connection);
        return Ok(());
    };

    let is_moderator = player_details
        .map(|player| player.get_rank().map(|rank| rank.rank_id()).unwrap_or(0))
        .unwrap_or(0)
        >= PlayerRank::Moderator.rank_id();

    if (!is_moderator) && (!user.is_profile_visible() || user.is_banned().is_some()) {
        not_found(web_connection);
        return Ok(());
    }

    let mut home = HomesDao::get_home(user.get_id());
    let mut defult_widgets = false;

    if home.is_none() {
        home = Some(crate::game::homes::home::Home::new(user.get_id(), "bg_pattern_abstract2"));
        defult_widgets = true;
    }

    let mut can_add_friend = false;
    let mut user_id: i32 = -1;
    let mut session_time: i64 = -1;

    if web_connection.session().get_boolean("authenticated") {
        user_id = web_connection.session().get_int("user.id");
        session_time = HomeEditDao::get_session(web_connection.session().get_int("user.id"));

        if session_time != -1 && user_id == user.get_id() {
            web_connection.session().delete("groupEditSession");
            // Java stores the `PlayerDetails` object; `SessionValue` only
            // supports `Str` / `Bool` / `Int`, so the user id is stored.
            web_connection.session().set("homeEditSession", SessionValue::Int(user_id));
        }

        if user_id != user.get_id() && !MessengerDao::friend_exists(user_id, user.get_id()) {
            can_add_friend = true;
        }
    }

    let widgets = if defult_widgets {
        StickerManager::get_instance().get_default_widgets(user.get_id())
    } else {
        WidgetDao::get_home_widgets_by_placement(user.get_id(), true)
    };

    let mut enabled_badges: Vec<Badge> = BadgeDao::get_badges(user.get_id())
        .into_iter()
        .filter(|badge| badge.is_equipped())
        .collect();
    enabled_badges.sort_by_key(|badge| badge.get_slot_id());

    let home_widgets = WidgetDao::get_home_widgets(user.get_id());
    let guestbook = home_widgets.iter().find(|widget| {
        widget
            .get_product()
            .map(|product| product.data.eq_ignore_ascii_case("guestbookwidget"))
            .unwrap_or(false)
    });

    web_connection.session().set("page", SessionValue::Str("me".into()));

    template.set("user", TemplateValue::of(user.clone()));
    template.set("tags", TemplateValue::of(TagDao::get_user_tags(user.get_id())));
    template.set("hasBadge", TemplateValue::of(!enabled_badges.is_empty()));
    template.set("editMode", TemplateValue::of(session_time != -1 && user_id == user.get_id()));
    template.set(
        "stickers",
        TemplateValue::json(
            serde_json::Value::Array(
            widgets
                .iter()
                .map(serde_json::to_value)
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_default(),
            )
        ),
    );
    template.set(
        "homeBannerAd",
        TemplateValue::of(HomeUtil::get_random_ad().unwrap_or_default()),
    );
    if let Some(home_ref) = &home {
        template.set("home", TemplateValue::of(home_ref.clone()));
    }
    template.set("canAddFriend", TemplateValue::of(can_add_friend));
    template.set(
        "guestbookSetting",
        TemplateValue::of(
            guestbook
                .map(|widget| widget.get_guestbook_state())
                .unwrap_or_else(|| "public".to_string()),
        ),
    );
    template.set("stickerLimit", TemplateValue::of(HomeUtil::get_sticker_limit(user.has_club_subscription())));
    template.set("tagCloud", TemplateValue::of(Vec::<String>::new()));

    if !enabled_badges.is_empty() {
        template.set(
            "badgeCode",
            TemplateValue::of(enabled_badges[0].get_badge_code().to_string()),
        );
    }

    template.set("hasFavouriteGroup", TemplateValue::of(false));

    if user.get_favourite_group_id() > 0 {
        if let Some(group) = GroupDao::get_group(user.get_favourite_group_id()) {
            template.set("hasFavouriteGroup", TemplateValue::of(true));
            template.set("group", TemplateValue::of(group));
        }
    }

    if web_connection.session().get_boolean("authenticated") {
        if user.get_id() == user_id {
            PlayerStatisticsDao::update_statistic(
                user_id,
                PlayerStatistic::GuestbookUnreadMessages,
                "0",
            );
        }
    }

    template.render_html().ok();
    Ok(())
}

/// Mirrors `inventory(WebConnection)`.
pub fn inventory(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let Some(player_details) = PlayerDao::get_details(user_id) else {
        web_connection.session().delete("user.id");
        web_connection.session().delete("authenticated");
        web_connection.redirect("/");
        return Ok(());
    };

    let rank_id = player_details.get_rank().map(|rank| rank.rank_id()).unwrap_or(0);
    let categories = StickerManager::get_instance().get_categories(rank_id);
    let mut sticker_categories: Vec<_> = categories
        .iter()
        .filter(|category| category.category_type == StickerCategory::STICKER_BACKGROUND_TYPE)
        .collect();
    let mut background_categories: Vec<_> = categories
        .iter()
        .filter(|category| category.category_type == StickerCategory::BACKGROUND_CATEGORY_TYPE)
        .collect();
    sticker_categories.sort_by(|a, b| a.name.cmp(&b.name));
    background_categories.sort_by(|a, b| a.name.cmp(&b.name));

    let widget_list = WidgetDao::get_inventory_widgets(player_details.get_id());
    let mut inventory_widgets: Vec<crate::game::homes::widget::Widget> = Vec::new();

    for widget in widget_list.iter() {
        if let Some(index) = inventory_widgets.iter().position(|w| w.sticker_id == widget.sticker_id) {
            let current_amount = inventory_widgets[index].amount;
            inventory_widgets[index].set_amount(current_amount + 1);
        } else {
            inventory_widgets.push(widget.clone());
        }
    }

    inventory_widgets.sort_by(|a, b| b.id.cmp(&a.id));

    let empty_boxes = if widget_list.len() > 20 {
        ((widget_list.len() as f64 / 4.0).ceil() as i32) * 4
    } else {
        20 - widget_list.len() as i32
    };

    let empty_box: Vec<serde_json::Value> = (0..empty_boxes.max(0)).map(|_| serde_json::Value::Null).collect();

    if let Some(first_widget) = widget_list.first() {
        let product = first_widget.get_product();
        let x_json = format!(
            "[[\"Inventory\",\"Web Store\"],[\"{css_class}\",{data}\",{name}\",\"Stickers\",null,1]]",
            css_class = product.as_ref().and_then(|p| p.get_css_class()).unwrap_or_default(),
            data = product.as_ref().map(|p| p.data.clone()).unwrap_or_default(),
            name = product.as_ref().map(|p| p.name.clone()).unwrap_or_default(),
        );
        web_connection.set_header("X-JSON", &x_json);
    } else {
        web_connection.set_header("X-JSON", "[[\"Inventory\",\"Web Store\"],[\"\",\"\",\"\",\"Stickers\",null,1]]");
    }

    let mut tpl = web_connection.template("homes/inventory/inventory");
    tpl.set("stickerCategories", TemplateValue::of(sticker_categories));
    tpl.set("backgroundCategories", TemplateValue::of(background_categories));
    tpl.set("emptyBoxes", TemplateValue::json(serde_json::Value::Array(empty_box)));
    tpl.set(
        "widgets",
        TemplateValue::json(
            serde_json::Value::Array(
            inventory_widgets
                .iter()
                .map(serde_json::to_value)
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_default(),
        )
        ),
    );
    tpl.render_html().ok();
    Ok(())
}

/// Mirrors `inventoryItems(WebConnection)`.
pub fn inventory_items(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let type_id = {
        let type_name = web_connection.post().get_string("type").unwrap_or_default();

        if type_name.eq_ignore_ascii_case("stickers") {
            1
        } else if type_name.eq_ignore_ascii_case("backgrounds") {
            4
        } else if type_name.eq_ignore_ascii_case("notes") {
            3
        } else {
            0
        }
    };

    let is_widgets = web_connection
        .post()
        .get_string("type")
        .map(|type_name| type_name.eq_ignore_ascii_case("widgets"))
        .unwrap_or(false);

    if !is_widgets {
        let widget_list = WidgetDao::get_inventory_widgets_by_type(
            web_connection.session().get_int("user.id"),
            type_id,
        );
        let mut inventory_widgets: Vec<crate::game::homes::widget::Widget> = Vec::new();

        for widget in widget_list.iter() {
            if let Some(index) = inventory_widgets.iter().position(|w| w.sticker_id == widget.sticker_id) {
                let current_amount = inventory_widgets[index].amount;
                inventory_widgets[index].set_amount(current_amount + 1);
            } else {
                inventory_widgets.push(widget.clone());
            }
        }

        inventory_widgets.sort_by(|a, b| b.id.cmp(&a.id));

        let empty_boxes = if widget_list.len() > 20 {
            ((widget_list.len() as f64 / 4.0).ceil() as i32) * 4
        } else {
            20 - widget_list.len() as i32
        };

        let empty_box: Vec<serde_json::Value> = (0..empty_boxes.max(0)).map(|_| serde_json::Value::Null).collect();

        let mut tpl = web_connection.template("homes/inventory/inventory_items");
        tpl.set("emptyBoxes", TemplateValue::json(serde_json::Value::Array(empty_box)));
        tpl.set(
            "widgets",
            TemplateValue::json(
                serde_json::Value::Array(
            inventory_widgets
                .iter()
                .map(serde_json::to_value)
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_default(),
        )
            ),
        );
        tpl.set("widgetMode", TemplateValue::of(false));
        tpl.render_html().ok();
    } else {
        let widget_list: Vec<crate::game::homes::widget::Widget> =
            if web_connection.session().contains("groupEditSession") {
                let group_id = web_connection.session().get_int("groupEditSession");
                WidgetDao::get_group_widgets(group_id)
                    .into_iter()
                    .filter(|widget| {
                        widget
                            .get_product()
                            .and_then(|product| product.get_type())
                            .map(|type_| type_ == StickerType::GroupWidget)
                            .unwrap_or(false)
                    })
                    .collect()
            } else {
                let user_id = web_connection.session().get_int("user.id");
                WidgetDao::get_home_widgets(user_id)
                    .into_iter()
                    .filter(|widget| {
                        widget
                            .get_product()
                            .and_then(|product| product.get_type())
                            .map(|type_| type_ == StickerType::HomeWidget)
                            .unwrap_or(false)
                    })
                    .filter(|widget| {
                        !widget
                            .get_product()
                            .map(|product| product.data.eq_ignore_ascii_case("profilewidget"))
                            .unwrap_or(false)
                    })
                    .collect()
            };

        let mut tpl = web_connection.template("homes/inventory/inventory_items");
        tpl.set("widgetMode", TemplateValue::of(true));
        tpl.set(
            "widgets",
            TemplateValue::json(
                serde_json::Value::Array(
            widget_list
                .iter()
                .map(serde_json::to_value)
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_default(),
        )
            ),
        );
        tpl.render_html().ok();
    }
    Ok(())
}

/// Mirrors `inventoryPreview(WebConnection)`.
pub fn inventory_preview(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let item_id = web_connection.post().get_int("itemId").unwrap_or(0);
    let type_name = web_connection.post().get_string("type").unwrap_or_default();
    let mut type_id = 0;

    if type_name.eq_ignore_ascii_case("stickers") {
        type_id = 1;
    }

    if type_name.eq_ignore_ascii_case("backgrounds") {
        type_id = 4;
    }

    if type_name.eq_ignore_ascii_case("notes") {
        type_id = 3;
    }

    if type_name.eq_ignore_ascii_case("widgets") {
        type_id = if web_connection.session().contains("groupEditSession") {
            StickerType::GroupWidget.type_id()
        } else {
            StickerType::HomeWidget.type_id()
        };
    }

    let widget = if type_id == StickerType::GroupWidget.type_id() {
        let group_id = web_connection.session().get_int("groupEditSession");
        WidgetDao::get_group_widgets(group_id)
            .into_iter()
            .find(|w| w.id == item_id)
    } else if type_id == StickerType::HomeWidget.type_id() {
        let user_id = web_connection.session().get_int("user.id");
        WidgetDao::get_home_widgets(user_id)
            .into_iter()
            .find(|w| w.id == item_id)
    } else {
        WidgetDao::get_inventory_widgets_by_type(
            web_connection.session().get_int("user.id"),
            type_id,
        )
        .into_iter()
        .find(|w| w.id == item_id)
    };

    if let Some(widget) = &widget {
        if type_id == 1 {
            let product = widget.get_product();
            let x_json = format!(
                "[\"{css_class}\",{data}\",{name}\",\"Sticker\",null,1]",
                css_class = product.as_ref().and_then(|p| p.get_css_class()).unwrap_or_default(),
                data = product.as_ref().map(|p| p.data.clone()).unwrap_or_default(),
                name = product.as_ref().map(|p| p.name.clone()).unwrap_or_default(),
            );
            web_connection.set_header("X-JSON", &x_json);
        } else if type_id == 4 {
            let product = widget.get_product();
            let x_json = format!(
                "[\"{css_class}\",\"b_{data}\",{name}\",\"Background\",null,1]",
                css_class = product.as_ref().and_then(|p| p.get_css_class()).unwrap_or_default(),
                data = product.as_ref().map(|p| p.data.clone()).unwrap_or_default(),
                name = product.as_ref().map(|p| p.name.clone()).unwrap_or_default(),
            );
            web_connection.set_header("X-JSON", &x_json);
        } else if type_id == 3 {
            web_connection.set_header("X-JSON", "[\"commodity_stickienote_pre\",null,\"Notes\",\"WebCommodity\",null,1]");
        } else if type_id == StickerType::GroupWidget.type_id()
            || type_id == StickerType::HomeWidget.type_id()
        {
            let product = widget.get_product();
            let x_json = format!(
                "[\"{css_class}\",null,\"\",\"Widget\",\"true\",1]",
                css_class = product.as_ref().and_then(|p| p.get_css_class()).unwrap_or_default(),
            );
            web_connection.set_header("X-JSON", &x_json);
        } else {
            web_connection.set_header("X-JSON", "[\"\",\"\",\"\",\"Sticker\",null,1]");
        }
    } else {
        web_connection.set_header("X-JSON", "[\"\",\"\",\"\",\"Sticker\",null,1]");
    }

    let mut tpl = web_connection.template("homes/inventory/inventory_preview");
    tpl.render_html().ok();
    Ok(())
}

/// Mirrors `startEditingSession(WebConnection)`.
pub fn start_editing_session(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let matches = web_connection.get_matches();
    let Some(match0) = matches.first() else {
        return Ok(());
    };

    if !is_numeric(match0) {
        web_connection.redirect("/me");
        return Ok(());
    }

    let target_id = match0.parse().unwrap_or(0);
    let user_id = web_connection.session().get_int("user.id");

    if target_id != user_id {
        web_connection.redirect("/me");
        return Ok(());
    }

    if HomesDao::get_home(target_id).is_none() {
        StickerManager::get_instance().create_home(target_id);
    }

    if !HomeEditDao::has_session(user_id) {
        HomeEditDao::create_session(user_id);
        web_connection.session().set("homeEditSession", SessionValue::Int(user_id));
        web_connection.session().delete("groupEditSession");
    }

    let Some(player_details) = PlayerDao::get_details(user_id) else {
        return Ok(());
    };

    web_connection.redirect(&format!("/home/{}", player_details.get_name()));
    Ok(())
}

/// Mirrors `cancelEditingSession(WebConnection)`.
pub fn cancel_editing_session(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let matches = web_connection.get_matches();
    let Some(match0) = matches.first() else {
        return Ok(());
    };

    if !is_numeric(match0) {
        web_connection.redirect("/me");
        return Ok(());
    }

    let target_id = match0.parse().unwrap_or(0);
    let user_id = web_connection.session().get_int("user.id");

    if target_id != user_id {
        web_connection.redirect("/me");
        return Ok(());
    }

    if HomeEditDao::has_session(user_id) {
        HomeEditDao::delete(user_id);
        web_connection.session().delete("homeEditSession");
        web_connection.session().delete("groupEditSession");
    }

    let Some(player_details) = PlayerDao::get_details(user_id) else {
        return Ok(());
    };

    web_connection.redirect(&format!("/home/{}", player_details.get_name()));
    Ok(())
}

/// Mirrors `save(WebConnection)`.
pub fn save(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let Some(player_details) = PlayerDao::get_details(user_id) else {
        web_connection.session().delete("user.id");
        web_connection.session().delete("authenticated");
        web_connection.redirect("/");
        return Ok(());
    };

    if !HomeEditDao::has_session(user_id) {
        web_connection.send_string("");
        return Ok(());
    }

    let mut home = HomesDao::get_home(user_id);
    let mut home_widgets = WidgetDao::get_home_widgets(user_id);

    if let Some(background) = web_connection.post().get_string("background") {
        if let Ok(background_id) = background.split(':').next().unwrap_or("").parse::<i32>() {
            let widget_list = WidgetDao::get_inventory_widgets(player_details.get_id());
            let widget = widget_list.iter().find(|w| w.id == background_id);

            if let Some(widget) = widget {
                if let Some(home) = home.as_mut() {
                    let background_data = widget.get_product().map(|p| p.data.clone()).unwrap_or_default();
                    home.set_background(&background_data);
                    home.save_background();
                }
            }
        }
    }

    let mut move_widgets = |sticker_key: &str| {
        let Some(stickers) = web_connection.post().get_string(sticker_key) else {
            return;
        };

        for sticker in stickers.split('/') {
            if let Ok(sticker_id) = sticker.split(':').next().unwrap_or("").parse::<i32>() {
                let stripped = sticker.replace(format!("{sticker_id}:").as_str(), "");
                let coord_data = stripped.split(',').collect::<Vec<_>>();

                if let (Some(x), Some(y), Some(z)) = (
                    coord_data.first().and_then(|value| value.parse::<i32>().ok()),
                    coord_data.get(1).and_then(|value| value.parse::<i32>().ok()),
                    coord_data.get(2).and_then(|value| value.parse::<i32>().ok()),
                ) {
                    if let Some(index) = home_widgets.iter().position(|w| w.id == sticker_id) {
                        home_widgets[index].set_x(x);
                        home_widgets[index].set_y(y);
                        home_widgets[index].set_z(z);
                        home_widgets[index].save();
                    }
                }
            }
        }
    };

    // Mirrors the Java `try` block; each section is individually guarded.
    if web_connection.post().contains("stickers") {
        let sticker_limit = HomeUtil::get_sticker_limit(player_details.has_club_subscription());

        if let Some(stickers) = web_connection.post().get_string("stickers") {
            if stickers.split('/').count() as i32 >= sticker_limit {
                web_connection.send_string("");
                return Ok(());
            }
        }

        move_widgets("stickers");
    }

    if web_connection.post().contains("widgets") {
        move_widgets("widgets");
    }

    if web_connection.post().contains("stickienotes") {
        move_widgets("stickienotes");
    }

    web_connection.send_string(format!(
        "<script language=\"JavaScript\" type=\"text/javascript\">\nwaitAndGo('{path}/home/{name}');\n</script>\n",
        path = GameConfiguration::get_instance().get_string("site.path"),
        name = player_details.get_name(),
    ));

    HomeEditDao::delete(user_id);
    web_connection.session().delete("homeEditSession");
    Ok(())
}

/// Mirrors `tagList(WebConnection)`.
pub fn tag_list(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = web_connection.session().get_int("user.id");

    if user_id < 1 {
        web_connection.send_string("");
        return Ok(());
    }

    let account_id = web_connection.post().get_int("accountId").unwrap_or(0);

    let mut template = web_connection.template("homes/widget/habblet/taglist");
    let Some(player_details) = PlayerDao::get_details(user_id) else {
        web_connection.send_string("");
        return Ok(());
    };

    let tags = TagDao::get_user_tags(account_id);

    template.set("tags", TemplateValue::of(tags));
    template.set("user", TemplateValue::of(player_details));
    template.render_html().ok();
    Ok(())
}
