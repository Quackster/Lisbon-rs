//! Mirrors `net.h4bbo.lisbon.messages.incoming.tutorial.RESET_TUTORIAL`.
use crate::game::player::player::Player;
use crate::game::player::statistics::player_statistic::PlayerStatistic;
use crate::messages::outgoing::alert::alert::ALERT;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct RESET_TUTORIAL;

impl MessageEvent for RESET_TUTORIAL {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        if !player.is_logged_in() {
            return Ok(());
        }

        if player.get_guide_manager().is_guide() {
            player.send(&ALERT::new("You cannot restart the tutorial while as a guide."));
            return Ok(());
        }

        player
            .get_statistic_manager()
            .set_long_value(PlayerStatistic::HasTutorial, 1);

        if !player.get_badge_manager().has_badge("ACH_Student1") {
            player
                .get_statistic_manager()
                .set_long_value(PlayerStatistic::IsGuidable, 1);
        }

        player.send(
            &ALERT::new("You may now do the tutorial again, please relog for it to take effect."),
        );

        Ok(())
    }
}
