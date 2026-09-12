//! Mirrors `net.h4bbo.lisbon.messages.outgoing.wobblesquabble.PT_PREPARE`.
use crate::game::entity::entity::Entity;
use crate::game::games::wobblesquabble::wobble_squabble_player::WobbleSquabblePlayer;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct PT_PREPARE {
    player1: WobbleSquabblePlayer,
    player2: WobbleSquabblePlayer,
}

impl PT_PREPARE {
    /// Mirrors the `PT_PREPARE(WobbleSquabblePlayer, WobbleSquabblePlayer)`
    /// constructor.
    pub fn new(player1: WobbleSquabblePlayer, player2: WobbleSquabblePlayer) -> Self {
        Self {
            player1,
            player2,
        }
    }
}

impl MessageComposer for PT_PREPARE {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        let id1 = self
            .player1
            .get_player()
            .lock()
            .get_room_user()
            .map(|room_user| room_user.get_instance_id())
            .unwrap_or(0);
        response.write_delimeter(format!("0:{}", id1), '\u{000d}');

        let id2 = self
            .player2
            .get_player()
            .lock()
            .get_room_user()
            .map(|room_user| room_user.get_instance_id())
            .unwrap_or(0);
        response.write(format!("1:{}", id2));
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        115 // "As"
    }
}
