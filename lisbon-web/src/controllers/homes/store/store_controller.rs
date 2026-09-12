//! Mirrors `org.alexdev.http.controllers.homes.store.StoreController`.

use std::collections::HashMap;

use lisbon_server::dao::mysql::currency_dao::CurrencyDao;
use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::server::rcon::messages::rcon_header::RconHeader;

use crate::dao::widget_dao::WidgetDao;
use crate::duckhttpd::{Settings, TemplateValue, WebConnection};
use http::StatusCode;
use crate::game::stickers::sticker_category::StickerCategory;
use crate::game::stickers::sticker_manager::StickerManager;
use crate::game::stickers::sticker_type::StickerType;
use crate::util::rcon_util::RconUtil;

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

/// Mirrors `main(WebConnection)`.
pub fn main(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
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

    let mut sticker_category = -1;
    let mut products: Vec<crate::game::stickers::sticker_product::StickerProduct> = Vec::new();

    if let Some(first) = sticker_categories.first() {
        sticker_category = first.id;
    } else if let Some(first) = background_categories.first() {
        sticker_category = first.id;
    }

    if sticker_category != -1 {
        products = StickerManager::get_instance()
            .get_catalogue_list()
            .into_iter()
            .filter(|product| product.category_id == sticker_category)
            .collect();
    }

    let empty_boxes = if products.len() > 20 {
        ((products.len() as f64 / 5.0).ceil() as i32) * 5
    } else {
        20 - products.len() as i32
    };

    if let Some(product) = products.first() {
        web_connection.set_header(
            "X-JSON",
            &format!(
                "[[\"Inventory\",\"Web Store\"],[{{\"itemCount\":{amount},\"previewCssClass\":\"{css_class}\",\"titleKey\":\"\"}}]]",
                amount = product.amount,
                css_class = product.get_css_class().unwrap_or_default(),
            ),
        );
    } else {
        web_connection.set_header("X-JSON", "[[\"Inventory\",\"Web Store\"],[{\"itemCount\":0,\"titleKey\":\"\"}]]");
    }

    let empty_box: Vec<serde_json::Value> =
        (0..empty_boxes.max(0)).map(|_| serde_json::Value::Null).collect();

    let mut tpl = web_connection.template("homes/store/main");
    tpl.set("stickerCategories", TemplateValue::of(sticker_categories));
    tpl.set("backgroundCategories", TemplateValue::of(background_categories));
    tpl.set("products", TemplateValue::of(&products));
    if let Some(product) = products.first() {
        tpl.set("product", TemplateValue::of(product));
    }
    tpl.set("emptyBoxes", TemplateValue::json(serde_json::Value::Array(empty_box)));
    tpl.render_html().ok();
    Ok(())
}

/// Mirrors `items(WebConnection)`.
pub fn items(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let Some(_player_details) = PlayerDao::get_details(user_id) else {
        web_connection.session().delete("user.id");
        web_connection.session().delete("authenticated");
        web_connection.redirect("/");
        return Ok(());
    };

    let sub_category = web_connection.post().get_int("subCategoryId").unwrap_or(0);
    let Some(category) = StickerManager::get_instance().get_category(sub_category) else {
        not_found(web_connection);
        return Ok(());
    };

    let products: Vec<_> = StickerManager::get_instance()
        .get_catalogue_list()
        .into_iter()
        .filter(|product| product.category_id == category.id)
        .collect();

    let empty_boxes = if products.len() > 20 {
        ((products.len() as f64 / 5.0).ceil() as i32) * 5
    } else {
        20 - products.len() as i32
    };

    let mut tpl = web_connection.template("homes/store/items");
    tpl.set("products", TemplateValue::of(&products));
    tpl.set("emptyProducts", TemplateValue::of(empty_boxes));
    tpl.render_html().ok();
    Ok(())
}

/// Mirrors `preview(WebConnection)`.
pub fn preview(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let product_id = web_connection.post().get_int("productId").unwrap_or(0);
    let sticker_product = StickerManager::get_instance()
        .get_catalogue_list()
        .into_iter()
        .find(|product| product.id == product_id);

    let Some(sticker_product) = sticker_product else {
        not_found(web_connection);
        return Ok(());
    };

    let product_type = sticker_product.get_type();

    if product_type == Some(StickerType::Sticker) || product_type == Some(StickerType::Note) {
        web_connection.set_header(
            "X-JSON",
            &format!(
                "[{{\"itemCount\":{amount},\"previewCssClass\":\"{css_class}\",\"titleKey\":\"{name}\"}}]",
                amount = sticker_product.amount,
                css_class = sticker_product.get_css_class().unwrap_or_default(),
                name = sticker_product.name,
            ),
        );
    } else if product_type == Some(StickerType::Background) {
        web_connection.set_header(
            "X-JSON",
            &format!(
                "[{{\"bgCssClass\":\"b_{data}\",\"itemCount\":{amount},\"previewCssClass\":\"{css_class}\",\"titleKey\":\"{name}\"}}]",
                data = sticker_product.data,
                amount = sticker_product.amount,
                css_class = sticker_product.get_css_class().unwrap_or_default(),
                name = sticker_product.name,
            ),
        );
    }

    let mut tpl = web_connection.template("homes/store/preview");
    tpl.set("product", TemplateValue::of(sticker_product));
    tpl.render_html().ok();
    Ok(())
}

/// Mirrors `purchaseConfirm(WebConnection)`.
pub fn purchase_confirm(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let product_id = web_connection.post().get_int("productId").unwrap_or(0);
    let Some(sticker_product) = StickerManager::get_instance()
        .get_catalogue_list()
        .into_iter()
        .find(|product| product.id == product_id)
    else {
        return Ok(());
    };

    let mut tpl = web_connection.template("homes/store/purchase_confirm");
    tpl.set("product", TemplateValue::of(sticker_product.clone()));

    let player_details = PlayerDao::get_details(web_connection.session().get_int("user.id"));

    if player_details.map(|details| details.get_credits()).unwrap_or(0) < sticker_product.price {
        tpl.set("noCredits", TemplateValue::of(true));
    } else {
        tpl.set("noCredits", TemplateValue::of(false));
    }

    tpl.render_html().ok();
    Ok(())
}

/// Mirrors `backgroundWarning(WebConnection)`.
pub fn background_warning(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut tpl = web_connection.template("homes/store/background_warning");
    tpl.render_html().ok();
    Ok(())
}

/// Mirrors `purchaseBackgrounds(WebConnection)`.
pub fn purchase_backgrounds(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let widget_id = web_connection.post().get_int("selectedId").unwrap_or(0);

    let Some(sticker_product) = StickerManager::get_instance()
        .get_catalogue_list()
        .into_iter()
        .find(|product| product.id == widget_id)
    else {
        not_found(web_connection);
        return Ok(());
    };

    if sticker_product.get_type() != Some(StickerType::Background) {
        not_found(web_connection);
        return Ok(());
    }

    let Some(player_details) = PlayerDao::get_details(user_id) else {
        // Java NPEs on a null player.
        return Ok(());
    };

    if player_details.get_credits() < sticker_product.price {
        web_connection.send_string("");
        return Ok(());
    }

    for _ in 0..sticker_product.amount {
        WidgetDao::purchase_widget(user_id, 0, 0, 0, 0, sticker_product.id, "", 0, false);
    }

    CurrencyDao::decrease_credits(&player_details, sticker_product.price);

    RconUtil::send_command(
        RconHeader::RefreshCredits,
        HashMap::from([("userId".to_string(), player_details.get_id().to_string())]),
    );

    web_connection.send_string("OK");
    Ok(())
}

/// Mirrors `purchaseStickers(WebConnection)`.
pub fn purchase_stickers(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let widget_id = web_connection.post().get_int("selectedId").unwrap_or(0);

    let Some(sticker_product) = StickerManager::get_instance()
        .get_catalogue_list()
        .into_iter()
        .find(|product| product.id == widget_id)
    else {
        not_found(web_connection);
        return Ok(());
    };

    if sticker_product.get_type() != Some(StickerType::Sticker) {
        not_found(web_connection);
        return Ok(());
    }

    let Some(player_details) = PlayerDao::get_details(user_id) else {
        // Java NPEs on a null player.
        return Ok(());
    };

    if player_details.get_credits() < sticker_product.price {
        web_connection.send_string("");
        return Ok(());
    }

    for _ in 0..sticker_product.amount {
        WidgetDao::purchase_widget(user_id, 0, 0, 0, 0, sticker_product.id, "", 0, false);
    }

    CurrencyDao::decrease_credits(&player_details, sticker_product.price);

    RconUtil::send_command(
        RconHeader::RefreshCredits,
        HashMap::from([("userId".to_string(), player_details.get_id().to_string())]),
    );

    web_connection.send_string("OK");
    Ok(())
}

/// Mirrors `purchaseStickieNotes(WebConnection)`.
pub fn purchase_stickie_notes(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let user_id = web_connection.session().get_int("user.id");
    let widget_id = web_connection.post().get_int("selectedId").unwrap_or(0);

    let Some(sticker_product) = StickerManager::get_instance()
        .get_catalogue_list()
        .into_iter()
        .find(|product| product.id == widget_id)
    else {
        not_found(web_connection);
        return Ok(());
    };

    if sticker_product.get_type() != Some(StickerType::Note) {
        not_found(web_connection);
        return Ok(());
    }

    let Some(player_details) = PlayerDao::get_details(user_id) else {
        // Java NPEs on a null player.
        return Ok(());
    };

    if player_details.get_credits() < sticker_product.price {
        web_connection.send_string("");
        return Ok(());
    }

    for _ in 0..sticker_product.amount {
        WidgetDao::purchase_widget(user_id, 0, 0, 0, 0, sticker_product.id, "", 0, false);
    }

    CurrencyDao::decrease_credits(&player_details, sticker_product.price);

    RconUtil::send_command(
        RconHeader::RefreshCredits,
        HashMap::from([("userId".to_string(), player_details.get_id().to_string())]),
    );

    web_connection.send_string("OK");
    Ok(())
}
