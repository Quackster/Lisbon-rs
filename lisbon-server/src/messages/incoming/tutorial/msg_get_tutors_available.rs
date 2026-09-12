//! Mirrors `net.h4bbo.lisbon.messages.incoming.tutorial.MSG_GET_TUTORS_AVAILABLE`.
use crate::game::achievements::achievement_manager::AchievementManager;
use crate::game::achievements::achievement_type::AchievementType;
use crate::game::guides::guide_manager::GuideManager;
use crate::game::player::player::Player;
use crate::game::player::statistics::player_statistic::PlayerStatistic;
use crate::messages::outgoing::tutorial::tutors_available::TUTORS_AVAILABLE;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MSG_GET_TUTORS_AVAILABLE;

impl MessageEvent for MSG_GET_TUTORS_AVAILABLE {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        if player
            .get_statistic_manager()
            .get_int_value(PlayerStatistic::IsGuidable)
            != 1
        {
            GuideManager::get_instance().try_clear_tutorial(player);
            return Ok(());
        }

        if !player.get_guide_manager().can_use_tutorial() {
            GuideManager::get_instance().try_clear_tutorial(player);
            return Ok(());
        }

        player.get_guide_manager().set_guidable(true);
        player.get_guide_manager().set_blocking_tutorial(true);

        AchievementManager::get_instance()
            .try_progress(&AchievementType::Graduate, player);
        player.send(&TUTORS_AVAILABLE::new(1));

        Ok(())
    }
}
