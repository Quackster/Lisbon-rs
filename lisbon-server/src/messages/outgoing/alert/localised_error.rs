//! Mirrors `net.h4bbo.lisbon.messages.outgoing.alert.LOCALISED_ERROR`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct LOCALISED_ERROR {
    external_text_entry: String,
}

impl LOCALISED_ERROR {
    /// Mirrors the `LOCALISED_ERROR(String)` constructor.
    pub fn new(external_text_entry: &str) -> Self {
        Self {
            external_text_entry: external_text_entry.to_string(),
        }
    }
}

impl MessageComposer for LOCALISED_ERROR {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write(self.external_text_entry.as_str());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        33 // "@a"
    }
}
