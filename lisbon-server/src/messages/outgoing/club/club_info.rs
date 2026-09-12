//! Mirrors `net.h4bbo.lisbon.messages.outgoing.club.CLUB_INFO`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct CLUB_INFO {
    remaining_days_this_month: i32,
    since_months: i32,
    prepaid_months: i32,
}

impl CLUB_INFO {
    /// Mirrors the `CLUB_INFO(int, int, int)` constructor.
    pub fn new(
        remaining_days_this_month: i32,
        since_months: i32,
        prepaid_months: i32,
    ) -> Self {
        Self {
            remaining_days_this_month,
            since_months,
            prepaid_months,
        }
    }
}

impl MessageComposer for CLUB_INFO {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_string("club_habbo");
        response.write_int(self.remaining_days_this_month);
        response.write_int(self.since_months);
        response.write_int(self.prepaid_months);
        response.write_int(1);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        7 // "@G"
    }
}
