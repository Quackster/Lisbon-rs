//! Mirrors `net.h4bbo.lisbon.messages.outgoing.catalogue.CATALOGUE_PAGES`.
use crate::game::catalogue::catalogue_page::CataloguePage;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct CATALOGUE_PAGES {
    catalogue_pages: Vec<CataloguePage>,
}

impl CATALOGUE_PAGES {
    /// Mirrors the `CATALOGUE_PAGES(List<CataloguePage>)` constructor.
    pub fn new(catalogue_pages: Vec<CataloguePage>) -> Self {
        Self { catalogue_pages }
    }
}

impl MessageComposer for CATALOGUE_PAGES {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        for page in &self.catalogue_pages {
            response.write_delimeter(page.get_name_index(), '\t');
            response.write_delimeter(page.get_name(), '\r');
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        126 // "A~"
    }
}
