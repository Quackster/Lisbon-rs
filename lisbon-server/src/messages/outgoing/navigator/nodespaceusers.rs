//! Mirrors `net.h4bbo.lisbon.messages.outgoing.navigator.NODESPACEUSERS`.
use crate::game::entity::entity::Entity;
use crate::game::player::player::Player;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct NODESPACEUSERS<'a> {
    players: Vec<&'a Player>,
}

impl<'a> NODESPACEUSERS<'a> {
    /// Mirrors the `NODESPACEUSERS(List<Player>)` constructor.
    pub fn new(players: Vec<&'a Player>) -> Self {
        Self { players }
    }
}

impl MessageComposer for NODESPACEUSERS<'_> {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        for player in &self.players {
            response.write_string(player.get_details().get_name());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        223 // "C_"
    }
}
