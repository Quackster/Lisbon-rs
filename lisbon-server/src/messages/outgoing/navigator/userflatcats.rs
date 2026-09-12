//! Mirrors `net.h4bbo.lisbon.messages.outgoing.navigator.USERFLATCATS`.
use crate::game::navigator::navigator_category::NavigatorCategory;
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct USERFLATCATS {
    category_list: Vec<NavigatorCategory>,
}

impl USERFLATCATS {
    /// Mirrors the `USERFLATCATS(List<NavigatorCategory>)` constructor.
    pub fn new(category_list: Vec<NavigatorCategory>) -> Self {
        Self { category_list }
    }
}

impl MessageComposer for USERFLATCATS {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.category_list.len() as i32);

        for category in &self.category_list {
            response.write_int(category.get_id());
            response.write_string(category.get_name());
        }
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        221 // "C]"
    }
}
