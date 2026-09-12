//! Mirrors `net.h4bbo.lisbon.messages.incoming.catalogue.GCAP`.
use std::sync::atomic::Ordering;

use crate::game::catalogue::catalogue_manager::CatalogueManager;
use crate::game::catalogue::rare_manager::RareManager;
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::outgoing::catalogue::catalogue_page::CATALOGUE_PAGE;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::config::game_configuration::GameConfiguration;
use crate::util::date_util::DateUtil;

#[allow(non_camel_case_types)]
pub struct GCAP;

impl GCAP {
    /// Mirrors `TimeUnit.valueOf(String).toSeconds(long)`.
    fn time_unit_to_seconds(unit: &str, value: i32) -> i64 {
        let value = value as i64;
        match unit.to_uppercase().as_str() {
            "NANOS" | "NANOSECONDS" => value / 1_000_000_000,
            "MICROS" | "MICROSECONDS" => value / 1_000_000,
            "MILLIS" | "MILLISECONDS" => value / 1_000,
            "SECONDS" => value,
            "MINUTES" => value * 60,
            "HOURS" => value * 3600,
            "DAYS" => value * 86400,
            _ => 0,
        }
    }
}

impl MessageEvent for GCAP {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(contents) = reader.contents() else {
            return Ok(());
        };
        let Some(page_name) = contents.split('/').nth(1) else {
            return Ok(());
        };

        let Some(mut catalogue_page) =
            CatalogueManager::get_instance().get_catalogue_page(page_name)
        else {
            return Ok(());
        };

        let rank_id = player
            .get_details()
            .get_rank()
            .map(|rank| rank.rank_id())
            .unwrap_or(i32::MIN);

        if rank_id >= catalogue_page.get_min_role().rank_id() {
            let mut catalogue_item_list = CatalogueManager::get_instance()
                .get_catalogue_page_items(catalogue_page.get_id(), false);

            let config = GameConfiguration::get_instance();

            if RareManager::get_instance().get_current_rare().is_some()
                && catalogue_page.get_id() == config.get_integer("rare.cycle.page.id")
            {
                let Some(current_rare) = RareManager::get_instance().get_current_rare() else {
                    return Ok(());
                };

                let mut rare_item = current_rare.copy();
                let rare_cost_map = RareManager::get_instance().get_rare_cost();
                let Some(rare_cost) = rare_cost_map.get(&current_rare.get_id()) else {
                    return Ok(());
                };
                rare_item.set_price(*rare_cost);
                catalogue_item_list = vec![rare_item];

                let interval = Self::time_unit_to_seconds(
                    &config.get_string("rare.cycle.refresh.timeunit"),
                    config.get_integer("rare.cycle.refresh.interval"),
                );
                let current_tick = RareManager::get_instance().get_tick().load(Ordering::SeqCst);
                let time_until = interval - current_tick;

                catalogue_page.set_body(Some(
                    config
                        .get_string("rare.cycle.page.text")
                        .replace(
                            "{rareCountdown}",
                            DateUtil::get_readable_seconds(time_until)
                                .as_deref()
                                .unwrap_or(""),
                        ),
                ));
            }

            player.send(&CATALOGUE_PAGE::new(catalogue_page, catalogue_item_list));
        }

        Ok(())
    }
}
