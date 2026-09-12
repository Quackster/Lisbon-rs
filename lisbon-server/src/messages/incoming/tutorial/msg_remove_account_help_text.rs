//! Mirrors `net.h4bbo.lisbon.messages.incoming.tutorial.MSG_REMOVE_ACCOUNT_HELP_TEXT`.
use crate::game::achievements::achievement_manager::AchievementManager;
use crate::game::achievements::achievement_type::AchievementType;
use crate::game::guides::guide_manager::GuideManager;
use crate::game::player::player::Player;
use crate::game::player::statistics::player_statistic::PlayerStatistic;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MSG_REMOVE_ACCOUNT_HELP_TEXT;

impl MessageEvent for MSG_REMOVE_ACCOUNT_HELP_TEXT {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        if !player.get_guide_manager().has_tutorial() {
            return Ok(());
        }

        if player.get_guide_manager().is_blocking_tutorial() {
            player.get_guide_manager().set_blocking_tutorial(false);
            player.get_guide_manager().set_cancel_tutorial(true);
            GuideManager::get_instance().try_clear_tutorial(player);
            player
                .get_statistic_manager()
                .set_long_value(PlayerStatistic::IsGuidable, 0);
            return Ok(());
        }

        let _id = reader.read_int();

        if player.get_guide_manager().is_cancel_tutorial() {
            player.get_guide_manager().set_cancel_tutorial(false);
            GuideManager::get_instance().try_clear_tutorial(player);
            player
                .get_statistic_manager()
                .set_long_value(PlayerStatistic::IsGuidable, 0);
            AchievementManager::get_instance()
                .try_progress(&AchievementType::Graduate, player);
            return Ok(());
        }

        if !player.get_guide_manager().can_use_tutorial() {
            player.get_guide_manager().set_can_use_tutorial(true);
        }

        Ok(())
    }
}
