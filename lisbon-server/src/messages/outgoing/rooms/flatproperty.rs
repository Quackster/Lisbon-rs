//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.FLATPROPERTY`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct FLATPROPERTY {
    property: String,
    value: String,
}

impl FLATPROPERTY {
    /// Mirrors the `FLATPROPERTY(String, Object)` constructor.
    pub fn new<V: std::fmt::Display>(property: &str, value: V) -> Self {
        Self {
            property: property.to_string(),
            value: format!("{value}"),
        }
    }
}

impl MessageComposer for FLATPROPERTY {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write(self.property.as_str());
        response.write("/");
        response.write(self.value.as_str());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        46 // "@n"
    }
}
