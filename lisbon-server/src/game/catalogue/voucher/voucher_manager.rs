//! Mirrors `net.h4bbo.lisbon.game.catalogue.voucher.VoucherManager`.

use std::sync::Arc;

use lazy_static::lazy_static;
use parking_lot::RwLock;

use crate::dao::mysql::currency_dao::CurrencyDao;
use crate::dao::mysql::player_statistics_dao::PlayerStatisticsDao;
use crate::dao::mysql::voucher_dao::VoucherDao;
use crate::game::catalogue::catalogue_item::CatalogueItem;
use crate::game::catalogue::catalogue_manager::CatalogueManager;
use crate::game::catalogue::voucher::voucher_redeem_mode::VoucherRedeemMode;
use crate::game::catalogue::voucher::voucher_redeem_status::VoucherRedeemStatus;
use crate::game::player::player_details::PlayerDetails;
use crate::game::player::statistics::player_statistic::PlayerStatistic;
use crate::log::Log;
use crate::util::date_util::DateUtil;

lazy_static! {
    static ref INSTANCE: RwLock<Option<Arc<VoucherManager>>> = RwLock::new(None);
}

#[derive(Clone)]
pub struct VoucherManager;

impl VoucherManager {
    fn new() -> Self {
        Self
    }

    /// Mirrors `redeem(PlayerDetails, VoucherRedeemMode, String, List<CatalogueItem>, AtomicInteger)`.
    pub fn redeem(
        &self,
        player_details: &PlayerDetails,
        _voucher_redeem_mode: VoucherRedeemMode,
        voucher_code: &str,
        redeemed_items: &mut Vec<CatalogueItem>,
        redeemed_credits: &mut i32,
    ) -> VoucherRedeemStatus {
        let voucher = VoucherDao::redeem_voucher(voucher_code, player_details.get_id());

        let voucher = match voucher {
            Some(voucher) => voucher,
            None => return VoucherRedeemStatus::Failure,
        };

        if !voucher.is_allow_new_users() {
            let online_time =
                PlayerStatisticsDao::get_statistic_long(player_details.get_id(), PlayerStatistic::OnlineTime);
            let days_since = online_time / 3600;

            if days_since < 1 {
                return VoucherRedeemStatus::FailureNewAccount;
            }
        }

        for catalogue_sale_code in voucher.get_items() {
            let catalogue_item = CatalogueManager::get_instance().get_catalogue_item(&catalogue_sale_code);

            match catalogue_item {
                Some(catalogue_item) => {
                    redeemed_items.push(catalogue_item.clone());

                    let _ = CatalogueManager::get_instance().purchase(
                        player_details,
                        catalogue_item,
                        Some(""),
                        None,
                        DateUtil::get_current_time_seconds() as i64,
                    );
                }
                None => {
                    Log::get_error_logger().error(format!(
                        "Could not redeem voucher {voucher_code} with sale code: {catalogue_sale_code}"
                    ));
                }
            }
        }

        VoucherDao::log_voucher(
            voucher_code,
            player_details.get_id(),
            voucher.get_credits(),
            redeemed_items,
        );

        if voucher.get_credits() > 0 {
            CurrencyDao::increase_credits(player_details, voucher.get_credits());
            *redeemed_credits = voucher.get_credits();
        }

        VoucherRedeemStatus::Success
    }

    /// Mirrors `getInstance()`.
    pub fn get_instance() -> Arc<VoucherManager> {
        if let Some(existing) = INSTANCE.read().as_ref() {
            return existing.clone();
        }
        let instance = Arc::new(Self::new());
        INSTANCE.write().replace(instance.clone());
        instance
    }

    /// Mirrors `reset()`.
    pub fn reset() {
        INSTANCE.write().take();
        Self::get_instance();
    }
}
