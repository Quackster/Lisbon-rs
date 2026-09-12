//! Mirrors `net.h4bbo.lisbon.messages.incoming.moderation.CHANGECALLCATEGORY`.
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::moderation::cfh::call_for_help_manager::CallForHelpManager;
use crate::game::player::player::Player;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct CHANGECALLCATEGORY;

impl MessageEvent for CHANGECALLCATEGORY {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        // Only players that have this fuse are allowed to change call category
        if !player.has_fuse(&Fuseright::ReceiveCallsForHelp) {
            return Ok(());
        }

        let call_id = reader
            .read_string()
            .parse::<i32>()
            .map_err(|_| "Invalid call ID".to_string())?;
        let category = reader.read_int();

        let Some(cfh) = CallForHelpManager::get_instance().get_call(call_id) else {
            return Ok(());
        };

        CallForHelpManager::get_instance().change_category(&cfh, category);

        Ok(())
    }
}
