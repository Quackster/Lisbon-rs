//! Mirrors `net.h4bbo.lisbon.messages.outgoing.moderation.PICKED_CRY`.
use crate::game::moderation::cfh::call_for_help::CallForHelp;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct PICKED_CRY {
    cfh: CallForHelp,
}

impl PICKED_CRY {
    /// Mirrors the `PICKED_CRY(CallForHelp)` constructor.
    pub fn new(cfh: &CallForHelp) -> Self {
        Self {
            cfh: cfh.clone(),
        }
    }
}

impl MessageComposer for PICKED_CRY {
    fn compose(&self, response: &mut NettyResponse) {
        response.write_string(self.cfh.get_cry_id());
        response.write_string(self.cfh.get_picked_up_by());
    }

    fn get_header(&self) -> i16 {
        // "BU"
        149
    }
}
