//! Mirrors `net.h4bbo.lisbon.game.club.ClubSubscription`.

use crate::dao::mysql::club_gift_dao::ClubGiftDao;
use crate::dao::mysql::currency_dao::CurrencyDao;
use crate::dao::mysql::player_dao::PlayerDao;
use crate::dao::mysql::player_statistics_dao::PlayerStatisticsDao;
use crate::game::item::item_manager::ItemManager;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::game::player::player_details::PlayerDetails;
use crate::game::player::statistics::player_statistic::PlayerStatistic;
use crate::game::player::statistics::player_statistic_manager::PlayerStatisticManager;
use crate::messages::outgoing::club::club_info::CLUB_INFO;
use crate::util::config::game_configuration::GameConfiguration;
use crate::util::date_util::DateUtil;

pub struct ClubSubscription;

impl ClubSubscription {
    /// Mirrors `sendHcDays(Player)`.
    pub fn send_hc_days(player: &Player) {
        let now = DateUtil::get_current_time_seconds() as i64;

        let mut total_days = 0;

        if player.get_details().get_club_expiration() != 0 {
            total_days = (player.get_details().get_club_expiration() - now) / 60 / 60 / 24;
        }

        if total_days < 0 {
            total_days = 0;
        }

        let mut remaining_days_this_month = 0;
        let mut prepaid_months = 0;
        let mut since_months = 0;

        if total_days > 0 {
            remaining_days_this_month = ((total_days - 1) % 31) + 1;
            prepaid_months = (total_days - remaining_days_this_month) / 31;

            if player.get_details().get_first_club_subscription() > 0 {
                let days = player.get_statistic_manager().get_long_value(PlayerStatistic::ClubMemberTime) / 86_400;
                since_months = if days > 0 {
                    (days / 31) as i32
                } else {
                    0
                };
            }
        }

        player.send(&CLUB_INFO::new(
            remaining_days_this_month as i32,
            since_months,
            prepaid_months as i32,
        ));
    }

    /// Mirrors `isGiftDue(Player)`.
    pub fn is_gift_due(player: &Player) -> bool {
        if !player.get_details().has_club_subscription() {
            return false;
        }

        if player.get_statistic_manager().get_int_value(PlayerStatistic::GiftsDue) > 0 {
            return true;
        }

        if player.get_details().get_first_club_subscription() == 0 {
            return true;
        }

        false
    }

    /// Mirrors `checkBadges(Player)`.
    pub fn check_badges(player: &Player) {
        if player.get_details().has_club_subscription() {
            if !player.get_badge_manager().has_badge("HC1") {
                player.get_badge_manager().try_add_badge("HC1", None, 0);
            }
        }

        if Self::has_gold_club_subscription(player) {
            if !player.get_badge_manager().has_badge("HC2") {
                player.get_badge_manager().try_add_badge("HC2", None, 0);
            }
        }

        if Self::has_platinum_club_subscription(player) {
            if !player.get_badge_manager().has_badge("HC3") {
                player.get_badge_manager().try_add_badge("HC3", None, 0);
            }
        }
    }

    /// Mirrors `subscribeClub(PlayerDetails, int)`.
    pub fn subscribe_club(player_details: &mut PlayerDetails, choice: i32) -> bool {
        let (credits, days) = Self::choice_data(choice);

        if days <= 0 {
            return false;
        }

        if player_details.get_credits() < credits {
            return false;
        }

        let now = DateUtil::get_current_time_seconds() as i64;
        let seconds_to_add = 24 * 60 * 60 * days;

        if player_details.get_first_club_subscription() == 0 {
            PlayerStatisticsDao::update_statistic(
                player_details.get_id(),
                PlayerStatistic::GiftsDue,
                "1",
            );
            PlayerStatisticsDao::update_statistic(
                player_details.get_id(),
                PlayerStatistic::ClubGiftDue,
                &now.to_string(),
            );
        }

        if player_details.get_club_expiration() - now <= 0 {
            player_details.set_club_expiration(now + i64::from(seconds_to_add) + 1);
        } else {
            player_details.set_club_expiration(
                player_details.get_club_expiration() + i64::from(seconds_to_add),
            );
        }

        PlayerDao::save_subscription(
            player_details.get_id(),
            player_details.get_first_club_subscription(),
            player_details.get_club_expiration(),
        );
        CurrencyDao::decrease_credits(player_details, credits);

        true
    }

    /// Mirrors `tryNextGift(Player)` (the Java `SQLException` is carried by
    /// the DAO calls).
    pub fn try_next_gift(player: &mut Player) {
        if !Self::is_gift_due(player) {
            return;
        }

        let item = if player.get_details().get_first_club_subscription() == 0 {
            player.get_details_mut().set_first_club_subscription(
                DateUtil::get_current_time_seconds() as i64,
            );
            let item = ItemManager::get_instance().create_gift(
                player.get_details().get_id(),
                player.get_details().get_name(),
                "club_sofa",
                &GameConfiguration::get_instance().get_string("club.gift.present.label"),
                "",
            );

            PlayerDao::save_subscription(
                player.get_details().get_id(),
                player.get_details().get_first_club_subscription(),
                player.get_details().get_club_expiration(),
            );

            item
        } else {
            let gift_data: Option<String> = player
                .get_last_gift()
                .map(String::from)
                .or_else(|| {
                    ClubGiftDao::get_last_gift(player.get_details().get_id())
                        .map(|(_, value)| value)
                });

            let gift_order = Self::gift_order();
            let next_sprite_gift: String = match gift_data {
                None => gift_order[0].clone(),
                Some(gift_data) => {
                    let mut position = 0;

                    for next_gift in &gift_order {
                        position += 1;

                        if next_gift.as_str() == gift_data {
                            break;
                        }
                    }

                    if position >= gift_order.len() {
                        position = 0;
                    }

                    gift_order[position].clone()
                }
            };

            ClubGiftDao::add_gift(player.get_details().get_id(), &next_sprite_gift);
            player.set_last_gift(Some(&next_sprite_gift));

            ItemManager::get_instance().create_gift(
                player.get_details().get_id(),
                player.get_details().get_name(),
                &next_sprite_gift,
                &GameConfiguration::get_instance().get_string("club.gift.present.label"),
                "",
            )
        };

        // Java: `player.getStatisticManager().incrementValue(GIFTS_DUE, -1)`.
        player
            .get_statistic_manager()
            .increment_value(PlayerStatistic::GiftsDue, -1);

        if let Some(inventory) = player.get_inventory() {
            inventory.add_item(&item);
            inventory.view(player, "new");
        }
    }

    /// Mirrors `getClubGiftSeconds()`.
    pub fn get_club_gift_seconds() -> i64 {
        let unit = GameConfiguration::get_instance().get_string("club.gift.timeunit");
        let interval =
            GameConfiguration::get_instance().get_integer("club.gift.interval") as i64;

        match unit.to_uppercase().as_str() {
            "NANOSECONDS" => interval / 1_000_000_000,
            "MICROSECONDS" => interval / 1_000_000,
            "MILLISECONDS" => interval / 1_000,
            "SECONDS" => interval,
            "MINUTES" => interval * 60,
            "HOURS" => interval * 3_600,
            "DAYS" => interval * 86_400,
            _ => interval,
        }
    }

    /// Mirrors `getChoiceData(int)`.
    pub fn choice_data(choice: i32) -> (i32, i32) {
        match choice {
            1 => (25, 31),
            2 => (60, 93),
            3 => (105, 186),
            _ => (-1, -1),
        }
    }

    /// Mirrors `getGiftOrder()`.
    pub fn gift_order() -> Vec<String> {
        vec![
            "hc_tv", "hcamme", "hc_crtn", "mocchamaster", "hc_crpt", "edicehc",
            "hc_wall_lamp", "doorD", "deal_hcrollers", "hcsohva", "hc_bkshlf",
            "hc_lmp", "hc_trll", "hc_tbl", "hc_machine", "hc_chr", "hc_rntgn",
            "hc_dsk", "hc_djset", "hc_lmpst", "hc_frplc", "hc_btlr",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    /// Mirrors `countMemberDays(Player)`.
    pub fn count_member_days(player: &Player) {
        Self::count_member_days_details(player.get_details(), player.get_statistic_manager());
    }

    /// Mirrors `countMemberDays(PlayerDetails, PlayerStatisticManager)`.
    pub fn count_member_days_details(
        details: &PlayerDetails,
        statistic_manager: &PlayerStatisticManager,
    ) {
        if details.has_club_subscription() {
            let last_updated = statistic_manager.get_long_value(PlayerStatistic::ClubMemberTimeUpdated);

            if last_updated > 0 {
                PlayerStatisticsDao::update_statistic(
                    details.get_id(),
                    PlayerStatistic::ClubMemberTime,
                    &format!(
                        "{}",
                        statistic_manager.get_long_value(PlayerStatistic::ClubMemberTime)
                            + DateUtil::get_current_time_seconds() as i64
                            - last_updated
                    ),
                );
            }

            PlayerStatisticsDao::update_statistic(
                details.get_id(),
                PlayerStatistic::ClubMemberTimeUpdated,
                &DateUtil::get_current_time_seconds().to_string(),
            );
        }
    }

    /// Mirrors `hasGoldClubSubscription(Player)`.
    pub fn has_gold_club_subscription(player: &Player) -> bool {
        if player.get_details().has_club_subscription() {
            let days = player
                .get_statistic_manager()
                .get_long_value(PlayerStatistic::ClubMemberTime)
                / 86_400;
            let since_months = if days > 0 { days / 31 } else { 0 };

            return since_months >= 12;
        }

        false
    }

    /// Mirrors `hasPlatinumClubSubscription(Player)`.
    pub fn has_platinum_club_subscription(player: &Player) -> bool {
        if player.get_details().has_club_subscription() {
            let days = player
                .get_statistic_manager()
                .get_long_value(PlayerStatistic::ClubMemberTime)
                / 86_400;
            let since_months = if days > 0 { days / 31 } else { 0 };

            return since_months >= 24;
        }

        false
    }
}
