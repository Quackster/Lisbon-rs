//! Mirrors `net.h4bbo.lisbon.messages.outgoing.wobblesquabble.PT_STATUS`.
use crate::game::games::wobblesquabble::wobble_squabble_player::WobbleSquabblePlayer;
use crate::game::games::wobblesquabble::wobble_squabble_status::WobbleSquabbleStatus;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct PT_STATUS {
    statuses: [WobbleSquabbleStatus; 2],
}

impl PT_STATUS {
    /// Mirrors the `PT_STATUS(WobbleSquabblePlayer, WobbleSquabblePlayer)` constructor.
    pub fn new(
        ws_player1: &WobbleSquabblePlayer,
        ws_player2: &WobbleSquabblePlayer,
    ) -> Self {
        let statuses = [
            WobbleSquabbleStatus::new(
                ws_player1.get_position(),
                ws_player1.get_balance(),
                ws_player1.get_move(),
                ws_player1.is_hit(),
            ),
            WobbleSquabbleStatus::new(
                ws_player2.get_position(),
                ws_player2.get_balance(),
                ws_player2.get_move(),
                ws_player2.is_hit(),
            ),
        ];

        Self { statuses }
    }
}

impl MessageComposer for PT_STATUS {
    fn compose(&self, response: &mut NettyResponse) {
        for ws_status in &self.statuses {
            response.write_delimeter(ws_status.get_position(), 9);
            response.write_delimeter(ws_status.get_balance(), 9);
            response.write_delimeter(ws_status.get_move().get_letter(), 9);
            response.write_delimeter(if ws_status.is_hit() { "h" } else { "" }, 9);
            response.write(13);
        }
    }

    fn get_header(&self) -> i16 {
        118
    }
}
