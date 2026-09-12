//! Mirrors `net.h4bbo.lisbon.messages.outgoing.rooms.items.IDATA`.
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::game::item::item::Item;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct IDATA {
    colour: Option<String>,
    text: Option<String>,
    item: Item,
}

impl IDATA {
    /// Mirrors the `IDATA(Item, String, String)` constructor.
    pub fn new(item: Item, colour: &str, text: &str) -> Self {
        Self {
            colour: Some(colour.to_string()),
            text: Some(text.to_string()),
            item,
        }
    }

    /// Mirrors the `IDATA(Item)` constructor.
    pub fn item_only(item: Item) -> Self {
        Self {
            colour: None,
            text: None,
            item,
        }
    }
}

impl MessageComposer for IDATA {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        let is_photo = self
            .item
            .get_definition()
            .get_sprite()
            .eq_ignore_ascii_case("photo");

        if self.item.has_behaviour(ItemBehaviour::PostIt) {
            response.write_delimeter(self.item.get_id(), '\u{0009}');
            response.write_delimeter(self.colour.as_deref().unwrap_or(""), ' ');
            response.write(self.text.as_deref().unwrap_or(""));
        } else {
            response.write_delimeter(self.item.get_id(), '\u{0009}');
            if is_photo {
                response.write_delimeter("I", ' ');
            } else {
                response.write_delimeter(
                    if self.item.has_behaviour(ItemBehaviour::WallItem) {
                        "I"
                    } else {
                        "S"
                    },
                    ' ',
                );
                response.write_delimeter(self.item.get_owner_id(), ' ');
            }
            response.write(self.item.get_custom_data());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        48 // "@p"
    }
}
