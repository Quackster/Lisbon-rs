//! Mirrors `net.h4bbo.lisbon.messages.outgoing.catalogue.DELIVER_PRESENT`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct DELIVER_PRESENT {
    sprite: String,
    custom_data: String,
    colour: String,
}

impl DELIVER_PRESENT {
    /// Mirrors the `DELIVER_PRESENT(String, String, String)` constructor.
    pub fn new(sprite: &str, custom_data: &str, colour: &str) -> Self {
        Self {
            sprite: sprite.to_string(),
            custom_data: custom_data.to_string(),
            colour: colour.to_string(),
        }
    }
}

impl MessageComposer for DELIVER_PRESENT {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_delimeter(self.sprite.as_str(), '\r');

        if self.sprite.eq_ignore_ascii_case("poster") {
            response.write_delimeter(
                format!("{} {}", self.sprite, self.custom_data),
                '\r',
            );
        } else {
            response.write_delimeter(self.sprite.as_str(), '\r');
        }

        response.write(self.colour.as_str());
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        129 // "BA"
    }
}
