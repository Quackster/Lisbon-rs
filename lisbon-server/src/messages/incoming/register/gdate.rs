//! Mirrors `net.h4bbo.lisbon.messages.incoming.register.GDATE`.
use crate::game::player::player::Player;
use crate::messages::outgoing::register::date::DATE;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;
use crate::util::date_util::DateUtil;

#[allow(non_camel_case_types)]
pub struct GDATE;

impl MessageEvent for GDATE {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, _reader: &mut NettyRequest) -> Result<(), String> {
        player.send(&DATE::new(
            DateUtil::get_short_date()
                .replace('-', ".")
                .as_str(),
        ));

        Ok(())
    }
}
