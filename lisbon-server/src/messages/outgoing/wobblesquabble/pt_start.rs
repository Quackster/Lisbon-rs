//! Mirrors `net.h4bbo.lisbon.messages.outgoing.wobblesquabble.PT_START`.
use std::sync::Arc;

use parking_lot::Mutex;

use crate::game::entity::entity::Entity;
use crate::game::games::wobblesquabble::wobble_squabble_player::WobbleSquabblePlayer;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct PT_START {
    player1: Arc<Mutex<WobbleSquabblePlayer>>,
    player2: Arc<Mutex<WobbleSquabblePlayer>>,
}

impl PT_START {
    /// Mirrors the `PT_START(WobbleSquabblePlayer, WobbleSquabblePlayer)` constructor.
    pub fn new(
        player1: Arc<Mutex<WobbleSquabblePlayer>>,
        player2: Arc<Mutex<WobbleSquabblePlayer>>,
    ) -> Self {
        Self { player1, player2 }
    }
}

impl MessageComposer for PT_START {
    fn compose(&self, response: &mut NettyResponse) {
        let id1 = self
            .player1
            .lock()
            .get_player()
            .lock()
            .get_room_user()
            .map(|room_user| room_user.get_instance_id())
            .unwrap_or(0);
        response.write_delimeter(format!("0:{}", id1), '\u{000d}');

        let id2 = self
            .player2
            .lock()
            .get_player()
            .lock()
            .get_room_user()
            .map(|room_user| room_user.get_instance_id())
            .unwrap_or(0);
        response.write(format!("1:{}", id2));
    }

    fn get_header(&self) -> i16 {
        // "Ar"
        114
    }
}
