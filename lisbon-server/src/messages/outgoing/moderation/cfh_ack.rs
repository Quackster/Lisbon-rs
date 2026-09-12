//! Mirrors `net.h4bbo.lisbon.messages.outgoing.moderation.CFH_ACK`.
use crate::game::moderation::cfh::call_for_help::CallForHelp;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct CFH_ACK {
    call: Option<CallForHelp>,
}

impl CFH_ACK {
    /// Mirrors the `CFH_ACK(CallForHelp)` constructor.
    pub fn new(call: Option<CallForHelp>) -> Self {
        Self { call }
    }
}

impl MessageComposer for CFH_ACK {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        // TODO: verify if structure and packet name is correct by looking
        // at the lingo (inherited from the Java).
        response.write_bool(self.call.is_some());

        if let Some(call) = &self.call {
            response.write_string(call.get_cry_id());
            response.write_string(call.get_formatted_request_time());
            response.write_string(call.get_message());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        319 // "D"
    }
}
