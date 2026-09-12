//! Mirrors `net.h4bbo.lisbon.messages.outgoing.catalogue.CATALOGUE_PAGE`.
use crate::game::catalogue::catalogue_item::CatalogueItem;
use crate::game::catalogue::catalogue_page::CataloguePage;
use crate::game::item::base::item_behaviour::ItemBehaviour;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct CATALOGUE_PAGE {
    catalogue_items: Vec<CatalogueItem>,
    page: CataloguePage,
}

impl CATALOGUE_PAGE {
    /// Mirrors the `CATALOGUE_PAGE(CataloguePage, List<CatalogueItem>)`
    /// constructor.
    pub fn new(page: CataloguePage, catalogue_items: Vec<CatalogueItem>) -> Self {
        Self {
            page,
            catalogue_items,
        }
    }
}

impl MessageComposer for CATALOGUE_PAGE {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_key_value("i", self.page.get_name_index());
        response.write_key_value("n", self.page.get_name());
        response.write_key_value("l", self.page.get_layout());
        response.write_key_value("g", self.page.get_image_headline());

        let image_teasers = self.page.get_image_teasers();
        if !image_teasers.is_empty() {
            response.write_key_value("e", image_teasers);
        }

        let body = self.page.get_body();
        if !body.is_empty() {
            response.write_key_value("h", body);
        }

        let link_list = self.page.get_link_list();
        if !link_list.is_empty() {
            response.write_key_value("u", link_list);
        }

        let label_pick = self.page.get_label_pick();
        if !label_pick.is_empty() {
            response.write_key_value("w", label_pick);
        }

        let label_extra_s = self.page.get_label_extra_s();
        if !label_extra_s.is_empty() {
            response.write_key_value("s", label_extra_s);
        }

        let label_extra = self.page.get_label_extra();
        for label_data_id in 1..11 {
            let extra_data_id = format!("label_extra_t_{label_data_id}");

            if let Some(value) = label_extra.get(&extra_data_id) {
                response.write_key_value(format!("t{label_data_id}"), value);
            }
        }

        for item in &self.catalogue_items {
            response.write("p:");
            response.write_delimeter(item.get_name(), '\t');
            response.write_delimeter(item.get_description(), '\t');
            response.write_delimeter(item.get_price(), '\t');
            response.write_delimeter("", '\t');
            response.write_delimeter(item.get_type(), '\t');
            response.write_delimeter(item.get_icon(), '\t');
            response.write_delimeter(item.get_size(), '\t');
            response.write_delimeter(item.get_dimensions(), '\t');
            response.write_delimeter(item.get_sale_code(), '\t');

            let definition = item.get_definition();
            let is_poster = definition
                .as_ref()
                .is_some_and(|definition| definition.get_sprite() == "poster");

            if item.is_package() || is_poster {
                response.write_delimeter("", '\t');
            }

            if item.is_package() {
                let packages = item.get_packages();
                response.write_delimeter(packages.len() as i32, '\t');

                for catalogue_package in &packages {
                    let package_definition = catalogue_package.get_definition();
                    response.write_delimeter(
                        package_definition
                            .as_ref()
                            .map(|definition| definition.get_icon(catalogue_package.get_special_sprite_id()))
                            .unwrap_or_default(),
                        '\t',
                    );
                    response.write_delimeter(catalogue_package.get_amount(), '\t');
                    response.write_delimeter(
                        package_definition
                            .as_ref()
                            .map(|definition| definition.get_colour().to_string())
                            .unwrap_or_default(),
                        '\t',
                    );
                }
            } else if definition
                .as_ref()
                .is_some_and(|definition| !definition.has_behaviour(ItemBehaviour::WallItem))
            {
                let colour = definition
                    .as_ref()
                    .map(|definition| definition.get_colour().to_string())
                    .unwrap_or_default();
                response.write_delimeter(colour, '\t');
            }

            response.write('\r');
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        127 // "A"
    }
}
