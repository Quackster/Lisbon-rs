//! Mirrors `net.h4bbo.lisbon.messages.incoming.purse.REDEEM_VOUCHER`.
use crate::game::entity::entity::Entity;
use crate::game::catalogue::catalogue_item::CatalogueItem;
use crate::game::catalogue::voucher::voucher_manager::VoucherManager;
use crate::game::catalogue::voucher::voucher_redeem_mode::VoucherRedeemMode;
use crate::game::catalogue::voucher::voucher_redeem_status::VoucherRedeemStatus;
use crate::game::player::player::Player;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::messages::outgoing::purse::voucher_redeem_error::{RedeemError, VOUCHER_REDEEM_ERROR};
use crate::messages::outgoing::purse::voucher_redeem_ok::VOUCHER_REDEEM_OK;
use crate::messages::outgoing::user::currencies::credit_balance::CREDIT_BALANCE;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct REDEEM_VOUCHER;

impl MessageEvent for REDEEM_VOUCHER {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        if !player.is_logged_in() {
            return Ok(());
        }

        let mut redeemed_credits = 0;
        let mut redeemed_item: Vec<CatalogueItem> = Vec::new();

        let voucher_status = VoucherManager::get_instance().redeem(
            player.get_details(),
            VoucherRedeemMode::InGame,
            &reader.read_string(),
            &mut redeemed_item,
            &mut redeemed_credits,
        );

        if voucher_status == VoucherRedeemStatus::Failure {
            player.send(&VOUCHER_REDEEM_ERROR::new(RedeemError::Invalid));
            return Ok(());
        }

        if voucher_status == VoucherRedeemStatus::FailureNewAccount {
            player.send(&ALERT::new(
                "Sorry, your account is too new and cannot redeem this voucher",
            ));
            return Ok(());
        }

        let has_item = !redeemed_item.is_empty();
        player.send(&VOUCHER_REDEEM_OK::new(redeemed_item));

        if redeemed_credits > 0 {
            player.send(&CREDIT_BALANCE::new(player.get_details().get_credits()));
        }

        if has_item {
            if let Some(inventory) = player.get_inventory() {
                inventory.reload(
                    player.get_details().get_id(),
                    player.get_room_user(),
                );

                if player
                    .get_room_user()
                    .and_then(|room_user| room_user.get_room())
                    .is_some()
                {
                    inventory.view(player, "new");
                }
            }
        }

        Ok(())
    }
}
