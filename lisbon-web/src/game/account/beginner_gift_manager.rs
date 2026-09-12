//! Mirrors `org.alexdev.http.game.account.BeginnerGiftManager`.

use lisbon_server::dao::mysql::alerts_dao::AlertsDao;
use lisbon_server::game::alerts::alert_type::AlertType;
use lisbon_server::game::item::item_manager::ItemManager;
use lisbon_server::game::player::player_details::PlayerDetails;
use lisbon_server::game::player::statistics::player_statistic::PlayerStatistic;
use lisbon_server::game::player::statistics::player_statistic_manager::PlayerStatisticManager;
use lisbon_server::util::config::game_configuration::GameConfiguration;
use lisbon_server::util::date_util::DateUtil;

pub struct BeginnerGiftManager;

impl BeginnerGiftManager {
    /// Mirrors `progress(PlayerDetails, PlayerStatisticManager)`.
    pub fn progress(player_details: &PlayerDetails, statistics: &PlayerStatisticManager) -> bool {
        if !(statistics.get_int_value(PlayerStatistic::NewbieRoomLayout) > 0
            && statistics.get_int_value(PlayerStatistic::NewbieGift) > 0)
        {
            return false;
        }

        if statistics.get_int_value(PlayerStatistic::NewbieGift) > 2 {
            return false;
        }

        if statistics.get_int_value(PlayerStatistic::NewbieGiftTime) > DateUtil::get_current_time_seconds() {
            return false;
        }

        let room_layout = statistics.get_int_value(PlayerStatistic::NewbieRoomLayout);
        let gift = statistics.get_int_value(PlayerStatistic::NewbieGift);

        let item_gift = match gift {
            1 => ItemManager::get_instance().get_definition_by_sprite(&format!("noob_stool*{room_layout}")),
            2 => ItemManager::get_instance().get_definition_by_sprite("noob_plant"),
            _ => None,
        };

        let Some(item_gift) = item_gift else {
            return false;
        };

        let present_label = GameConfiguration::get_instance()
            .get_string("alerts.gift.message")
            .replace("%item_name%", item_gift.get_name());

        AlertsDao::create_alert(player_details.get_id(), AlertType::Present, &present_label);

        ItemManager::get_instance().create_gift(
            player_details.get_id(),
            player_details.get_name(),
            item_gift.get_sprite(),
            &present_label,
            "",
        );

        let next_gift = gift + 1;

        if next_gift < 3 {
            statistics.set_long_value(PlayerStatistic::NewbieGift, next_gift as i64);
            statistics.set_long_value(
                PlayerStatistic::NewbieGiftTime,
                DateUtil::get_current_time_seconds() as i64 + 86400,
            );
        } else {
            statistics.set_long_value(PlayerStatistic::NewbieGift, 3);
            statistics.set_long_value(PlayerStatistic::NewbieGiftTime, 0);
        }

        true
    }
}
