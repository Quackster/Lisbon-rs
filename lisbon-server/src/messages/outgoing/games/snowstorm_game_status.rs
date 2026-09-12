//! Mirrors `net.h4bbo.lisbon.messages.outgoing.games.SNOWSTORM_GAMESTATUS`.
use crate::game::games::snowstorm::snow_storm_turn::SnowStormTurn;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct SNOWSTORM_GAMESTATUS {
    turns: Vec<SnowStormTurn>,
}

impl SNOWSTORM_GAMESTATUS {
    /// Mirrors the `SNOWSTORM_GAMESTATUS(List<SnowStormTurn>)` constructor.
    pub fn new(events: Vec<SnowStormTurn>) -> Self {
        Self { turns: events }
    }
}

impl MessageComposer for SNOWSTORM_GAMESTATUS {
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(1);
        response.write_int(1);
        response.write_int(if self.turns.is_empty() {
            1
        } else {
            self.turns.len() as i32
        });

        for turn in &self.turns {
            response.write_int(turn.get_sub_turns().len() as i32);

            for game_object in turn.get_sub_turns() {
                game_object.serialise_object(response);
            }
        }
    }

    fn get_header(&self) -> i16 {
        244
    }
}
