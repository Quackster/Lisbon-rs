//! Mirrors `net.h4bbo.lisbon.messages.outgoing.openinghours.INFO_HOTEL_CLOSED`.
use crate::messages::types::MessageComposer;
use crate::server::netty::streams::NettyResponse;

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct INFO_HOTEL_CLOSED {
    open_time_hour: i32,
    open_time_minute: i32,
    disconnect: bool,
}

impl INFO_HOTEL_CLOSED {
    /// Mirrors the `INFO_HOTEL_CLOSED(LocalTime, boolean)` constructor (the
    /// Java `LocalTime` is decomposed into its hour and minute fields).
    pub fn new(open_time_hour: i32, open_time_minute: i32, disconnect: bool) -> Self {
        Self {
            open_time_hour,
            open_time_minute,
            disconnect,
        }
    }
}

impl MessageComposer for INFO_HOTEL_CLOSED {
    /// Mirrors `compose(NettyResponse)`.
    fn compose(&self, response: &mut NettyResponse) {
        response.write_int(self.open_time_hour);
        response.write_int(self.open_time_minute);
        response.write_bool(self.disconnect);
    }

    /// Mirrors `getHeader()`.
    fn get_header(&self) -> i16 {
        292 // "Dd"
    }
}
