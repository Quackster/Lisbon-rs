//! Mirrors `org.alexdev.http.controllers.housekeeping.HousekeepingTransactionsController`.

use lisbon_server::dao::mysql::player_dao::PlayerDao;
use lisbon_server::dao::mysql::transaction_dao::TransactionDao;
use lisbon_server::game::player::player_rank::PlayerRank;

use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::game::housekeeping::housekeeping_manager::HousekeepingManager;
use crate::routes::HOUSEKEEPING_PATH;
use crate::util::session_util::SessionUtil;

fn is_numeric(value: &str) -> bool {
    !value.is_empty() && value.chars().all(|character| character.is_ascii_digit())
}

fn check_permission(
    web_connection: &WebConnection,
    permission: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    if !web_connection.session().get_boolean(SessionUtil::LOGGED_IN_HOUSKEEPING) {
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(false);
    }

    let user_id = web_connection.session().get_int("user.id");
    let Some(player_details) = PlayerDao::get_details(user_id) else {
        // Java NPEs on a null player.
        return Ok(false);
    };

    if !HousekeepingManager::get_instance().has_permission(
        player_details.get_rank().unwrap_or(PlayerRank::Rankless),
        permission,
    ) {
        web_connection.redirect(&format!("/{}", HOUSEKEEPING_PATH));
        return Ok(false);
    }

    Ok(true)
}

/// Mirrors `search(WebConnection)`.
pub fn search(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !check_permission(web_connection, "transaction/lookup")? {
        return Ok(());
    }

    let mut tpl = web_connection.template("housekeeping/transaction_lookup");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );

    if web_connection.post().get_values().len() > 0 {
        let search_query = web_connection.post().get_string("searchQuery").unwrap_or_default();
        let transactions = TransactionDao::get_transactions_past_month(&search_query, true);

        tpl.set("transactions", TemplateValue::of(transactions));
    }

    if web_connection.get().get_values().len() > 0 {
        let search_query = web_connection.get().get_string("searchQuery").unwrap_or_default();
        let transactions = TransactionDao::get_transactions_past_month(&search_query, true);

        tpl.set("transactions", TemplateValue::of(transactions));
    }

    tpl.set("pageName", TemplateValue::of("Transaction Lookup"));
    tpl.render_html().ok();

    // Delete alert after it's been rendered
    web_connection.session().delete("alertMessage");
    Ok(())
}

/// Mirrors `item_lookup(WebConnection)`.
pub fn item_lookup(
    web_connection: &WebConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    if !check_permission(web_connection, "transaction/lookup")? {
        return Ok(());
    }

    let mut tpl = web_connection.template("housekeeping/transaction_item_lookup");
    tpl.set(
        "housekeepingManager",
        TemplateValue::of(HousekeepingManager::get_instance()),
    );

    let id_string = web_connection.get().get_string("id").unwrap_or_default();
    let item_id = if is_numeric(&id_string) {
        id_string.parse().unwrap_or(0)
    } else {
        0
    };

    let transactions = TransactionDao::get_transaction_by_item(item_id);
    tpl.set("transactions", TemplateValue::of(transactions));

    tpl.set("pageName", TemplateValue::of("Transaction Lookup"));
    tpl.render_html().ok();

    // Delete alert after it's been rendered
    web_connection.session().delete("alertMessage");
    Ok(())
}
