//! Mirrors `net.h4bbo.lisbon.messages.incoming.moderation.MESSAGETOCALLER`.
use crate::game::entity::entity::Entity;
use crate::game::fuserights::fuseright::Fuseright;
use crate::game::moderation::cfh::call_for_help_manager::CallForHelpManager;
use crate::game::player::player::Player;
use crate::game::player::player_manager::PlayerManager;
use crate::messages::outgoing::moderation::cry_reply::CRY_REPLY;
use crate::messages::types::MessageEvent;
use crate::server::netty::streams::NettyRequest;

#[allow(non_camel_case_types)]
pub struct MESSAGETOCALLER;

impl MessageEvent for MESSAGETOCALLER {
    /// Mirrors `handle(Player, NettyRequest)`.
    fn handle(&self, player: &Player, reader: &mut NettyRequest) -> Result<(), String> {
        // Only players that have this fuse are allowed to reply to call for helps
        if !player.has_fuse(&Fuseright::ReceiveCallsForHelp) {
            return Ok(());
        }

        // The inconsistent v21 client sends the call ID non-VL64 encoded :/
        let call_id = reader
            .read_string()
            .parse::<i32>()
            .map_err(|_| "Invalid call ID".to_string())?;
        let message = reader.read_string();

        // Retrieve call for help by ID provided by client
        let Some(cfh) = CallForHelpManager::get_instance().get_call(call_id) else {
            return Ok(());
        };

        // Call has been handled, delete it :)
        CallForHelpManager::get_instance().delete_call(&cfh);

        // Get callee of call for help
        let Some(caller) = PlayerManager::get_instance().get_player_by_id(cfh.get_caller()) else {
            return Ok(());
        };

        // Notify callee
        caller.lock().send(&CRY_REPLY::new(&message));

        Ok(())
    }
}
