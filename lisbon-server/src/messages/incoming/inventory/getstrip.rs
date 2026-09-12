//! Mirrors `net.h4bbo.lisbon.messages.incoming.inventory.GETSTRIP`.
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct GETSTRIP;

impl MessageEvent for GETSTRIP {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        let Some(strip_view) = reader.contents() else {
            return Ok(());
        };

        if let Some(inventory) = player.get_inventory() {
            inventory.view(player, &strip_view);
        }

        Ok(())
    }
}
