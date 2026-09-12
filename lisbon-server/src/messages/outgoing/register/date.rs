//! Mirrors `net.h4bbo.lisbon.messages.outgoing.register.DATE`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct DATE {
    short_date: String,
}

impl DATE {
    /// Mirrors the `DATE(String)` constructor.
    pub fn new(short_date: &str) -> Self {
        Self {
            short_date: short_date.to_string(),
        }
    }
}

impl MessageComposer for DATE {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write(self.short_date.as_str());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        163 // "Bc"
    }
}
