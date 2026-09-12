//! Mirrors `net.h4bbo.lisbon.messages.outgoing.purse.VOUCHER_REDEEM_OK`.
use crate::game::catalogue::catalogue_item::CatalogueItem;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct VOUCHER_REDEEM_OK {
    redeemable_items: Vec<CatalogueItem>,
}

impl VOUCHER_REDEEM_OK {
    /// Mirrors the `VOUCHER_REDEEM_OK(List<CatalogueItem>)` constructor.
    pub fn new(redeemable_items: Vec<CatalogueItem>) -> Self {
        Self { redeemable_items }
    }
}

impl MessageComposer for VOUCHER_REDEEM_OK {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        if self.redeemable_items.is_empty() {
            return;
        }

        if self.redeemable_items.len() <= 2 {
            for catalogue_item in &self.redeemable_items {
                let definition = catalogue_item.get_definition();

                response.write_string(
                    definition
                        .as_ref()
                        .map(|definition| definition.get_name())
                        .unwrap_or(""),
                );
                response.write_string(
                    definition
                        .as_ref()
                        .map(|definition| definition.get_description())
                        .unwrap_or(""),
                );
            }
        } else {
            let names: Vec<String> = self
                .redeemable_items
                .iter()
                .map(|item| {
                    item.get_definition()
                        .map(|definition| definition.get_name().to_string())
                        .unwrap_or_default()
                })
                .collect();

            response.write_string(names.join(", "));
            response.write_string("");
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        212 // "CT"
    }
}
