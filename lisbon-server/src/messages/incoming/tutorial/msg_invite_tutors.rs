//! Mirrors `net.h4bbo.lisbon.messages.incoming.tutorial.MSG_INVITE_TUTORS`.
use crate::game::player::player::Player;
use crate::messages::outgoing::tutorial::invitation_sent::INVITATION_SENT;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::config::game_configuration::GameConfiguration;
use crate::util::date_util::DateUtil;

#[allow(non_camel_case_types)]
pub struct MSG_INVITE_TUTORS;

impl MessageEvent for MSG_INVITE_TUTORS {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        if !player.get_guide_manager().is_guidable() {
            return Ok(());
        }

        if player.get_guide_manager().is_waiting_for_guide() {
            return Ok(());
        }

        player.send(&INVITATION_SENT);

        let timeout_minutes = GameConfiguration::get_instance()
            .get_integer("guide.search.timeout.minutes");
        player.get_guide_manager().set_started_for_waiting_guides_time(
            DateUtil::get_current_time_seconds() as i32 + (timeout_minutes as i64 * 60) as i32,
        );
        player.get_guide_manager().set_waiting_for_guide(true);

        Ok(())
    }
}
