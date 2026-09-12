//! Mirrors `org.alexdev.http.controllers.site.CollectablesController`.

use std::collections::HashMap;

use lisbon_server::dao::mysql::currency_dao::CurrencyDao;
use lisbon_server::dao::mysql::transaction_dao::TransactionDao;
use lisbon_server::game::catalogue::collectables::collectables_manager::CollectablesManager;
use lisbon_server::game::catalogue::catalogue_manager::CatalogueManager;
use lisbon_server::game::item::item_manager::ItemManager;
use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::messages::incoming::catalogue::grpc::GRPC;
use lisbon_server::server::rcon::messages::rcon_header::RconHeader;
use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::date_util::DateUtil;

use crate::duckhttpd::TemplateValue;
use crate::duckhttpd::WebConnection;
use crate::duckhttpd::web_connection::SessionValue;
use crate::game::collectables::collectable_entry::CollectableEntry;
use crate::util::rcon_util::RconUtil;
use crate::util::xss_util::XssUtil as XSSUtil;

/// Mirrors `collectables(WebConnection)`.
pub fn collectables(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    XSSUtil::clear(web_connection);

    let page_id = GameConfiguration::get_instance().get_integer("collectables.page");
    let collectables_data =
        CollectablesManager::get_instance().get_collectable_data_by_page(page_id);

    web_connection.session().set("page", SessionValue::Str("credits".to_string()));

    let mut template = web_connection.template("collectables");
    template.set("hasCollectable", TemplateValue::of(collectables_data.is_some()));

    let mut entries: Vec<CollectableEntry> = Vec::new();

    if let Some(mut data) = collectables_data {
        data.check_cycle();

        if let Some(active_item) = data.get_active_item() {
            template.set(
                "collectableSprite",
                TemplateValue::of(
                    active_item
                        .get_definition()
                        .map(|definition| definition.get_sprite().to_string())
                        .unwrap_or_default(),
                ),
            );
            template.set(
                "collectableName",
                TemplateValue::of(
                    active_item
                        .get_definition()
                        .map(|definition| definition.get_name().to_string())
                        .unwrap_or_default(),
                ),
            );
            template.set(
                "collectableDescription",
                TemplateValue::of(
                    active_item
                        .get_definition()
                        .map(|definition| definition.get_description().to_string())
                        .unwrap_or_default(),
                ),
            );
            template.set(
                "expireSeconds",
                TemplateValue::of(
                    data.get_expiry() - DateUtil::get_current_time_seconds() as i64,
                ),
            );
        }

        for sprite in data.get_sprites() {
            if let Some(definition) = ItemManager::get_instance().get_definition_by_sprite(&sprite) {
                entries.push(CollectableEntry::new(
                    &sprite,
                    definition.get_name(),
                    definition.get_description(),
                ));
            }
        }
    }

    template.set("collectablesShowroom", TemplateValue::of(&entries));
    template.render();

    Ok(())
}

/// Mirrors `confirm(WebConnection)`.
pub fn confirm(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let page_id = GameConfiguration::get_instance().get_integer("collectables.page");
    let collectables_data =
        CollectablesManager::get_instance().get_collectable_data_by_page(page_id);

    if collectables_data.is_none() {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut template = web_connection.template("habblet/collectiblesConfirm");

    if let Some(data) = collectables_data {
        if let Some(active_item) = data.get_active_item() {
            template.set(
                "collectableName",
                TemplateValue::of(
                    active_item
                        .get_definition()
                        .map(|definition| definition.get_name().to_string())
                        .unwrap_or_default(),
                ),
            );
            template.set("collectableCost", TemplateValue::of(active_item.get_price()));
        }
    }

    template.render();

    Ok(())
}

/// Mirrors `purchase(WebConnection)`.
pub fn purchase(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean("authenticated") {
        web_connection.redirect("/");
        return Ok(());
    }

    let mut template = web_connection.template("habblet/collectiblesPurchase");

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

    if player_details.is_banned().is_some() {
        web_connection.redirect("/account/banned");
        return Ok(());
    }

    let page_id = GameConfiguration::get_instance().get_integer("collectables.page");
    let collectables_data =
        CollectablesManager::get_instance().get_collectable_data_by_page(page_id);

    if collectables_data.is_none() {
        web_connection.redirect("/");
        return Ok(());
    }

    let data = collectables_data.unwrap();

    if let Some(active_item) = data.get_active_item() {
        let price = active_item.get_price();

        if player_details.get_credits() >= price {
            if price > 0 {
                CurrencyDao::decrease_credits(&player_details, price);
            }

            template.set(
                "message",
                TemplateValue::of(format!(
                    "You've successfully bought a {}",
                    active_item
                        .get_definition()
                        .map(|definition| definition.get_name().to_string())
                        .unwrap_or_default()
                )),
            );

            let transaction_description = GRPC::get_transaction_description(&active_item);

            let catalogue_id = active_item.get_id();
            let items = CatalogueManager::get_instance().purchase(
                &player_details,
                active_item,
                Some(""),
                Some(""),
                DateUtil::get_current_time_seconds() as i64,
            );

            if let Some(description) = transaction_description {
                let item_ids = items
                    .iter()
                    .map(|item| item.get_id().to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                TransactionDao::create_transaction(
                    player_details.get_id(),
                    &item_ids,
                    &catalogue_id.to_string(),
                    1,
                    &format!("Collectable {description}"),
                    price,
                    0,
                    true,
                );
            }

            RconUtil::send_command(
                RconHeader::RefreshHand,
                HashMap::from([("userId".to_string(), player_details.get_id().to_string())]),
            );

            RconUtil::send_command(
                RconHeader::RefreshCredits,
                HashMap::from([("userId".to_string(), player_details.get_id().to_string())]),
            );
        } else {
            if price > player_details.get_credits() {
                template.set(
                    "message",
                    TemplateValue::of(
                        "Purchasing the collectable failed. You don't have enough pixels.",
                    ),
                );
            }
        }
    }

    template.render();

    Ok(())
}
