//! Mirrors `org.alexdev.http.controllers.habblet.VoucherController`.

use std::collections::HashMap;

use lisbon_server::game::catalogue::catalogue_item::CatalogueItem;
use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::game::catalogue::voucher::voucher_manager::VoucherManager;
use lisbon_server::game::catalogue::voucher::voucher_redeem_mode::VoucherRedeemMode;
use lisbon_server::game::catalogue::voucher::voucher_redeem_status::VoucherRedeemStatus;
use lisbon_server::server::rcon::messages::rcon_header::RconHeader;

use crate::duckhttpd::{TemplateValue, WebConnection};
use crate::util::rcon_util::RconUtil;

/// Mirrors `redeemVoucher(WebConnection)`.
pub fn redeem_voucher(web_connection: &WebConnection) -> Result<(), Box<dyn std::error::Error>> {
    let mut template = web_connection.template("habblet/redeemvoucher");

    let player_details = match template
        .get("playerDetails")
        .and_then(|value| PlayerDetails::from_json(value.value()))
    {
        Some(details) => details,
        None => return Ok(()),
    };

    let voucher_code = web_connection
        .post()
        .get_string("voucherCode")
        .unwrap_or_default();

    let mut redeemed_credits: i32 = 0;
    let mut redeemed_items: Vec<CatalogueItem> = Vec::new();

    let voucher_status = VoucherManager::get_instance().redeem(
        &player_details,
        VoucherRedeemMode::InGame,
        &voucher_code,
        &mut redeemed_items,
        &mut redeemed_credits,
    );

    if voucher_status == VoucherRedeemStatus::Failure {
        template.set("voucherResult", TemplateValue::of("error"));
    }

    if voucher_status == VoucherRedeemStatus::FailureNewAccount {
        template.set("voucherResult", TemplateValue::of("too_new"));
    }

    if voucher_status == VoucherRedeemStatus::Success {
        template.set("voucherResult", TemplateValue::of("success"));
    }

    if !redeemed_items.is_empty() {
        RconUtil::send_command(
            RconHeader::RefreshHand,
            HashMap::from([("userId".to_string(), player_details.get_id().to_string())]),
        );
    }

    if redeemed_credits > 0 {
        RconUtil::send_command(
            RconHeader::RefreshCredits,
            HashMap::from([("userId".to_string(), player_details.get_id().to_string())]),
        );
    }

    template.render();
    Ok(())
}
