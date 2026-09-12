//! Mirrors `net.h4bbo.lisbon.messages.outgoing.trade.TRADE_ITEMS`.
use crate::game::entity::entity::Entity;
use crate::game::inventory::inventory::Inventory;
use crate::game::item::item::Item;
use crate::game::player::player::Player;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[allow(non_camel_case_types)]
pub struct TRADE_ITEMS<'a> {
    player: &'a Player,
    own_items: Vec<Item>,
    player_accepted_trade: bool,
    trade_partner: &'a Player,
    partner_items: Vec<Item>,
    partner_accepted_trade: bool,
}

impl<'a> TRADE_ITEMS<'a> {
    /// Mirrors the 6-arg `TRADE_ITEMS` constructor.
    pub fn new(
        player: &'a Player,
        own_items: Vec<Item>,
        player_accepted_trade: bool,
        trade_partner: &'a Player,
        partner_items: Vec<Item>,
        partner_accepted_trade: bool,
    ) -> Self {
        Self {
            player,
            own_items,
            player_accepted_trade,
            trade_partner,
            partner_items,
            partner_accepted_trade,
        }
    }
}

impl MessageComposer for TRADE_ITEMS<'_> {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_delimeter(self.player.get_details().get_name(), '\u{0009}');
        response.write_delimeter(
            if self.player_accepted_trade { "true" } else { "false" },
            '\u{0009}',
        );

        for (i, item) in self.own_items.iter().enumerate() {
            Inventory::serialise(response, item, i as i32);
        }

        // The Java writes `Character.toString((char) 13)` (the raw 0x0D
        // byte).
        response.write("\u{000d}");

        response.write_delimeter(
            self.trade_partner.get_details().get_name(),
            '\u{0009}',
        );
        response.write_delimeter(
            if self.partner_accepted_trade { "true" } else { "false" },
            '\u{0009}',
        );

        for (i, item) in self.partner_items.iter().enumerate() {
            Inventory::serialise(response, item, i as i32);
        }
    }

    fn get_header(&self) -> i16 {
        108 // "Al"
    }
}
