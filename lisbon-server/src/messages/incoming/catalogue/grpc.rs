//! Mirrors `net.h4bbo.lisbon.messages.incoming.catalogue.GRPC`.
use crate::dao::mysql::currency_dao::CurrencyDao;
use crate::dao::mysql::transaction_dao::TransactionDao;
use crate::game::catalogue::catalogue_item::CatalogueItem;
use crate::game::catalogue::catalogue_manager::CatalogueManager;
use crate::game::catalogue::collectables::collectables_manager::CollectablesManager;
use crate::game::catalogue::rare_manager::RareManager;
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::item::base::item_definition::ItemDefinition;
use crate::game::item::item::Item;
use crate::game::item::item_manager::ItemManager;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::game::texts::texts_manager::TextsManager;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::messages::outgoing::alert::no_user_found::NO_USER_FOUND;
use crate::messages::outgoing::catalogue::no_credits::NO_CREDITS;
use crate::messages::outgoing::rooms::items::item_delivered::ITEM_DELIVERED;
use crate::messages::outgoing::user::currencies::credit_balance::CREDIT_BALANCE;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::config::game_configuration::GameConfiguration;
use crate::util::date_util::DateUtil;
use crate::util::string_util::StringUtil;

#[allow(non_camel_case_types)]
pub struct GRPC;

impl GRPC {
    /// Mirrors `getTransactionDescription(CatalogueItem)`.
    pub fn get_transaction_description(item: &CatalogueItem) -> Option<String> {
        if !item.is_package() {
            return Self::get_item_description(item.get_definition().as_ref());
        }

        let mut descriptions = Vec::new();

        for catalogue_package in item.get_packages() {
            let Some(description) =
                Self::get_item_description(catalogue_package.get_definition().as_ref())
            else {
                continue;
            };
            descriptions.push(description);
        }

        Some(format!("Package purchase ({})", descriptions.join(", ")))
    }

    /// Mirrors `getItemDescription(ItemDefinition, int)`.
    fn get_item_description(definition: Option<&ItemDefinition>) -> Option<String> {
        let Some(definition) = definition else {
            return None;
        };

        if definition.get_sprite() == "film" {
            return Some("Film purchase".to_string());
        }

        Some(format!("{} purchase", definition.get_name()))
    }
}

impl MessageEvent for GRPC {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(content) = reader.contents() else {
            return Ok(());
        };
        let data: Vec<&str> = content.split('\r').collect();
        let Some(sale_code) = data.get(3).copied() else {
            return Ok(());
        };

        let Some(item) = CatalogueManager::get_instance().get_catalogue_item(sale_code) else {
            return Ok(());
        };

        let mut price = item.get_price();

        // If the item is not a buyable special rare, then check if they can actually buy it
        let current_rare = RareManager::get_instance().get_current_rare();

        if let Some(rare) = &current_rare {
            if item.get_id() != rare.get_id() {
                let Some(page) = CatalogueManager::get_instance()
                    .get_catalogue_pages()
                    .into_iter()
                    .find(|p| item.has_page(p.get_id()))
                else {
                    return Ok(());
                };

                let rank_id = player
                    .get_details()
                    .get_rank()
                    .map(|rank| rank.rank_id())
                    .unwrap_or(i32::MIN);

                if page.get_min_role().rank_id() > rank_id {
                    return Ok(());
                }
            }
        }

        if let Some(rare) = &current_rare {
            if rare.get_id() == item.get_id() && !player.has_fuse(&Fuseright::Credits) {
                if let Some(rare_cost) =
                    RareManager::get_instance().get_rare_cost().get(&rare.get_id())
                {
                    price = *rare_cost;
                }
            }
        }

        if price > player.get_details().get_credits() {
            player.send(&NO_CREDITS);
            return Ok(());
        }

        let Some(is_gift) = data.get(5).copied() else {
            return Ok(());
        };

        if is_gift == "1" {
            // It's a gift!
            let Some(receiver_name) = data.get(6).copied() else {
                return Ok(());
            };

            let Some(receiving_user_details) =
                PlayerManager::get_instance().get_player_data_by_name(receiver_name)
            else {
                player.send(&NO_USER_FOUND::new(receiver_name));
                return Ok(());
            };

            let mut present_note = String::new();
            let Some(extra_data_raw) = data.get(4).copied() else {
                return Ok(());
            };
            let mut extra_data = extra_data_raw.to_string();

            if let Some(note) = data.get(7).copied() {
                present_note = note.to_string();
            }

            if present_note.is_empty() {
                present_note = " ".to_string();
            }

            extra_data = extra_data.replace(Item::PRESENT_DELIMETER, "");
            present_note = present_note.replace(Item::PRESENT_DELIMETER, "");

            if item
                .get_definition()
                .map(|definition| definition.get_sprite() == "poster")
                .unwrap_or(false)
            {
                extra_data = item.get_item_special_id().to_string();
            }

            let present = ItemManager::get_instance().create_gift(
                receiving_user_details.get_id(),
                player.get_details().get_name(),
                item.get_sale_code(),
                &StringUtil::filter_input(&present_note, false),
                &extra_data,
            );

            if let Some(transaction_description) = Self::get_transaction_description(&item) {
                let description = format!(
                    "Gift purchase from {} for {} - {}",
                    player.get_details().get_name(),
                    receiving_user_details.get_name(),
                    transaction_description
                );

                TransactionDao::create_transaction(
                    receiving_user_details.get_id(),
                    &present.get_id().to_string(),
                    &item.get_id().to_string(),
                    1,
                    &description,
                    item.get_price(),
                    0,
                    true,
                );

                TransactionDao::create_transaction(
                    player.get_details().get_id(),
                    &present.get_id().to_string(),
                    &item.get_id().to_string(),
                    1,
                    &description,
                    item.get_price(),
                    0,
                    true,
                );
            }

            if let Some(receiver) =
                PlayerManager::get_instance().get_player_by_id(receiving_user_details.get_id())
            {
                let receiver = receiver.lock();
                if let Some(inventory) = receiver.get_inventory() {
                    inventory.add_item(&present);
                    inventory.view(&receiver, "last");
                }
            }

            let alert_text = TextsManager::get_instance()
                .get_value("successfully_purchase_gift_for")
                .replace("%user%", receiving_user_details.get_name());
            player.send(&ALERT::new(&alert_text));
        } else {
            let extra_data: Option<String> = if !item.is_package() {
                data.get(4).copied().map(|value| value.to_string())
            } else {
                None
            };

            let items = CatalogueManager::get_instance().purchase(
                player.get_details(),
                item.clone(),
                extra_data.as_deref(),
                None,
                DateUtil::get_current_time_seconds() as i64,
            );

            if !items.is_empty() {
                if let Some(inventory) = player.get_inventory() {
                    inventory.view(player, "new");
                }
            }

            let is_collectable = CollectablesManager::get_instance().is_collectable(&item);

            let item_ids: String = items
                .iter()
                .map(|item| item.get_id().to_string())
                .collect::<Vec<_>>()
                .join(",");

            if let Some(transaction_description) = Self::get_transaction_description(&item) {
                if is_collectable {
                    TransactionDao::create_transaction(
                        player.get_details().get_id(),
                        &item_ids,
                        &item.get_id().to_string(),
                        1,
                        &format!("Collectible - {transaction_description}"),
                        item.get_price(),
                        0,
                        true,
                    );
                } else {
                    TransactionDao::create_transaction(
                        player.get_details().get_id(),
                        &item_ids,
                        &item.get_id().to_string(),
                        1,
                        &transaction_description,
                        item.get_price(),
                        0,
                        true,
                    );
                }
            }

            let show_item_delivered = !item
                .get_definition()
                .map(|definition| definition.get_sprite() == "film")
                .unwrap_or(false);

            if show_item_delivered {
                if !GameConfiguration::get_instance().get_bool("disable.purchase.successful.alert") {
                    player.send(&ITEM_DELIVERED);
                }
            }
        }

        CurrencyDao::decrease_credits(player.get_details(), price);
        player.send(&CREDIT_BALANCE::new(player.get_details().get_credits()));

        Ok(())
    }
}
